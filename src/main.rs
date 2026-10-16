#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use breakfree_office::{contract, gui::OfficeApp, registry};
use std::{env, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    match args.as_slice() {
        [arg] if arg == "--version" => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        [arg, file] if arg == "--check-manifest" => {
            use std::io::Read;
            let mut bytes = Vec::new();
            fs::File::open(file)?
                .take((breakfree_office::process::STDOUT_LIMIT + 1) as u64)
                .read_to_end(&mut bytes)?;
            if bytes.len() > breakfree_office::process::STDOUT_LIMIT {
                return Err("Manifest exceeds the size limit.".into());
            }
            let manifest = contract::validate(&bytes, None)?;
            println!(
                "Valid protocol 1 response for {} ({})",
                manifest.app_id, manifest.app_version
            );
            return Ok(());
        }
        [arg] if arg == "--check-installations" => {
            let mut registry = registry::Registry::new(registry::settings_path())?;
            registry.refresh();
            for (id, app) in contract::APP_IDS.iter().zip(&registry.apps) {
                println!(
                    "{id}: {}",
                    if app.manifest.is_some() {
                        "ready"
                    } else {
                        app.error.as_deref().unwrap_or("not installed")
                    }
                );
            }
            return if registry
                .apps
                .iter()
                .any(|a| a.path.is_some() && a.error.is_some())
            {
                Err("One or more configured editors are unavailable.".into())
            } else {
                Ok(())
            };
        }
        [] => {}
        _ => return Err(
            "Usage: breakfree-office [--version | --check-manifest FILE | --check-installations]"
                .into(),
        ),
    }
    let icon =
        image::load_from_memory(include_bytes!("../docs/brand/breakfree-icon.webp"))?.to_rgba8();
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1120.0, 760.0])
            .with_min_inner_size([880.0, 600.0])
            .with_icon(eframe::egui::IconData {
                rgba: icon.as_raw().clone(),
                width: icon.width(),
                height: icon.height(),
            }),
        ..Default::default()
    };
    let file: PathBuf = registry::settings_path();
    eframe::run_native(
        "BreakFree Office",
        options,
        Box::new(move |cc| Ok(Box::new(OfficeApp::new(&cc.egui_ctx, file)))),
    )?;
    Ok(())
}
