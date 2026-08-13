use std::env;

mod utils;
mod icons;
mod links;
mod sync;

use icons::{check_app_icons, ensure_app_icons};
use links::check_appshell_links;
use sync::sync_web_ui;

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("sync-web-ui") => {
            let apps: Vec<String> = args.collect();
            sync_web_ui(apps);
        }
        Some("check-app-icons") => {
            let apps: Vec<String> = args.collect();
            let fail = check_app_icons(apps);
            if fail { std::process::exit(1); }
        }
        Some("ensure-app-icons") => {
            let apps: Vec<String> = args.collect();
            let fail = ensure_app_icons(apps);
            if fail { std::process::exit(1); }
        }
        Some("check-appshell-links") => {
            let apps: Vec<String> = args.collect();
            let fail = check_appshell_links(apps);
            if fail { std::process::exit(1); }
        }
        Some(cmd) => {
            eprintln!("Unknown command: {}", cmd);
            std::process::exit(1);
        }
        None => {
            eprintln!("Usage: cargo xtask <command>");
            eprintln!("Commands:");
            eprintln!("  sync-web-ui           Sync shared-assets web UI styles into companion apps");
            eprintln!("  check-app-icons       Validate companion apps use service-specific tab icons");
            eprintln!("  ensure-app-icons      Ensure companion apps use the service-specific brand icon");
            eprintln!("  check-appshell-links  Assert companion apps wire AppShell header/footer GH links");
            std::process::exit(1);
        }
    }
}
