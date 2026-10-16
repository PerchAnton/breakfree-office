use crate::{
    contract::APP_NAMES,
    registry::{AppState, Recent, Registry},
    visual,
};
use eframe::egui::{self, Align, Color32, Layout, RichText, Sense, Vec2};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex, mpsc},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    Home,
    New,
    Open,
    Recent,
    Settings,
    About,
}
impl Page {
    pub const ALL: [Self; 6] = [
        Self::Home,
        Self::New,
        Self::Open,
        Self::Recent,
        Self::Settings,
        Self::About,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::New => "New",
            Self::Open => "Open",
            Self::Recent => "Recent",
            Self::Settings => "Settings",
            Self::About => "About",
        }
    }
}

enum Job {
    Refresh,
    Configure(usize, PathBuf),
    Forget(usize),
    New(usize),
    Open(PathBuf, Option<usize>),
    RemoveRecent(Option<PathBuf>),
}
enum Outcome {
    Done(String),
    Choose(PathBuf, Vec<usize>),
    Error(String),
}
struct Update {
    apps: [AppState; 3],
    recent: Vec<Recent>,
    outcome: Outcome,
}

pub struct OfficeApp {
    pub page: Page,
    registry: Option<Arc<Mutex<Registry>>>,
    apps: [AppState; 3],
    recent: Vec<Recent>,
    receiver: Option<mpsc::Receiver<Update>>,
    notice: String,
    error: bool,
    choice: Option<(PathBuf, Vec<usize>)>,
    logo: Option<egui::TextureHandle>,
    pub font_name: String,
    zoom: f32,
    initial_refresh: bool,
    #[cfg(feature = "validation-capture")]
    capture_frames: usize,
}

impl OfficeApp {
    pub fn new(ctx: &egui::Context, file: PathBuf) -> Self {
        let font_name = visual::apply(ctx);
        #[cfg(feature = "validation-capture")]
        let initial_zoom = std::env::var("BREAKFREE_CAPTURE_ZOOM")
            .ok()
            .and_then(|s| s.parse::<f32>().ok())
            .filter(|s| [1.0, 1.25, 1.5].contains(s))
            .unwrap_or(ctx.zoom_factor());
        #[cfg(not(feature = "validation-capture"))]
        let initial_zoom = ctx.zoom_factor();
        if initial_zoom != ctx.zoom_factor() {
            ctx.set_zoom_factor(initial_zoom);
        }
        let (registry, apps, recent, notice, error) = match Registry::new(file) {
            Ok(registry) => {
                let apps = registry.apps.clone();
                let recent = registry.settings.recent.clone();
                (
                    Some(Arc::new(Mutex::new(registry))),
                    apps,
                    recent,
                    "Ready".into(),
                    false,
                )
            }
            Err(error) => (None, Default::default(), Vec::new(), error, true),
        };
        let initial_refresh = apps.iter().any(|app| app.path.is_some());
        let logo = image::load_from_memory(include_bytes!("../docs/brand/breakfree-icon.webp"))
            .ok()
            .map(|im| {
                let rgba = im.to_rgba8();
                ctx.load_texture(
                    "approved-breakfree-mark",
                    egui::ColorImage::from_rgba_unmultiplied(
                        [rgba.width() as usize, rgba.height() as usize],
                        rgba.as_raw(),
                    ),
                    egui::TextureOptions::LINEAR,
                )
            });
        Self {
            page: Page::Home,
            registry,
            apps,
            recent,
            receiver: None,
            notice,
            error,
            choice: None,
            logo,
            font_name,
            zoom: initial_zoom,
            initial_refresh,
            #[cfg(feature = "validation-capture")]
            capture_frames: 0,
        }
    }

    fn busy(&self) -> bool {
        self.receiver.is_some()
    }
    fn start(&mut self, job: Job, ctx: &egui::Context) {
        if self.busy() {
            return;
        }
        let Some(registry) = self.registry.clone() else {
            return;
        };
        let (tx, rx) = mpsc::channel();
        self.receiver = Some(rx);
        self.notice = "Working…".into();
        self.error = false;
        let ctx = ctx.clone();
        thread::spawn(move || {
            let Ok(mut registry) = registry.lock() else {
                return;
            };
            let outcome = match job {
                Job::Refresh => { registry.refresh(); Ok(Outcome::Done("Application check complete.".into())) },
                Job::Configure(i, path) => registry.configure(i, &path).map(|_| Outcome::Done(format!("{} is ready.", APP_NAMES[i]))),
                Job::Forget(i) => registry.forget(i).map(|_| Outcome::Done("Location forgotten. The application was not deleted.".into())),
                Job::New(i) => registry.launch(i, "new", None).map(Outcome::Done),
                Job::Open(file, Some(i)) => registry.launch(i, "open", Some(&file)).map(Outcome::Done),
                Job::Open(file, None) => registry.candidates(&file).and_then(|ids| match ids.as_slice() {
                    [] => Err("No installed editor advertises reading this file type. Configure a compatible editor in Settings.".into()),
                    [i] => registry.launch(*i, "open", Some(&file)).map(Outcome::Done),
                    _ => Ok(Outcome::Choose(file, ids)),
                }),
                Job::RemoveRecent(path) => registry.clear_recent(path.as_deref()).map(|_| Outcome::Done("Recent-file list updated. No file was deleted.".into())),
            }.unwrap_or_else(Outcome::Error);
            let _ = tx.send(Update {
                apps: registry.apps.clone(),
                recent: registry.settings.recent.clone(),
                outcome,
            });
            ctx.request_repaint();
        });
    }

    fn poll(&mut self) {
        if let Some(receiver) = &self.receiver {
            match receiver.try_recv() {
                Ok(update) => {
                    self.apps = update.apps;
                    self.recent = update.recent;
                    match update.outcome {
                        Outcome::Done(message) => {
                            self.notice = message;
                            self.error = false;
                        }
                        Outcome::Error(message) => {
                            self.notice = message;
                            self.error = true;
                        }
                        Outcome::Choose(path, ids) => {
                            self.choice = Some((path, ids));
                            self.notice = "Choose an editor.".into();
                        }
                    }
                    self.receiver = None;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.receiver = None;
                    self.notice = "Background operation ended unexpectedly.".into();
                    self.error = true;
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
    }

    fn open_picker(&mut self, ctx: &egui::Context) {
        if let Some(file) = rfd::FileDialog::new()
            .set_title("Open a local document")
            .pick_file()
        {
            self.start(Job::Open(file, None), ctx);
        }
    }

    fn menu(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu")
            .exact_height(30.0)
            .frame(
                egui::Frame::new()
                    .fill(visual::NAV)
                    .inner_margin(egui::Margin::symmetric(12, 0)),
            )
            .show(ctx, |ui| {
                ui.visuals_mut().button_frame = false;
                ui.horizontal(|ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Home").clicked() {
                            self.page = Page::Home;
                            ui.close();
                        }
                        if ui.button("New\tCtrl+N").clicked() {
                            self.page = Page::New;
                            ui.close();
                        }
                        if ui
                            .add_enabled(
                                !self.busy() && self.registry.is_some(),
                                egui::Button::new("Open…\tCtrl+O"),
                            )
                            .clicked()
                        {
                            ui.close();
                            self.open_picker(ctx);
                        }
                        if ui.button("Recent files").clicked() {
                            self.page = Page::Recent;
                            ui.close();
                        }
                        ui.separator();
                        if ui.button("Settings").clicked() {
                            self.page = Page::Settings;
                            ui.close();
                        }
                        if ui.button("Exit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.menu_button("Help", |ui| {
                        if ui.button("About BreakFree Office").clicked() {
                            self.page = Page::About;
                            ui.close();
                        }
                    });
                });
            });
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        #[cfg(feature = "validation-capture")]
        if let Ok(path) = std::env::var("BREAKFREE_CAPTURE_TO") {
            self.capture_frames += 1;
            ctx.request_repaint();
            if self.capture_frames == 8 {
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
            }
            let image = ctx.input(|i| {
                i.events.iter().find_map(|e| {
                    if let egui::Event::Screenshot { image, .. } = e {
                        Some(image.clone())
                    } else {
                        None
                    }
                })
            });
            if let Some(image) = image {
                let bytes: Vec<u8> = image.pixels.iter().flat_map(|p| p.to_array()).collect();
                if let Err(error) = image::save_buffer(
                    &path,
                    &bytes,
                    image.size[0] as u32,
                    image.size[1] as u32,
                    image::ColorType::Rgba8,
                ) {
                    eprintln!("Could not save validation capture: {error}");
                }
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
        self.poll();
        if self.initial_refresh {
            self.initial_refresh = false;
            self.start(Job::Refresh, ctx);
        }
        if self.busy() {
            ctx.request_repaint_after(Duration::from_millis(50));
        }
        if ctx.input_mut(|input| input.consume_key(egui::Modifiers::COMMAND, egui::Key::N)) {
            self.page = Page::New;
        }
        if ctx.input_mut(|input| input.consume_key(egui::Modifiers::COMMAND, egui::Key::O))
            && !self.busy()
            && self.registry.is_some()
        {
            self.open_picker(ctx);
        }
        if ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            self.choice = None;
        }
        for (i, key) in [
            egui::Key::Num1,
            egui::Key::Num2,
            egui::Key::Num3,
            egui::Key::Num4,
            egui::Key::Num5,
            egui::Key::Num6,
        ]
        .iter()
        .enumerate()
        {
            if ctx.input_mut(|input| input.consume_key(egui::Modifiers::ALT, *key)) {
                self.page = Page::ALL[i];
            }
        }
        self.menu(ctx);
        egui::TopBottomPanel::bottom("status")
            .exact_height(26.0)
            .frame(
                egui::Frame::new()
                    .fill(visual::NAV)
                    .inner_margin(egui::Margin::symmetric(12, 4)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if self.busy() {
                        ui.spinner();
                    }
                    let ready = self.apps.iter().filter(|s| s.manifest.is_some()).count();
                    ui.small(if ready == 0 {
                        "No editors installed".into()
                    } else {
                        format!("{ready} of 3 editors ready")
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.small("Light theme · English");
                    });
                });
            });
        egui::SidePanel::left("navigation")
            .exact_width(visual::NAV_WIDTH)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(visual::NAV)
                    .inner_margin(egui::Margin::symmetric(14, 20)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if let Some(logo) = &self.logo {
                        ui.image((logo.id(), Vec2::splat(30.0)));
                    }
                    ui.label(RichText::new("BreakFree\nOffice").size(15.0).strong());
                });
                ui.add_space(26.0);
                for page in Page::ALL {
                    let selected = self.page == page;
                    let response = ui.add_sized(
                        [ui.available_width(), visual::ROW_HEIGHT],
                        egui::Button::new(page.label())
                            .selected(selected)
                            .frame(selected),
                    );
                    focus(ui, &response);
                    if response.clicked() {
                        self.page = page;
                    }
                    response.on_hover_text(format!(
                        "{} · Alt+{}",
                        page.label(),
                        Page::ALL.iter().position(|p| *p == page).unwrap_or(0) + 1
                    ));
                    ui.add_space(3.0);
                }
            });
        egui::CentralPanel::default().frame(egui::Frame::new().fill(visual::BG).inner_margin(visual::CONTENT_MARGIN)).show(ctx, |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.heading(self.page.label());
                ui.add_space(22.0);
                if self.error {
                    egui::Frame::new().fill(Color32::from_rgb(255, 244, 231)).inner_margin(12).show(ui, |ui| {
                        ui.label(&self.notice);
                        if ui.button("Dismiss message").clicked() { self.error = false; }
                    });
                    ui.add_space(16.0);
                }
                match self.page {
                    Page::Home => { self.app_cards(ui, ctx); ui.add_space(26.0); ui.label(RichText::new("Recent").size(18.0)); ui.add_space(12.0); self.recent_files(ui, ctx, true); },
                    Page::New => { self.app_cards(ui, ctx); ui.add_space(28.0); ui.label(RichText::new("Templates").size(18.0)); ui.label("No templates are installed."); },
                    Page::Open => {
                        ui.label("Open a file from this computer."); ui.add_space(14.0);
                        if button(ui, "Browse this computer…", !self.busy() && self.registry.is_some()).clicked() { self.open_picker(ctx); }
                        ui.add_space(28.0); self.recent_files(ui, ctx, false);
                    },
                    Page::Recent => self.recent_files(ui, ctx, false),
                    Page::Settings => self.settings(ui, ctx),
                    Page::About => {
                        ui.label(RichText::new("BreakFree Office").size(20.0).strong());
                        ui.label("Your work. Your freedom."); ui.add_space(14.0);
                        ui.label(format!("Version {} · Rust desktop prototype", env!("CARGO_PKG_VERSION")));
                        ui.label(format!("Visual system {} · integration protocol 1", visual::VERSION));
                        ui.label(format!("Interface font: {}", self.font_name));
                        ui.label("Independent editors for documents, spreadsheets and presentations.");
                        ui.add_space(18.0);
                        ui.label("Office launches editors; it does not edit your documents. No accounts or online services are required.");
                        ui.label("Microsoft Office file compatibility and Windows visual acceptance have not been verified. No project software license has been selected.");
                    },
                }
                if !self.error && self.notice != "Ready" { ui.add_space(20.0); ui.label(RichText::new(&self.notice).size(12.0).color(visual::MUTED)); }
            });
        });
        if let Some((path, ids)) = self.choice.clone() {
            egui::Window::new("Choose an editor")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(format!(
                        "Open {} with:",
                        path.file_name().unwrap_or_default().to_string_lossy()
                    ));
                    for i in ids {
                        if button(ui, APP_NAMES[i], !self.busy()).clicked() {
                            self.choice = None;
                            self.start(Job::Open(path.clone(), Some(i)), ctx);
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.choice = None;
                    }
                });
        }
    }

    fn app_cards(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.label(RichText::new("Create new").size(18.0));
        ui.add_space(12.0);
        let enabled = !self.busy() && self.registry.is_some();
        let width = ((ui.available_width() - 24.0) / 3.0).max(160.0);
        ui.horizontal_wrapped(|ui| {
            for i in 0..3 {
                let ready = self.apps[i].manifest.is_some();
                let can_new = self.apps[i]
                    .manifest
                    .as_ref()
                    .is_some_and(|m| m.supports("new"));
                egui::Frame::new()
                    .fill(Color32::WHITE)
                    .stroke(egui::Stroke::new(1.0, visual::LINE))
                    .corner_radius(3)
                    .inner_margin(16)
                    .show(ui, |ui| {
                        ui.with_layout(Layout::top_down(Align::Min), |ui| {
                            ui.set_width(width - 32.0);
                            ui.set_min_height(122.0);
                            ui.label(RichText::new(APP_NAMES[i]).size(15.0).strong());
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new(if ready {
                                    "Ready"
                                } else if self.apps[i].path.is_none() {
                                    "Not installed"
                                } else {
                                    "Unavailable"
                                })
                                .size(12.0)
                                .color(visual::MUTED),
                            );
                            ui.add_space(12.0);
                            if button(
                                ui,
                                ["Blank document", "Blank spreadsheet", "Blank presentation"][i],
                                enabled && can_new,
                            )
                            .clicked()
                            {
                                self.start(Job::New(i), ctx);
                            }
                            if !ready && ui.small_button("Set up editor").clicked() {
                                self.page = Page::Settings;
                            }
                        });
                    });
            }
        });
    }

    fn recent_files(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, compact: bool) {
        if self.recent.is_empty() {
            ui.label(RichText::new("No recent files").color(visual::MUTED));
            ui.label("Files appear here after you open them with an editor.");
            return;
        }
        let recent = self.recent.clone();
        for entry in recent.iter().take(if compact { 6 } else { 30 }) {
            ui.separator();
            let name = entry.path.file_name().unwrap_or_default().to_string_lossy();
            let row = ui.add_enabled(
                !self.busy() && self.registry.is_some(),
                egui::Button::new(name.as_ref())
                    .frame(false)
                    .sense(Sense::click()),
            );
            focus(ui, &row);
            row.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Button,
                    true,
                    format!("Open {}", entry.path.display()),
                )
            });
            if row.clicked() {
                self.start(Job::Open(entry.path.clone(), None), ctx);
            }
            row.on_hover_text(entry.path.display().to_string())
                .context_menu(|ui| {
                    if ui.button("Remove from recent list").clicked() {
                        self.start(Job::RemoveRecent(Some(entry.path.clone())), ctx);
                        ui.close();
                    }
                });
            ui.horizontal(|ui| {
                ui.add(
                    egui::Label::new(
                        RichText::new(entry.path.display().to_string())
                            .small()
                            .color(visual::MUTED),
                    )
                    .truncate(),
                );
                ui.label(
                    RichText::new(age(entry.opened_at))
                        .small()
                        .color(visual::MUTED),
                );
            });
            ui.add_space(6.0);
        }
        ui.separator();
        if compact {
            if ui.button("Show all recent files").clicked() {
                self.page = Page::Recent;
            }
        } else if button(ui, "Clear recent list", !self.busy()).clicked() {
            self.start(Job::RemoveRecent(None), ctx);
        }
    }

    fn settings(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.label(RichText::new("Installed editors").size(18.0));
        ui.label("Only select executables you trust. Selection runs the program to check its capabilities.");
        ui.add_space(18.0);
        let enabled = !self.busy() && self.registry.is_some();
        for (i, name) in APP_NAMES.iter().enumerate() {
            ui.label(RichText::new(*name).strong());
            let state = &self.apps[i];
            if let Some(manifest) = &state.manifest {
                ui.label(format!("Ready · {}", manifest.app_version));
            } else {
                ui.label(
                    RichText::new(state.error.as_deref().unwrap_or("Not installed"))
                        .color(visual::MUTED),
                );
            }
            if let Some(path) = &state.path {
                ui.add(egui::Label::new(path.display().to_string()).wrap());
            }
            let has_path = state.path.is_some();
            ui.horizontal(|ui| {
                if button(
                    ui,
                    &format!(
                        "Choose {} executable…",
                        name.trim_start_matches("BreakFree ")
                    ),
                    enabled,
                )
                .clicked()
                {
                    let dialog = rfd::FileDialog::new()
                        .set_title(format!("Choose a trusted {name} executable"));
                    #[cfg(windows)]
                    let dialog = dialog.add_filter("Executable", &["exe"]);
                    if let Some(path) = dialog.pick_file() {
                        self.start(Job::Configure(i, path), ctx);
                    }
                }
                ui.push_id(i, |ui| {
                    if button(ui, "Forget location", enabled && has_path).clicked() {
                        self.start(Job::Forget(i), ctx);
                    }
                });
            });
            ui.add_space(14.0);
            ui.separator();
            ui.add_space(10.0);
        }
        if button(ui, "Check installed editors", enabled).clicked() {
            self.start(Job::Refresh, ctx);
        }
        ui.add_space(22.0);
        ui.horizontal(|ui| {
            ui.label("Interface zoom");
            let old = self.zoom;
            egui::ComboBox::from_id_salt("interface-zoom")
                .selected_text(format!("{}%", (self.zoom * 100.0) as u32))
                .show_ui(ui, |ui| {
                    for scale in [1.0, 1.25, 1.5] {
                        ui.selectable_value(
                            &mut self.zoom,
                            scale,
                            format!("{}%", (scale * 100.0) as u32),
                        );
                    }
                });
            if self.zoom != old {
                ctx.set_zoom_factor(self.zoom);
            }
        });
        ui.label(
            RichText::new(
                "Zoom applies to this session. Windows display scaling is controlled by Windows.",
            )
            .small()
            .color(visual::MUTED),
        );
    }
}

impl eframe::App for OfficeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render(ctx);
    }
}

fn button(ui: &mut egui::Ui, label: &str, enabled: bool) -> egui::Response {
    let response = ui.add_enabled(enabled, egui::Button::new(label));
    focus(ui, &response);
    response
}
fn focus(ui: &egui::Ui, response: &egui::Response) {
    if response.has_focus() {
        ui.painter().rect_stroke(
            response.rect.expand(2.0),
            2,
            egui::Stroke::new(1.5, visual::ACCENT),
            egui::StrokeKind::Outside,
        );
    }
}
fn age(timestamp: u64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let minutes = now.saturating_sub(timestamp) / 60;
    match minutes {
        0 => "Just now".into(),
        1..60 => format!("{minutes} min ago"),
        60..1440 => format!("{} h ago", minutes / 60),
        _ => format!("{} days ago", minutes / 1440),
    }
}
