use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("sync-web-ui") => {
            let apps: Vec<String> = args.collect();
            sync_web_ui(apps);
        }
        Some(cmd) => {
            eprintln!("Unknown command: {}", cmd);
            std::process::exit(1);
        }
        None => {
            eprintln!("Usage: cargo xtask <command>");
            eprintln!("Commands:");
            eprintln!("  sync-web-ui   Sync shared-assets web UI styles into companion apps");
            std::process::exit(1);
        }
    }
}

fn sync_web_ui(apps: Vec<String>) {
    let current_dir = env::current_dir().unwrap();
    // xtask is typically run from the workspace root (shared-assets/shared-rust)
    // so shared-assets root is one level up.
    let root = current_dir.parent().unwrap();
    let styles_src = root.join("styles");
    
    if apps.is_empty() {
        // Run for all apps in the parent directory
        let parent = root.parent().unwrap();
        if let Ok(entries) = fs::read_dir(parent) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let name = entry.file_name();
                        let name_str = name.to_string_lossy();
                        if name_str == "shared-assets" || name_str == "studio2201.github.io" || name_str == ".github" {
                            continue;
                        }
                        sync_one(&entry.path(), &styles_src);
                    }
                }
            }
        }
    } else {
        for app in apps {
            let app_path = Path::new(&app).canonicalize().unwrap();
            sync_one(&app_path, &styles_src);
        }
    }
}

fn sync_one(app: &Path, styles_src: &Path) {
    let assets = app.join("assets").join("shared-assets");
    let frontend = app.join("frontend");
    let dashboard = app.join("src").join("dashboard");
    
    let dest = if assets.exists() || frontend.exists() || dashboard.exists() {
        app.join("assets").join("shared-assets").join("styles")
    } else {
        println!("skip (no UI tree): {}", app.display());
        return;
    };
    
    fs::create_dir_all(&dest).unwrap();
    
    // Instead of using rsync (which defeats the purpose of removing bash dependencies),
    // we use a Rust directory copy implementation.
    copy_dir_all(styles_src, &dest).unwrap();
    println!("synced -> {}", dest.display());
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    
    // Remove all existing files in dst to simulate --delete
    for entry in fs::read_dir(dst)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            fs::remove_dir_all(entry.path())?;
        } else {
            fs::remove_file(entry.path())?;
        }
    }
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if entry.file_name() == ".git" {
            continue;
        }
        let dest_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dest_path)?;
        } else {
            fs::copy(entry.path(), &dest_path)?;
        }
    }
    Ok(())
}
