//! Test-only executable. It is never available in the normal production build.
use std::{
    env, fs,
    io::{self, Write},
    process, thread,
    time::Duration,
};

fn main() {
    let exe = env::current_exe().unwrap();
    let name = exe.file_stem().unwrap().to_string_lossy();
    let args: Vec<String> = env::args().skip(1).collect();
    if args == ["--hold-pipe"] {
        thread::sleep(Duration::from_secs(20));
        return;
    }
    if args == ["--version"] {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if args == ["--capabilities"] {
        if name.contains("timeout") {
            thread::sleep(Duration::from_secs(20));
        }
        if name.contains("malformed") {
            println!("not JSON");
            return;
        }
        if name.contains("stdout-limit") {
            io::stdout().write_all(&vec![b'x'; 100_000]).unwrap();
            return;
        }
        if name.contains("stderr-limit") {
            io::stderr().write_all(&vec![b'x'; 100_000]).unwrap();
            return;
        }
        if name.contains("probe-exit") {
            process::exit(7);
        }
        if name.contains("holding-child") {
            let mut child = process::Command::new(&exe)
                .arg("--hold-pipe")
                .spawn()
                .unwrap();
            thread::spawn(move || {
                let _ = child.wait();
            });
        }
        let app = if name.starts_with("sheets") {
            "sheets"
        } else if name.starts_with("slides") {
            "slides"
        } else {
            "docs"
        };
        let ext = match app {
            "sheets" => "bfsheet",
            "slides" => "bfslide",
            _ => "bfdoc",
        };
        let version = if name.contains("protocol2") { 2 } else { 1 };
        let data = serde_json::json!({"protocol_version":version,"app_id":format!("breakfree.{app}"),"app_version":env!("CARGO_PKG_VERSION"),"display_name":format!("Test-only {app}"),"capabilities":["new","open"],"formats":[{"id":format!("breakfree.{app}.native"),"extensions":[ext],"read":true,"write":true}]});
        println!("{data}");
        return;
    }
    if name.contains("dispatch-exit") {
        process::exit(8);
    }
    fs::write(
        exe.with_extension("args.json"),
        serde_json::to_vec(&args).unwrap(),
    )
    .unwrap();
}
