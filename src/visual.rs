//! Visual system v1, expressed in logical device-independent pixels.
use eframe::egui::{self, Color32, FontData, FontDefinitions, FontFamily, FontId, TextStyle};

pub const VERSION: &str = "1.1.0";
pub const NAV_WIDTH: f32 = 176.0;
pub const ROW_HEIGHT: f32 = 40.0;
pub const CONTENT_MARGIN: i8 = 36;
pub const BG: Color32 = Color32::from_rgb(255, 255, 255);
pub const NAV: Color32 = Color32::from_rgb(243, 243, 243);
pub const INK: Color32 = Color32::from_rgb(36, 36, 36);
pub const MUTED: Color32 = Color32::from_rgb(97, 97, 97);
pub const LINE: Color32 = Color32::from_rgb(222, 222, 222);
pub const ACCENT: Color32 = Color32::from_rgb(105, 65, 155);

pub const HOVER: Color32 = Color32::from_rgb(235, 235, 235);
pub const SELECTED: Color32 = Color32::from_rgb(230, 230, 230);
pub const CARD_WIDTH: f32 = 176.0;
pub const CARD_HEIGHT: f32 = 152.0;
pub const FILE_ROW_HEIGHT: f32 = 60.0;

pub fn apply(ctx: &egui::Context) -> String {
    let mut style = (*ctx.style()).clone();
    style.visuals = egui::Visuals::light();
    style.visuals.panel_fill = BG;
    style.visuals.window_fill = Color32::WHITE;
    style.visuals.override_text_color = Some(INK);
    style.visuals.hyperlink_color = ACCENT;
    style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, LINE);
    style.visuals.window_stroke = egui::Stroke::new(1.0, LINE);
    style.visuals.selection.bg_fill = Color32::from_rgb(233, 226, 242);
    style.visuals.selection.stroke = egui::Stroke::new(1.0, ACCENT);
    style.visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(2);
    style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(2);
    style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(2);
    style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(2);
    style.visuals.widgets.inactive.bg_fill = Color32::WHITE;
    style.visuals.widgets.inactive.weak_bg_fill = Color32::WHITE;
    style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, LINE);
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(237, 237, 237);
    style.visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(237, 237, 237);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(226, 226, 226);
    style.visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, ACCENT);
    style.spacing.scroll = egui::style::ScrollStyle::solid();
    style.spacing.scroll.foreground_color = true;
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(12.0, 5.0);
    style.spacing.interact_size = egui::vec2(36.0, 30.0);
    style
        .text_styles
        .insert(TextStyle::Body, FontId::proportional(14.0));
    style
        .text_styles
        .insert(TextStyle::Button, FontId::proportional(14.0));
    style
        .text_styles
        .insert(TextStyle::Small, FontId::proportional(12.0));
    style
        .text_styles
        .insert(TextStyle::Heading, FontId::proportional(28.0));
    ctx.set_style(style);
    // Read the user's system font without redistributing it with the application.
    let candidates: &[(&str, &str)] = if cfg!(windows) {
        &[("C:/Windows/Fonts/segoeui.ttf", "Segoe UI (system)")]
    } else if cfg!(target_os = "macos") {
        &[(
            "/System/Library/Fonts/Supplemental/Arial.ttf",
            "Arial (system, macOS fallback)",
        )]
    } else {
        &[(
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "DejaVu Sans (system fallback)",
        )]
    };
    for (path, name) in candidates {
        if let Ok(bytes) = std::fs::read(path) {
            let mut fonts = FontDefinitions::default();
            fonts
                .font_data
                .insert("system-ui".into(), FontData::from_owned(bytes).into());
            fonts
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, "system-ui".into());
            ctx.set_fonts(fonts);
            return name.to_string();
        }
    }
    "Bundled egui fallback (Ubuntu Light; licensed upstream)".into()
}
