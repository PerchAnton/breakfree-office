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
    #[cfg(feature = "validation-capture")]
    let window_size = match env::var("BREAKFREE_CAPTURE_SIZE").as_deref() {
        Ok("minimum") => [880.0, 600.0],
        Ok("large") => [1440.0, 900.0],
        _ => [1120.0, 760.0],
    };
    #[cfg(not(feature = "validation-capture"))]
    let window_size = [1120.0, 760.0];
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size(window_size)
            .with_min_inner_size([880.0, 600.0])
            .with_icon(eframe::egui::IconData {
                rgba: icon.as_raw().clone(),
                width: icon.width(),
                height: icon.height(),
            }),
        ..Default::default()
    };
    #[cfg(feature = "validation-capture")]
    let file: PathBuf = env::var_os("BREAKFREE_CAPTURE_SETTINGS")
        .map(PathBuf::from)
        .unwrap_or_else(registry::settings_path);
    #[cfg(not(feature = "validation-capture"))]
    let file: PathBuf = registry::settings_path();
    eframe::run_native(
        "BreakFree Office",
        options,
        Box::new(move |cc| Ok(Box::new(OfficeApp::new(&cc.egui_ctx, file)))),
    )?;
    Ok(())
}
