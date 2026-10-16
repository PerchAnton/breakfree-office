use breakfree_office::gui::{OfficeApp, Page};
use eframe::egui::{self, Key};
use egui_kittest::{
    Harness,
    kittest::{NodeT, Queryable},
};

#[test]
fn desktop_navigation_empty_states_and_menu_are_real_controls() {
    let dir = tempfile::tempdir().unwrap();
    let mut harness = Harness::builder()
        .with_size([1120.0, 760.0])
        .build_eframe(|cc| OfficeApp::new(&cc.egui_ctx, dir.path().join("settings.json")));
    assert!(harness.query_by_label("No recent files").is_some());
    harness.get_by_label("Settings").click();
    harness.run();
    assert_eq!(harness.state().page, Page::Settings);
    for name in [
        "Choose Docs executable…",
        "Choose Sheets executable…",
        "Choose Slides executable…",
        "Check installed editors",
    ] {
        assert!(harness.query_by_label(name).is_some());
    }
    harness.get_by_label("About").click();
    harness.run();
    assert_eq!(harness.state().page, Page::About);
    harness.get_by_label("File").click();
    harness.run();
    assert!(harness.query_by_label("Recent files").is_some());
    harness.get_by_label("Recent files").click();
    harness.run();
    assert_eq!(harness.state().page, Page::Recent);
}

#[test]
fn logical_dpi_and_resize_checks_are_not_claimed_as_windows_screenshots() {
    for ratio in [1.0, 1.25, 1.5] {
        for size in [[880.0, 600.0], [1120.0, 760.0], [1440.0, 900.0]] {
            let dir = tempfile::tempdir().unwrap();
            let mut harness = Harness::builder()
                .with_size(size)
                .with_pixels_per_point(ratio)
                .build_eframe(|cc| OfficeApp::new(&cc.egui_ctx, dir.path().join("settings.json")));
            for (label, page) in [
                ("New", Page::New),
                ("Open", Page::Open),
                ("Recent", Page::Recent),
                ("Settings", Page::Settings),
                ("About", Page::About),
                ("Home", Page::Home),
            ] {
                // AccessKit reports physical bounds; egui pointer input is logical.
                // Node::click in kittest 0.32.3 does not convert scaled bounds.
                let rect = harness.get_by_label(label).rect();
                let pos = rect.center() / ratio;
                assert!(rect.max.x <= size[0] * ratio);
                assert!(rect.max.y <= size[1] * ratio);
                harness.input_mut().events.extend([
                    egui::Event::PointerMoved(pos),
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed: true,
                        modifiers: egui::Modifiers::NONE,
                    },
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        modifiers: egui::Modifiers::NONE,
                    },
                ]);
                harness.run();
                assert_eq!(harness.state().page, page, "{label} at {ratio}, {size:?}");
            }
        }
    }
}

#[test]
fn keyboard_shortcuts_and_focus_reach_navigation() {
    let dir = tempfile::tempdir().unwrap();
    let mut harness = Harness::builder()
        .with_size([1120.0, 760.0])
        .build_eframe(|cc| OfficeApp::new(&cc.egui_ctx, dir.path().join("settings.json")));
    harness.key_press(Key::Tab);
    harness.run();
    // Tab enters the keyboard focus chain; no pointer action is needed to open File.
    harness.key_press(Key::Enter);
    harness.run();
    assert!(harness.query_by_label("Recent files").is_some());
    harness.key_press(Key::Escape);
    harness.run();
    harness.key_press_modifiers(egui::Modifiers::COMMAND, Key::N);
    harness.run();
    assert_eq!(harness.state().page, Page::New);
    harness.key_press_modifiers(egui::Modifiers::ALT, Key::Num5);
    harness.run();
    assert_eq!(harness.state().page, Page::Settings);
}

#[test]
fn transparent_approved_asset_is_not_flattened_against_black() {
    let im = image::load_from_memory(include_bytes!("../docs/brand/breakfree-icon.webp"))
        .unwrap()
        .to_rgba8();
    assert_eq!((im.width(), im.height()), (192, 192));
    assert_eq!(im.get_pixel(0, 0).0[3], 0);
    assert!(im.pixels().any(|p| p.0[3] == 255));
}

#[test]
fn native_frontend_uses_light_visuals_independent_of_document_content() {
    let context = egui::Context::default();
    breakfree_office::visual::apply(&context);
    assert!(!context.style().visuals.dark_mode);
    assert_eq!(
        context.style().visuals.panel_fill,
        breakfree_office::visual::BG
    );
}

#[test]
fn missing_editor_actions_are_disabled_and_cards_align_as_compact_desktop_rows() {
    let dir = tempfile::tempdir().unwrap();
    let harness = Harness::builder()
        .with_size([1120.0, 760.0])
        .build_eframe(|cc| OfficeApp::new(&cc.egui_ctx, dir.path().join("settings.json")));
    let buttons = ["Blank document", "Blank spreadsheet", "Blank presentation"]
        .map(|label| harness.get_by_label(label));
    for node in &buttons {
        assert!(node.accesskit_node().is_disabled());
    }
    for node in &buttons[1..] {
        assert!((node.rect().min.y - buttons[0].rect().min.y).abs() < 2.0);
    }
}
