use crate::{
    contract::{APP_IDS, APP_NAMES},
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
    recent_query: String,
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
            page: {
                #[cfg(feature = "validation-capture")]
                {
                    std::env::var("BREAKFREE_CAPTURE_PAGE")
                        .ok()
                        .and_then(|name| {
                            Page::ALL
                                .into_iter()
                                .find(|p| p.label().eq_ignore_ascii_case(&name))
                        })
                        .unwrap_or(Page::Home)
                }
                #[cfg(not(feature = "validation-capture"))]
                {
                    Page::Home
                }
            },
            registry,
            apps,
            recent,
            recent_query: String::new(),
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
            .exact_height(32.0)
            .frame(
                egui::Frame::new()
                    .fill(visual::NAV)
                    .inner_margin(egui::Margin::symmetric(12, 1)),
            )
            .show(ctx, |ui| {
                ui.visuals_mut().button_frame = false;
                ui.horizontal(|ui| {
                    let _file_menu = ui.menu_button("File", |ui| {
                        if ui.button("Home").clicked() {
                            self.page = Page::Home;
                            ui.close();
                        }
                        if ui.button(format!("New\t{}", command_hint("N"))).clicked() {
                            self.page = Page::New;
                            ui.close();
                        }
                        if ui
                            .add_enabled(
                                !self.busy() && self.registry.is_some(),
                                egui::Button::new(format!("Open…\t{}", command_hint("O"))),
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
                    #[cfg(feature = "validation-capture")]
                    if self.capture_frames == 2
                        && std::env::var("BREAKFREE_CAPTURE_STATE").as_deref() == Ok("menu")
                    {
                        egui::Popup::open_id(
                            ctx,
                            egui::Popup::default_response_id(&_file_menu.response),
                        );
                    }
                    ui.menu_button("Help", |ui| {
                        if ui.button("About BreakFree Office").clicked() {
                            self.page = Page::About;
                            ui.close();
                        }
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(
                            RichText::new("BreakFree Office")
                                .size(12.0)
                                .color(visual::MUTED),
                        );
                    });
                });
            });
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        #[cfg(feature = "validation-capture")]
        if let Ok(path) = std::env::var("BREAKFREE_CAPTURE_TO") {
            self.capture_frames += 1;
            ctx.request_repaint();
            if self.capture_frames == 24 {
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
        if self.choice.is_some()
            && ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
        {
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
        self.navigation(ctx);
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(visual::BG).inner_margin(visual::CONTENT_MARGIN))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().id_salt("page-content").auto_shrink([false, false]).show(ui, |ui| {
                    ui.heading(self.page.label());
                    ui.add_space(24.0);
                    if self.error {
                        egui::Frame::new().fill(Color32::from_rgb(255, 244, 231)).inner_margin(12).show(ui, |ui| {
                            ui.label(&self.notice);
                            if ui.button("Dismiss message").clicked() { self.error = false; }
                        });
                        ui.add_space(16.0);
                    }
                    match self.page {
                        Page::Home => {
                            self.app_cards(ui, ctx);
                            ui.add_space(30.0);
                            self.recent_files(ui, ctx, true);
                        }
                        Page::New => {
                            self.app_cards(ui, ctx);
                            ui.add_space(30.0);
                            section(ui, "Templates");
                            ui.add_space(8.0);
                            ui.label(RichText::new("No templates are installed.").color(visual::MUTED));
                        }
                        Page::Open => self.open_page(ui, ctx),
                        Page::Recent => self.recent_files(ui, ctx, false),
                        Page::Settings => self.settings(ui, ctx),
                        Page::About => {
                            ui.horizontal(|ui| {
                                if let Some(logo) = &self.logo { ui.image((logo.id(), Vec2::splat(48.0))); }
                                ui.vertical(|ui| {
                                    ui.label(RichText::new("BreakFree Office").size(20.0).strong());
                                    ui.label("Your work. Your freedom.");
                                });
                            });
                            ui.add_space(24.0);
                            ui.label(format!("Version {} · Rust desktop prototype", env!("CARGO_PKG_VERSION")));
                            ui.label(format!("Visual system {} · integration protocol 1", visual::VERSION));
                            ui.label(format!("Interface font: {}", self.font_name));
                            ui.add_space(18.0);
                            ui.label("Independent editors for documents, spreadsheets and presentations.");
                            ui.label("Office launches editors; it does not edit your documents. No accounts or online services are required.");
                            ui.add_space(18.0);
                            ui.label(RichText::new("Microsoft Office file compatibility and Windows visual acceptance have not been verified. No project software license has been selected.").color(visual::MUTED));
                        }
                    }
                    if !self.error && self.notice != "Ready" {
                        ui.add_space(20.0);
                        ui.label(RichText::new(&self.notice).small().color(visual::MUTED));
                    }
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

    fn navigation(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("navigation")
            .exact_width(visual::NAV_WIDTH)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(visual::NAV)
                    .inner_margin(egui::Margin::symmetric(8, 12)),
            )
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 4.0;
                let compact = ui.available_height() < 410.0;
                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    if let Some(logo) = &self.logo {
                        ui.image((logo.id(), Vec2::splat(28.0)));
                    }
                    ui.label(RichText::new("BreakFree Office").size(14.0));
                });
                ui.add_space(if compact { 4.0 } else { 30.0 });
                for page in [Page::Home, Page::New, Page::Open, Page::Recent] {
                    self.navigation_item(ui, page);
                }
                // Reserve the footer while retaining top-to-bottom keyboard order.
                ui.add_space(
                    (ui.available_height() - 2.0 * (visual::ROW_HEIGHT + 4.0) - 12.0).max(0.0),
                );
                ui.separator();
                for page in [Page::Settings, Page::About] {
                    self.navigation_item(ui, page);
                }
            });
    }

    fn navigation_item(&mut self, ui: &mut egui::Ui, page: Page) {
        let selected = self.page == page;
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(ui.available_width(), visual::ROW_HEIGHT),
            Sense::click(),
        );
        #[cfg(feature = "validation-capture")]
        if page == Page::New
            && self.capture_frames == 2
            && std::env::var("BREAKFREE_CAPTURE_STATE").as_deref() == Ok("focus")
        {
            response.request_focus();
        }
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::SelectableLabel,
                ui.is_enabled(),
                selected,
                page.label(),
            )
        });
        if selected || response.hovered() || response.is_pointer_button_down_on() {
            ui.painter().rect_filled(
                rect,
                2,
                if selected {
                    visual::SELECTED
                } else {
                    visual::HOVER
                },
            );
        }
        if selected {
            let marker = egui::Rect::from_min_size(
                rect.left_top() + egui::vec2(0.0, 10.0),
                egui::vec2(3.0, 20.0),
            );
            ui.painter().rect_filled(marker, 1, visual::ACCENT);
        }
        let icon = match page {
            Page::Home => Glyph::Home,
            Page::New => Glyph::New,
            Page::Open => Glyph::Folder,
            Page::Recent => Glyph::Clock,
            Page::Settings => Glyph::Settings,
            Page::About => Glyph::Info,
        };
        draw_glyph(
            ui.painter(),
            rect.left_center() + egui::vec2(24.0, 0.0),
            icon,
            visual::INK,
        );
        ui.painter().text(
            rect.left_center() + egui::vec2(48.0, 0.0),
            egui::Align2::LEFT_CENTER,
            page.label(),
            egui::FontId::proportional(14.0),
            visual::INK,
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
    }

    fn open_page(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        // A narrow viewport stacks the location pane above the files, rather than clipping it.
        if ui.available_width() < 620.0 {
            self.local_location(ui, ctx);
            ui.add_space(24.0);
            self.recent_files(ui, ctx, false);
        } else {
            ui.horizontal_top(|ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(198.0, 220.0),
                    Layout::top_down(Align::Min),
                    |ui| self.local_location(ui, ctx),
                );
                ui.separator();
                ui.add_space(16.0);
                ui.vertical(|ui| self.recent_files(ui, ctx, false));
            });
        }
    }

    fn local_location(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        section(ui, "This computer");
        ui.add_space(12.0);
        ui.label(
            RichText::new("Open a local document, spreadsheet or presentation.")
                .color(visual::MUTED),
        );
        ui.add_space(12.0);
        if button(
            ui,
            "Browse this computer…",
            !self.busy() && self.registry.is_some(),
        )
        .clicked()
        {
            self.open_picker(ctx);
        }
    }

    fn app_cards(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        section(ui, "Create new");
        ui.add_space(12.0);
        let enabled = !self.busy() && self.registry.is_some();
        ui.spacing_mut().item_spacing.x = 16.0;
        let columns = ((ui.available_width() + 16.0) / (visual::CARD_WIDTH + 16.0))
            .floor()
            .clamp(1.0, 3.0) as usize;
        for row in [0, 1, 2].chunks(columns) {
            ui.horizontal_top(|ui| {
                for &i in row {
                    let ready = self.apps[i].manifest.is_some();
                    let can_new = self.apps[i]
                        .manifest
                        .as_ref()
                        .is_some_and(|m| m.supports("new"));
                    let label = ["Blank document", "Blank spreadsheet", "Blank presentation"][i];
                    ui.push_id(("new-file", i), |ui| {
                        ui.allocate_ui_with_layout(
                            egui::vec2(visual::CARD_WIDTH, visual::CARD_HEIGHT + 28.0),
                            Layout::top_down(Align::Center),
                            |ui| {
                                // Preserve readable contrast while exposing the real disabled state.
                                let painter = ui.painter().clone();
                                let response = ui
                                    .add_enabled_ui(enabled && can_new, |ui| {
                                        let (rect, response) = ui.allocate_exact_size(
                                            egui::vec2(visual::CARD_WIDTH, visual::CARD_HEIGHT),
                                            Sense::click(),
                                        );
                                        response.widget_info(|| {
                                            egui::WidgetInfo::labeled(
                                                egui::WidgetType::Button,
                                                ui.is_enabled(),
                                                label,
                                            )
                                        });
                                        painter.rect(
                                            rect,
                                            2,
                                            if response.hovered() {
                                                visual::HOVER
                                            } else {
                                                visual::NAV
                                            },
                                            egui::Stroke::new(
                                                1.0,
                                                if response.hovered() {
                                                    visual::ACCENT
                                                } else {
                                                    visual::LINE
                                                },
                                            ),
                                            egui::StrokeKind::Inside,
                                        );
                                        document_preview(&painter, rect, i);
                                        painter.text(
                                            rect.left_bottom() + egui::vec2(12.0, -32.0),
                                            egui::Align2::LEFT_CENTER,
                                            label,
                                            egui::FontId::proportional(13.0),
                                            visual::INK,
                                        );
                                        painter.text(
                                            rect.left_bottom() + egui::vec2(12.0, -13.0),
                                            egui::Align2::LEFT_CENTER,
                                            APP_NAMES[i],
                                            egui::FontId::proportional(12.0),
                                            visual::MUTED,
                                        );
                                        focus(ui, &response);
                                        response
                                    })
                                    .inner;
                                if response.clicked() {
                                    self.start(Job::New(i), ctx);
                                }
                                let status = if ready {
                                    if can_new {
                                        "Ready"
                                    } else {
                                        "Creation unavailable"
                                    }
                                } else if self.apps[i].path.is_none() {
                                    "Not installed"
                                } else {
                                    "Unavailable"
                                };
                                response.on_disabled_hover_text(if !ready {
                                    "Set up this editor in Settings to create files."
                                } else {
                                    "This editor cannot create a new file right now."
                                });
                                ui.label(RichText::new(status).size(12.0).color(visual::MUTED));
                            },
                        );
                    });
                }
            });
        }
        if self.apps.iter().any(|a| a.manifest.is_none()) {
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    RichText::new("Choose installed editors.")
                        .small()
                        .color(visual::MUTED),
                );
                if ui.link("Set up editors").clicked() {
                    self.page = Page::Settings;
                }
            });
        }
    }

    fn recent_files(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, compact: bool) {
        section(ui, "Recent files");
        ui.add_space(10.0);
        if !self.recent.is_empty() {
            let search = ui.add_sized(
                [ui.available_width().min(400.0), 30.0],
                egui::TextEdit::singleline(&mut self.recent_query).hint_text("Search recent files"),
            );
            search.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::TextEdit, true, "Search recent files")
            });
            ui.add_space(12.0);
        }
        let width = ui.available_width();
        let (header, _) = ui.allocate_exact_size(egui::vec2(width, 28.0), Sense::hover());
        ui.painter().text(
            header.left_center() + egui::vec2(40.0, 0.0),
            egui::Align2::LEFT_CENTER,
            "Name",
            egui::FontId::proportional(12.0),
            visual::MUTED,
        );
        if width >= 330.0 {
            ui.painter().text(
                header.right_center() - egui::vec2(12.0, 0.0),
                egui::Align2::RIGHT_CENTER,
                "Last opened",
                egui::FontId::proportional(12.0),
                visual::MUTED,
            );
        }
        ui.painter().hline(
            header.x_range(),
            header.bottom(),
            egui::Stroke::new(1.0, visual::LINE),
        );
        if self.recent.is_empty() {
            ui.add_space(24.0);
            ui.horizontal(|ui| {
                let (icon, _) = ui.allocate_exact_size(egui::vec2(32.0, 40.0), Sense::hover());
                draw_glyph(ui.painter(), icon.center(), Glyph::Clock, visual::MUTED);
                ui.vertical(|ui| {
                    ui.label("No recent files");
                    ui.label(
                        RichText::new("Files you open with an editor will appear here.")
                            .small()
                            .color(visual::MUTED),
                    );
                });
            });
            ui.add_space(16.0);
            if button(ui, "Open a file…", !self.busy() && self.registry.is_some()).clicked() {
                self.open_picker(ctx);
            }
            return;
        }
        let query = self.recent_query.trim().to_lowercase();
        let recent: Vec<_> = self
            .recent
            .iter()
            .filter(|entry| entry.path.to_string_lossy().to_lowercase().contains(&query))
            .cloned()
            .collect();
        if recent.is_empty() {
            ui.add_space(16.0);
            ui.label("No matching recent files");
            if ui.button("Clear search").clicked() {
                self.recent_query.clear();
            }
        }
        for entry in recent.iter().take(if compact { 6 } else { 30 }) {
            let enabled = !self.busy() && self.registry.is_some();
            let response = ui
                .push_id(&entry.path, |ui| {
                    ui.add_enabled_ui(enabled, |ui| {
                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(ui.available_width(), visual::FILE_ROW_HEIGHT),
                            Sense::click(),
                        );
                        response.widget_info(|| {
                            egui::WidgetInfo::labeled(
                                egui::WidgetType::Button,
                                ui.is_enabled(),
                                format!("Open {}", entry.path.display()),
                            )
                        });
                        if response.hovered() {
                            ui.painter().rect_filled(rect, 0, visual::NAV);
                        }
                        let app = APP_IDS
                            .iter()
                            .position(|id| *id == entry.app_id)
                            .unwrap_or(0);
                        file_symbol(
                            ui.painter(),
                            rect.left_center() + egui::vec2(18.0, 0.0),
                            app,
                        );
                        let name = entry.path.file_name().unwrap_or_default().to_string_lossy();
                        let text_width = (rect.width()
                            - if rect.width() >= 330.0 { 160.0 } else { 52.0 })
                        .max(20.0);
                        clipped_text(
                            ui,
                            rect.min + egui::vec2(40.0, 9.0),
                            &name,
                            14.0,
                            visual::INK,
                            text_width,
                        );
                        let parent = entry
                            .path
                            .parent()
                            .unwrap_or(&entry.path)
                            .display()
                            .to_string();
                        clipped_text(
                            ui,
                            rect.min + egui::vec2(40.0, 32.0),
                            &parent,
                            12.0,
                            visual::MUTED,
                            text_width,
                        );
                        if rect.width() >= 330.0 {
                            ui.painter().text(
                                rect.right_center() - egui::vec2(12.0, 0.0),
                                egui::Align2::RIGHT_CENTER,
                                age(entry.opened_at),
                                egui::FontId::proportional(12.0),
                                visual::MUTED,
                            );
                        }
                        focus(ui, &response);
                        response
                    })
                    .inner
                })
                .inner;
            if response.clicked() {
                self.start(Job::Open(entry.path.clone(), None), ctx);
            }
            response
                .on_hover_text(entry.path.display().to_string())
                .context_menu(|ui| {
                    if button(ui, "Remove from recent list", enabled).clicked() {
                        self.start(Job::RemoveRecent(Some(entry.path.clone())), ctx);
                        ui.close();
                    }
                });
        }
        ui.add_space(12.0);
        if compact {
            if ui.link("Show all recent files").clicked() {
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
            ui.horizontal_wrapped(|ui| {
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

fn command_hint(key: &str) -> String {
    format!(
        "{}+{key}",
        if cfg!(target_os = "macos") {
            "Cmd"
        } else {
            "Ctrl"
        }
    )
}

fn section(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).size(18.0));
}

fn clipped_text(ui: &egui::Ui, pos: egui::Pos2, text: &str, size: f32, color: Color32, width: f32) {
    let mut job = egui::text::LayoutJob::simple_singleline(
        text.into(),
        egui::FontId::proportional(size),
        color,
    );
    job.wrap = egui::text::TextWrapping {
        max_width: width,
        max_rows: 1,
        break_anywhere: true,
        ..Default::default()
    };
    let galley = ui.fonts(|fonts| fonts.layout_job(job));
    ui.painter().galley(pos, galley, color);
}

#[derive(Clone, Copy)]
enum Glyph {
    Home,
    New,
    Folder,
    Clock,
    Settings,
    Info,
}

// Original line pictograms drawn with Rust; no external icon font or Microsoft assets.
fn draw_glyph(p: &egui::Painter, center: egui::Pos2, icon: Glyph, color: Color32) {
    let stroke = egui::Stroke::new(1.3, color);
    let at = |x, y| center + egui::vec2(x, y);
    let line = |points: &[(f32, f32)]| {
        p.add(egui::Shape::line(
            points.iter().map(|&(x, y)| at(x, y)).collect(),
            stroke,
        ));
    };
    match icon {
        Glyph::Home => {
            line(&[(-9.0, -1.0), (0.0, -8.0), (9.0, -1.0)]);
            line(&[
                (-6.0, -2.0),
                (-6.0, 8.0),
                (-2.0, 8.0),
                (-2.0, 2.0),
                (2.0, 2.0),
                (2.0, 8.0),
                (6.0, 8.0),
                (6.0, -2.0),
            ]);
        }
        Glyph::New => {
            line(&[
                (-6.0, 8.0),
                (-6.0, -8.0),
                (3.0, -8.0),
                (7.0, -4.0),
                (7.0, 0.0),
            ]);
            line(&[(3.0, -8.0), (3.0, -4.0), (7.0, -4.0)]);
            line(&[(0.0, 5.0), (10.0, 5.0)]);
            line(&[(5.0, 0.0), (5.0, 10.0)]);
        }
        Glyph::Folder => {
            line(&[
                (-9.0, 7.0),
                (-9.0, -6.0),
                (-2.0, -6.0),
                (0.0, -3.0),
                (8.0, -3.0),
                (8.0, 0.0),
            ]);
            line(&[
                (-9.0, 7.0),
                (-5.0, -1.0),
                (10.0, -1.0),
                (6.0, 7.0),
                (-9.0, 7.0),
            ]);
        }
        Glyph::Clock => {
            p.circle_stroke(center, 8.0, stroke);
            line(&[(0.0, -5.0), (0.0, 0.0), (4.0, 2.0)]);
        }
        Glyph::Settings => {
            p.circle_stroke(center, 6.0, stroke);
            p.circle_stroke(center, 2.0, stroke);
            for n in 0..8 {
                let a = n as f32 * std::f32::consts::TAU / 8.0;
                p.line_segment(
                    [
                        at(a.cos() * 6.0, a.sin() * 6.0),
                        at(a.cos() * 9.0, a.sin() * 9.0),
                    ],
                    stroke,
                );
            }
        }
        Glyph::Info => {
            p.circle_stroke(center, 8.0, stroke);
            p.circle_filled(at(0.0, -4.0), 0.9, color);
            line(&[(0.0, -1.0), (0.0, 5.0)]);
        }
    }
}

fn file_symbol(p: &egui::Painter, center: egui::Pos2, kind: usize) {
    let colors = [
        visual::ACCENT,
        Color32::from_rgb(36, 112, 105),
        Color32::from_rgb(173, 75, 59),
    ];
    let rect = egui::Rect::from_center_size(center, egui::vec2(21.0, 27.0));
    p.rect(
        rect,
        1,
        Color32::WHITE,
        egui::Stroke::new(1.0, colors[kind]),
        egui::StrokeKind::Inside,
    );
    p.rect_filled(
        egui::Rect::from_min_size(rect.min, egui::vec2(3.0, 27.0)),
        0,
        colors[kind],
    );
    p.text(
        center + egui::vec2(1.0, 0.0),
        egui::Align2::CENTER_CENTER,
        ["D", "S", "P"][kind],
        egui::FontId::proportional(12.0),
        colors[kind],
    );
}

fn document_preview(p: &egui::Painter, card: egui::Rect, kind: usize) {
    let center = card.center_top() + egui::vec2(0.0, 50.0);
    let size = [
        egui::vec2(54.0, 72.0),
        egui::vec2(96.0, 64.0),
        egui::vec2(100.0, 57.0),
    ][kind];
    let paper = egui::Rect::from_center_size(center, size);
    p.rect_filled(
        paper.translate(egui::vec2(2.0, 2.0)),
        0,
        Color32::from_black_alpha(12),
    );
    p.rect(
        paper,
        0,
        Color32::WHITE,
        egui::Stroke::new(1.0, Color32::from_gray(183)),
        egui::StrokeKind::Inside,
    );
    if kind == 1 {
        let grid = paper.shrink(5.0);
        p.rect_filled(
            egui::Rect::from_min_size(grid.min, egui::vec2(grid.width(), 8.0)),
            0,
            visual::NAV,
        );
        for n in 1..6 {
            let y = grid.top() + n as f32 * 9.0;
            p.hline(grid.x_range(), y, egui::Stroke::new(0.5, visual::LINE));
        }
        for n in 1..5 {
            let x = grid.left() + n as f32 * 17.0;
            p.vline(x, grid.y_range(), egui::Stroke::new(0.5, visual::LINE));
        }
    }
}
