use std::fs;

use crate::utils::{get_apps, rg_fe, count_fe};

pub fn check_appshell_links(apps: Vec<String>) -> bool {
    let mut fail = false;
    for app in get_apps(apps) {
        let name = app.file_name().unwrap().to_string_lossy().to_string();
        let mut local_fail = false;

        let page = app.join("src/dashboard/dashboard_page.rs");
        let js = app.join("src/dashboard/scripts_actions.rs");
        let fe = app.join("frontend/src");

        if name == "statesync" || (!fe.exists() && app.join("src/dashboard").exists()) {
            if !page.exists() {
                println!("FAIL statesync: missing dashboard_page.rs");
                fail = true;
                continue;
            }
            let page_content = fs::read_to_string(&page).unwrap_or_default();
            if !page_content.contains("github.com/studio2201/statesync") {
                println!("FAIL statesync: header title missing GH repo link");
                local_fail = true;
            }
            if js.exists() {
                let js_content = fs::read_to_string(&js).unwrap_or_default();
                if !js_content.contains("releases/tag/v") {
                    println!("FAIL statesync: version footer missing release tag link");
                    local_fail = true;
                }
            }
            if !local_fail {
                println!("ok   statesync (Maud title + version release link)");
            } else {
                fail = true;
            }
            continue;
        }

        if fe.exists() {
            if !rg_fe(&fe, "AppShell") {
                println!("FAIL {}: no AppShell usage", name);
                local_fail = true;
            }

            let repo_pat = format!("repo: Some\\(\"{}\"", name);
            let repo_hits = count_fe(&fe, &repo_pat);
            if repo_hits < 2 {
                println!("FAIL {}: expected Header+Footer repo: Some(\"{}\") (found {})", name, name, repo_hits);
                local_fail = true;
            }

            if !rg_fe(&fe, "show_version:") {
                println!("FAIL {}: missing show_version in FooterProps", name);
                local_fail = true;
            }

            if !rg_fe(&fe, r#"version:\s*(Some\(|self\.|env!|version)"#) && !rg_fe(&fe, r#"^\s+version,"#) {
                println!("FAIL {}: FooterProps missing version value", name);
                local_fail = true;
            }

            if !local_fail {
                println!("ok   {} (AppShell, repo×{})", name, repo_hits);
            } else {
                fail = true;
            }
        } else {
            println!("skip {} (no web UI)", name);
        }
    }
    fail
}
