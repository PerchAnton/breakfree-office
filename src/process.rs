use crate::{
    Result,
    contract::{self, Manifest},
};
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};

pub const PROBE_TIMEOUT: Duration = Duration::from_secs(3);
pub const STDOUT_LIMIT: usize = 65_536;
pub const STDERR_LIMIT: usize = 16_384;

pub fn executable(path: &Path) -> Result<PathBuf> {
    if !path.is_absolute() {
        return Err("Choose an absolute path to a trusted executable.".into());
    }
    let path = fs::canonicalize(path)
        .map_err(|_| "Application is not installed at the selected location.")?;
    if !path.is_file() {
        return Err("Choose an executable file, not an app bundle or directory.".into());
    }
    #[cfg(windows)]
    if path
        .extension()
        .is_none_or(|ext| !ext.eq_ignore_ascii_case("exe"))
    {
        return Err("Choose an .exe file; batch scripts are not accepted.".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if path
            .metadata()
            .map_err(|_| "Cannot read executable permissions.")?
            .permissions()
            .mode()
            & 0o111
            == 0
        {
            return Err("The selected file is not executable.".into());
        }
    }
    Ok(path)
}

fn command(path: &Path) -> Command {
    let mut command = Command::new(path);
    command.stdin(Stdio::null());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0200); // CREATE_NO_WINDOW | CREATE_NEW_PROCESS_GROUP
    }
    command
}

fn stop(child: &mut Child) {
    #[cfg(unix)]
    {
        // SAFETY: the child owns the isolated process group created immediately above.
        // A negative PID addresses that group, never the launcher's group.
        unsafe {
            libc::kill(-(child.id() as i32), libc::SIGKILL);
        }
    }
    let _ = child.kill();
    let _ = child.wait();
}

fn collect<R: Read + Send + 'static>(
    mut stream: R,
    limit: usize,
    cancelled: Arc<AtomicBool>,
    tx: mpsc::Sender<Result<Vec<u8>>>,
) {
    thread::spawn(move || {
        let mut data = Vec::new();
        let mut chunk = [0; 4096];
        let result = loop {
            if cancelled.load(Ordering::Relaxed) {
                break Err("Probe cancelled.".into());
            }
            match stream.read(&mut chunk) {
                Ok(0) => break Ok(data),
                Ok(n) if data.len() + n > limit => {
                    break Err("Capability output exceeded the allowed size.".into());
                }
                Ok(n) => data.extend_from_slice(&chunk[..n]),
                Err(_) => break Err("Could not read capability output.".into()),
            }
        };
        let _ = tx.send(result);
    });
}

pub fn probe(path: &Path, id: &str) -> Result<Manifest> {
    probe_with_timeout(path, id, PROBE_TIMEOUT)
}

pub fn probe_with_timeout(path: &Path, id: &str, timeout: Duration) -> Result<Manifest> {
    let path = executable(path)?;
    let mut child = command(&path)
        .arg("--capabilities")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| "Could not start the selected executable.")?;
    let cancelled = Arc::new(AtomicBool::new(false));
    let (out_tx, out_rx) = mpsc::channel();
    let (err_tx, err_rx) = mpsc::channel();
    collect(
        child.stdout.take().ok_or("Missing stdout pipe.")?,
        STDOUT_LIMIT,
        cancelled.clone(),
        out_tx,
    );
    collect(
        child.stderr.take().ok_or("Missing stderr pipe.")?,
        STDERR_LIMIT,
        cancelled.clone(),
        err_tx,
    );
    let deadline = Instant::now() + timeout;
    let (mut stdout, mut stderr) = (None, None);
    let result = (|| {
        loop {
            if stdout.is_none()
                && let Ok(output) = out_rx.try_recv()
            {
                stdout = Some(output?);
            }
            if stderr.is_none()
                && let Ok(output) = err_rx.try_recv()
            {
                stderr = Some(output?);
            }
            if let Some(status) = child
                .try_wait()
                .map_err(|_| "Could not inspect probe process.")?
                && stdout.is_some()
                && stderr.is_some()
            {
                if !status.success() {
                    return Err(format!("Capability request failed: {status}."));
                }
                return contract::validate(stdout.as_deref().unwrap_or_default(), Some(id));
            }
            if Instant::now() >= deadline {
                return Err("Capability request timed out.".into());
            }
            thread::sleep(Duration::from_millis(8));
        }
    })();
    cancelled.store(true, Ordering::Relaxed);
    if result.is_err() {
        stop(&mut child);
    }
    result
}

pub fn document(path: &Path) -> Result<PathBuf> {
    let path = fs::canonicalize(path).map_err(|_| "The document is missing or inaccessible.")?;
    if !path.is_file() {
        return Err("Choose an existing local document file.".into());
    }
    Ok(path)
}

pub fn dispatch(
    path: &Path,
    manifest: &Manifest,
    action: &str,
    file: Option<&Path>,
) -> Result<u32> {
    if !["new", "open"].contains(&action) || !manifest.supports(action) {
        return Err("Application does not support this action.".into());
    }
    let path = executable(path)?;
    let mut command = command(&path);
    command.arg(format!("--{action}"));
    match (action, file) {
        ("open", Some(file)) => {
            let file = document(file)?;
            if !manifest.can_open(&file) {
                return Err("Application does not advertise reading this file type.".into());
            }
            command.arg(file);
        }
        ("new", None) => {}
        _ => return Err("Invalid document request.".into()),
    }
    let mut child = command
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| "Could not launch the editor.")?;
    let pid = child.id();
    let deadline = Instant::now() + Duration::from_millis(150);
    while Instant::now() < deadline {
        if let Some(status) = child
            .try_wait()
            .map_err(|_| "Cannot inspect editor process.")?
        {
            return if status.success() {
                Ok(pid)
            } else {
                Err(format!("Editor exited immediately: {status}."))
            };
        }
        thread::sleep(Duration::from_millis(8));
    }
    thread::spawn(move || {
        let _ = child.wait();
    });
    Ok(pid)
}
