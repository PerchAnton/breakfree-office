use crate::{
    Result,
    contract::{APP_IDS, Manifest},
    process,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env, fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

const SETTINGS_LIMIT: u64 = 65_536;
const RECENT_LIMIT: usize = 30;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Recent {
    pub path: PathBuf,
    pub app_id: String,
    pub opened_at: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    pub settings_version: u32,
    pub trusted_executables: BTreeMap<String, PathBuf>,
    #[serde(default)]
    pub recent: Vec<Recent>,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            settings_version: 1,
            trusted_executables: BTreeMap::new(),
            recent: Vec::new(),
        }
    }
}

pub fn settings_path() -> PathBuf {
    let home = env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" })
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    if cfg!(windows) {
        env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join("AppData/Local"))
            .join("BreakFree/Office/settings.json")
    } else if cfg!(target_os = "macos") {
        home.join("Library/Application Support/BreakFree Office/settings.json")
    } else {
        env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .filter(|p| p.is_absolute())
            .unwrap_or_else(|| home.join(".config"))
            .join("breakfree-office/settings.json")
    }
}

fn load(path: &Path) -> Result<Settings> {
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Settings::default());
        }
        Err(_) => return Err("Could not read settings. The original file was preserved.".into()),
    };
    let mut bytes = Vec::new();
    file.take(SETTINGS_LIMIT + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Could not read settings.")?;
    if bytes.len() as u64 > SETTINGS_LIMIT {
        return Err("Settings exceed the size limit; original file preserved.".into());
    }
    let value = crate::contract::strict_json(&bytes)?;
    let settings: Settings = serde_json::from_value(value)
        .map_err(|_| "Invalid settings; repair the original file and restart Office.")?;
    if settings.settings_version != 1
        || settings.recent.len() > RECENT_LIMIT
        || settings
            .trusted_executables
            .iter()
            .any(|(id, path)| !APP_IDS.contains(&id.as_str()) || !path.is_absolute())
        || settings
            .recent
            .iter()
            .any(|r| !APP_IDS.contains(&r.app_id.as_str()) || !r.path.is_absolute())
    {
        return Err(
            "Invalid settings version, application ID or path; original file preserved.".into(),
        );
    }
    Ok(settings)
}

fn save(path: &Path, settings: &Settings) -> Result<()> {
    let bytes =
        serde_json::to_vec_pretty(settings).map_err(|_| "Cannot serialize settings paths.")?;
    if bytes.len() as u64 > SETTINGS_LIMIT {
        return Err("Settings exceed the size limit.".into());
    }
    let parent = path
        .parent()
        .ok_or("Settings require a parent directory.")?;
    fs::create_dir_all(parent).map_err(|_| "Cannot create settings directory.")?;
    let mut file = tempfile::NamedTempFile::new_in(parent)
        .map_err(|_| "Cannot prepare atomic settings save.")?;
    file.write_all(&bytes)
        .and_then(|_| file.write_all(b"\n"))
        .and_then(|_| file.as_file().sync_all())
        .map_err(|_| "Could not write settings; original preserved.")?;
    file.persist(path)
        .map_err(|_| "Could not replace settings; original preserved.")?;
    Ok(())
}

#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub path: Option<PathBuf>,
    pub manifest: Option<Manifest>,
    pub error: Option<String>,
}

pub struct Registry {
    pub file: PathBuf,
    pub settings: Settings,
    pub apps: [AppState; 3],
}

impl Registry {
    pub fn new(file: PathBuf) -> Result<Self> {
        let settings = load(&file)?;
        let apps = std::array::from_fn(|i| AppState {
            path: settings.trusted_executables.get(APP_IDS[i]).cloned(),
            ..AppState::default()
        });
        Ok(Self {
            file,
            settings,
            apps,
        })
    }

    fn commit(&mut self, settings: Settings) -> Result<()> {
        save(&self.file, &settings)?;
        self.settings = settings;
        Ok(())
    }

    pub fn configure(&mut self, index: usize, path: &Path) -> Result<()> {
        let id = *APP_IDS.get(index).ok_or("Unknown application.")?;
        let path = process::executable(path)?;
        let manifest = process::probe(&path, id)?;
        let mut updated = self.settings.clone();
        updated.trusted_executables.insert(id.into(), path.clone());
        self.commit(updated)?;
        self.apps[index] = AppState {
            path: Some(path),
            manifest: Some(manifest),
            error: None,
        };
        Ok(())
    }

    pub fn forget(&mut self, index: usize) -> Result<()> {
        let id = *APP_IDS.get(index).ok_or("Unknown application.")?;
        let mut updated = self.settings.clone();
        updated.trusted_executables.remove(id);
        self.commit(updated)?;
        self.apps[index] = AppState::default();
        Ok(())
    }

    pub fn inspect(&mut self, index: usize) {
        let Some(path) = self
            .settings
            .trusted_executables
            .get(APP_IDS[index])
            .cloned()
        else {
            self.apps[index] = AppState::default();
            return;
        };
        self.apps[index] = match process::probe(&path, APP_IDS[index]) {
            Ok(manifest) => AppState {
                path: Some(path),
                manifest: Some(manifest),
                error: None,
            },
            Err(error) => AppState {
                path: Some(path),
                manifest: None,
                error: Some(error),
            },
        };
    }

    pub fn refresh(&mut self) {
        for i in 0..APP_IDS.len() {
            self.inspect(i);
        }
    }

    pub fn candidates(&mut self, file: &Path) -> Result<Vec<usize>> {
        let file = process::document(file)?;
        self.refresh();
        Ok(self
            .apps
            .iter()
            .enumerate()
            .filter_map(|(i, state)| {
                state
                    .manifest
                    .as_ref()
                    .filter(|m| m.can_open(&file))
                    .map(|_| i)
            })
            .collect())
    }

    pub fn launch(&mut self, index: usize, action: &str, file: Option<&Path>) -> Result<String> {
        let id = *APP_IDS.get(index).ok_or("Unknown application.")?;
        self.inspect(index);
        let state = &self.apps[index];
        let manifest = state.manifest.as_ref().ok_or_else(|| {
            state.error.clone().unwrap_or(
                "Application is not installed. Choose its trusted executable in Settings.".into(),
            )
        })?;
        let path = state.path.as_ref().ok_or("Application is not installed.")?;
        process::dispatch(path, manifest, action, file)?;
        if let Some(file) = file {
            let file = process::document(file)?;
            let mut settings = self.settings.clone();
            settings.recent.retain(|r| r.path != file);
            settings.recent.insert(
                0,
                Recent {
                    path: file,
                    app_id: id.into(),
                    opened_at: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                },
            );
            settings.recent.truncate(RECENT_LIMIT);
            if let Err(error) = self.commit(settings) {
                return Ok(format!(
                    "Request sent. Recent-file history could not be saved: {error}"
                ));
            }
        }
        Ok(format!(
            "Request sent to {}. File loading is not acknowledged by protocol v1.",
            crate::contract::APP_NAMES[index]
        ))
    }

    pub fn clear_recent(&mut self, path: Option<&Path>) -> Result<()> {
        let mut updated = self.settings.clone();
        if let Some(path) = path {
            updated.recent.retain(|r| r.path != path);
        } else {
            updated.recent.clear();
        }
        self.commit(updated)
    }
}
