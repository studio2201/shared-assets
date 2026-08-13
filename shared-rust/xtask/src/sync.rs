use std::env;
use std::fs;
use std::path::Path;

use crate::utils::{get_apps, copy_dir_all};

pub fn sync_web_ui(apps: Vec<String>) {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root = manifest.parent().unwrap().parent().unwrap();
    let styles_src = root.join("styles");
    
    let apps_to_sync = get_apps(apps);
    for app in apps_to_sync {
        sync_one(&app, &styles_src);
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
    
    if let Err(e) = copy_dir_all(styles_src, &dest) {
        println!("Failed to sync {}: {}", dest.display(), e);
    } else {
        println!("synced -> {}", dest.display());
    }
}
