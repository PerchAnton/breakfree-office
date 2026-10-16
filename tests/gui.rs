use breakfree_office::gui::{OfficeApp, Page};
use eframe::egui::{self, Key, accesskit::Role};
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
    assert!(
        harness
            .query_by_role_and_label(Role::Button, "Recent files")
            .is_some()
    );
    harness
        .get_by_role_and_label(Role::Button, "Recent files")
        .click();
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
                let rect = harness.get_by_role_and_label(Role::Button, label).rect();
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
    assert!(
        harness
            .query_by_role_and_label(Role::Button, "Recent files")
            .is_some()
    );
    harness.key_press(Key::Escape);
    harness.run();
    assert!(
        harness
            .query_by_role_and_label(Role::Button, "Recent files")
            .is_none()
    );
    harness.get_by_label("New").focus();
    harness.run();
    assert!(harness.get_by_label("New").is_focused());
    harness.key_press(Key::Enter);
    harness.run();
    assert_eq!(harness.state().page, Page::New);
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

#[test]
fn zoomed_minimum_window_keeps_navigation_reachable_and_cards_inside_content() {
    for zoom in [1.0, 1.25, 1.5] {
        let dir = tempfile::tempdir().unwrap();
        let mut harness = Harness::builder()
            .with_size([880.0, 600.0])
            .build_eframe(|cc| OfficeApp::new(&cc.egui_ctx, dir.path().join("settings.json")));
        harness.ctx.set_zoom_factor(zoom);
        harness.run();
        for label in ["Home", "New", "Open", "Recent", "Settings", "About"] {
            let rect = harness.get_by_role_and_label(Role::Button, label).rect();
            assert!(
                rect.min.x >= 0.0 && rect.max.x <= 880.0,
                "{label}: {rect:?} at {zoom}"
            );
            assert!(
                rect.min.y >= 32.0 * zoom && rect.max.y <= 574.0,
                "{label}: {rect:?} at {zoom}"
            );
        }
        for label in ["Blank document", "Blank spreadsheet", "Blank presentation"] {
            let rect = harness.get_by_role_and_label(Role::Button, label).rect();
            assert!(
                rect.min.x >= 176.0 * zoom && rect.max.x <= 880.0,
                "{label}: {rect:?} at {zoom}"
            );
        }
        harness.get_by_label("Set up editors").scroll_to_me();
        harness.run();
        let setup = harness.get_by_label("Set up editors").rect();
        assert!(
            setup.min.y >= 32.0 * zoom && setup.max.y <= 574.0,
            "Setup link must scroll into view at {zoom}: {setup:?}"
        );
        // Scrolling Home must not make the next page start halfway down its content.
        harness.state_mut().page = Page::Settings;
        harness.run();
        let first_setting = harness.get_by_label("Choose Docs executable…").rect();
        assert!(
            first_setting.min.y >= 32.0 * zoom && first_setting.max.y <= 574.0,
            "Settings must start at the top, independently of Home: {first_setting:?}"
        );
        harness.state_mut().page = Page::Home;
        harness.run();
        harness.set_size(egui::vec2(1440.0, 900.0));
        harness.run();
        let top = harness.get_by_label("Blank document").rect().top();
        assert_eq!(harness.get_by_label("Blank presentation").rect().top(), top);
    }
}

#[test]
fn recent_search_filters_existing_records_and_can_be_cleared_without_mutation() {
    use breakfree_office::registry::{Recent, Settings};
    let dir = tempfile::tempdir().unwrap();
    let paths = [
        dir.path().join("Budget 2026.xlsx"),
        dir.path().join("План роботи.docx"),
    ];
    for path in &paths {
        std::fs::write(path, b"GUI test fixture; not an office document").unwrap();
    }
    let settings = Settings {
        recent: paths
            .iter()
            .enumerate()
            .map(|(i, path)| Recent {
                path: path.clone(),
                app_id: ["breakfree.sheets", "breakfree.docs"][i].into(),
                opened_at: 1,
            })
            .collect(),
        ..Default::default()
    };
    let file = dir.path().join("settings.json");
    let original = serde_json::to_vec(&settings).unwrap();
    std::fs::write(&file, &original).unwrap();
    let mut harness = Harness::builder()
        .with_size([1120.0, 760.0])
        .build_eframe(|cc| OfficeApp::new(&cc.egui_ctx, file.clone()));
    let labels = paths.map(|p| format!("Open {}", p.display()));
    for label in &labels {
        assert!(harness.query_by_label(label).is_some());
    }
    let input = harness.get_by_label("Search recent files");
    input.focus();
    input.type_text("BUDGET");
    harness.run();
    assert!(harness.query_by_label(&labels[0]).is_some());
    assert!(harness.query_by_label(&labels[1]).is_none());
    harness
        .get_by_label("Search recent files")
        .type_text(" not present");
    harness.run();
    assert!(harness.query_by_label("No matching recent files").is_some());
    harness.get_by_label("Clear search").click();
    harness.run();
    for label in &labels {
        assert!(harness.query_by_label(label).is_some());
    }
    assert_eq!(std::fs::read(&file).unwrap(), original);
}

#[test]
fn backstage_locations_switch_with_keyboard_and_reflow_at_small_sizes() {
    for size in [[880.0, 600.0], [1120.0, 760.0]] {
        let dir = tempfile::tempdir().unwrap();
        let mut harness = Harness::builder()
            .with_size(size)
            .build_eframe(|cc| OfficeApp::new(&cc.egui_ctx, dir.path().join("settings.json")));
        harness.state_mut().page = Page::Open;
        harness.run();
        assert!(harness.query_by_label("No recent files").is_some());
        harness
            .get_by_role_and_label(Role::Button, "This computer")
            .focus();
        harness.run();
        harness.key_press(Key::Enter);
        harness.run();
        assert!(harness.query_by_label("Local files").is_some());
        assert!(harness.query_by_label("Browse this computer…").is_some());
        assert!(harness.query_by_label("No recent files").is_none());
        let browse = harness.get_by_label("Browse this computer…").rect();
        assert!(browse.max.x <= size[0] && browse.max.y <= size[1]);
        harness.get_by_label("Recent documents").focus();
        harness.run();
        harness.key_press(Key::Space);
        harness.run();
        assert!(harness.query_by_label("No recent files").is_some());
        assert!(harness.query_by_label("Local files").is_none());
    }
}

#[test]
fn recent_options_work_from_keyboard_and_remove_only_the_selected_record() {
    use breakfree_office::registry::{Recent, Settings};
    let dir = tempfile::tempdir().unwrap();
    let paths = [dir.path().join("First.txt"), dir.path().join("Second.txt")];
    for path in &paths {
        std::fs::write(path, b"Unchanged test content").unwrap();
    }
    let settings = Settings {
        recent: paths
            .iter()
            .map(|path| Recent {
                path: path.clone(),
                app_id: "breakfree.docs".into(),
                opened_at: 1,
            })
            .collect(),
        ..Default::default()
    };
    let file = dir.path().join("settings.json");
    std::fs::write(&file, serde_json::to_vec(&settings).unwrap()).unwrap();
    let mut harness = Harness::builder()
        .with_size([1120.0, 760.0])
        .build_eframe(|cc| OfficeApp::new(&cc.egui_ctx, file.clone()));
    harness.state_mut().page = Page::Recent;
    harness.run();
    let action_label = format!("More options for {}", paths[0].display());
    let open_label = format!("Open {}", paths[0].display());
    let action = harness.get_by_label(&action_label);
    assert!(action.rect().left() >= harness.get_by_label(&open_label).rect().right());
    action.focus();
    harness.run();
    harness.key_press(Key::Enter);
    harness.run();
    assert!(harness.query_by_label("Remove from recent list").is_some());
    harness.key_press(Key::Escape);
    harness.run();
    assert!(harness.query_by_label("Remove from recent list").is_none());
    harness.get_by_label(&action_label).focus();
    harness.run();
    harness.key_press(Key::Enter);
    harness.run();
    harness.get_by_label("Remove from recent list").focus();
    harness.run();
    harness.key_press(Key::Enter);
    for _ in 0..100 {
        harness.step();
        if harness.query_by_label(&open_label).is_none() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    assert!(harness.query_by_label(&open_label).is_none());
    assert!(
        harness
            .query_by_label(&format!("Open {}", paths[1].display()))
            .is_some()
    );
    let saved: Settings = serde_json::from_slice(&std::fs::read(file).unwrap()).unwrap();
    assert_eq!(saved.recent.len(), 1);
    assert_eq!(saved.recent[0].path, paths[1]);
    for path in paths {
        assert_eq!(std::fs::read(path).unwrap(), b"Unchanged test content");
    }
}
