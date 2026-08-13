use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use md5::{Md5, Digest};
use regex::Regex;
use ignore::WalkBuilder;

const RED_CHECK_MD5: &str = "b38828f8820f79d0865ef3d530567fc0";

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

fn get_apps(apps_args: Vec<String>) -> Vec<PathBuf> {
    let mut apps = Vec::new();
    if apps_args.is_empty() {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
        let parent = manifest.parent().unwrap().parent().unwrap().parent().unwrap();
        if let Ok(entries) = fs::read_dir(parent) {
            let mut paths: Vec<_> = entries.filter_map(Result::ok).map(|e| e.path()).collect();
            paths.sort();
            for path in paths {
                if path.is_dir() {
                    let name = path.file_name().unwrap().to_string_lossy();
                    if name == "shared-assets" || name == "studio2201.github.io" || name == ".github" {
                        continue;
                    }
                    apps.push(path);
                }
            }
        }
    } else {
        for app in apps_args {
            if let Ok(p) = Path::new(&app).canonicalize() {
                apps.push(p);
            } else {
                apps.push(PathBuf::from(app));
            }
        }
    }
    apps
}

fn get_md5(path: &Path) -> Option<String> {
    let content = fs::read(path).ok()?;
    let mut hasher = Md5::new();
    hasher.update(&content);
    let hash = hasher.finalize();
    Some(hash.iter().map(|b| format!("{:02x}", b)).collect::<String>())
}

fn check_app_icons(apps: Vec<String>) -> bool {
    let mut fail = false;
    for app in get_apps(apps) {
        let name = app.file_name().unwrap().to_string_lossy().to_string();
        
        let assets = app.join("assets");
        let icon = assets.join("icon.png");
        let svg = assets.join("favicon.svg");
        let index = app.join("frontend").join("index.html");

        if !app.join("frontend").exists() && !app.join("src").join("dashboard").exists() {
            println!("skip {} (no web UI)", name);
            continue;
        }

        let mut fav = None;
        for cand in &["favicon.png", "favicon.jpg", "icon.png"] {
            let p = assets.join(cand);
            if p.exists() {
                fav = Some(p);
                break;
            }
        }

        if fav.is_none() {
            println!("FAIL {}: missing assets/favicon.png (or .jpg / icon.png)", name);
            fail = true;
            continue;
        }
        
        let fav = fav.unwrap();
        let fav_name = fav.file_name().unwrap().to_string_lossy();

        if icon.exists() && fav_name.ends_with(".png") {
            if let (Some(hf), Some(hi)) = (get_md5(&fav), get_md5(&icon)) {
                if hf != hi {
                    println!("WARN {}: favicon.png != icon.png (tab icon should match brand icon)", name);
                }
            }
        }

        if svg.exists() {
            if let Some(hs) = get_md5(&svg) {
                if hs == RED_CHECK_MD5 {
                    println!("FAIL {}: assets/favicon.svg is the legacy red-check (todo) icon", name);
                    fail = true;
                }
            }
        }

        if index.exists() {
            if let Ok(content) = fs::read_to_string(&index) {
                let svg_regex1 = Regex::new(r#"type="image/svg\+xml".*favicon|favicon\.svg.*rel="icon"|rel="icon".*svg"#).unwrap();
                if svg_regex1.is_match(&content) {
                    let svg_regex2 = Regex::new(r#"rel="icon"[^>]*image/svg\+xml|type="image/svg\+xml"[^>]*rel="icon""#).unwrap();
                    if svg_regex2.is_match(&content) {
                        let mut first_svg = false;
                        for line in content.lines() {
                            if line.contains(r#"rel="icon""#) {
                                if line.to_lowercase().contains("svg") {
                                    first_svg = true;
                                }
                                break;
                            }
                        }
                        if first_svg {
                            println!("FAIL {}: index.html prefers SVG favicon (browsers will ignore PNG brand icon)", name);
                            fail = true;
                        }
                    }
                }

                if !content.contains("favicon.png") {
                    println!("FAIL {}: index.html does not reference favicon.png", name);
                    fail = true;
                }
            }
        }

        if !fail {
            println!("ok   {}", name);
        }
    }
    fail
}

fn ensure_app_icons(apps: Vec<String>) -> bool {
    let mut fail = false;
    for app in get_apps(apps) {
        let name = app.file_name().unwrap().to_string_lossy().to_string();
        let mut changed = false;
        
        let assets = app.join("assets");
        let icon = assets.join("icon.png");
        let fav = assets.join("favicon.png");
        let svg = assets.join("favicon.svg");
        let index = app.join("frontend").join("index.html");

        if !app.join("frontend").exists() && !app.join("src").join("dashboard").exists() {
            println!("skip {} (no web UI)", name);
            continue;
        }

        if !assets.exists() {
            println!("skip {} (no assets/)", name);
            continue;
        }

        if icon.exists() {
            let same = fav.exists() && fs::read(&icon).unwrap_or_default() == fs::read(&fav).unwrap_or_default();
            if !same {
                if let Err(e) = fs::copy(&icon, &fav) {
                    println!("FAIL {}: failed to copy icon.png to favicon.png: {}", name, e);
                } else {
                    println!("  {}: synced assets/favicon.png <- icon.png", name);
                    changed = true;
                }
            }
        } else if !fav.exists() && !assets.join("favicon.jpg").exists() {
            println!("WARN {}: missing assets/icon.png and assets/favicon.png", name);
        }

        if svg.exists() {
            if let Some(hs) = get_md5(&svg) {
                if hs == RED_CHECK_MD5 {
                    fs::remove_file(&svg).unwrap_or_default();
                    println!("  {}: removed legacy red-check favicon.svg", name);
                    changed = true;
                }
            }
        }

        if index.exists() {
            let original_content = fs::read_to_string(&index).unwrap_or_default();
            let mut content = original_content.clone();
            
            let svg_re = Regex::new(r#"favicon\.svg|type="image/svg\+xml".*icon|rel="icon"[^>]*svg"#).unwrap();
            if svg_re.is_match(&content) {
                let drop_re = Regex::new(r#"favicon\.svg|type="image/svg\+xml"[^>]*rel="icon"|rel="icon"[^>]*type="image/svg\+xml"|rel="alternate icon""#).unwrap();
                let mut new_lines = Vec::new();
                for line in content.split('\n') {
                    if !drop_re.is_match(line) {
                        new_lines.push(line);
                    }
                }
                
                if new_lines.is_empty() && !content.is_empty() {
                    println!("FAIL {}: refused to empty index.html", name);
                    fail = true;
                    continue;
                }
                content = new_lines.join("\n");
            }

            let copy_re = Regex::new(r#"data-trunk rel="copy-file"[^>]*favicon\.png|copy-file" href="\.\./assets/favicon\.png""#).unwrap();
            if !copy_re.is_match(&content) {
                if content.contains("data-trunk") {
                    if content.contains(r#"rel="icon""#) {
                        let mut new_lines = Vec::new();
                        let mut done = false;
                        for line in content.split('\n') {
                            if line.contains(r#"rel="icon""#) && !done {
                                new_lines.push("    <link data-trunk rel=\"copy-file\" href=\"../assets/favicon.png\" />");
                                done = true;
                            }
                            new_lines.push(line);
                        }
                        content = new_lines.join("\n");
                    }
                }
            }

            let rel_re1 = Regex::new(r#"rel="icon"[^>]*favicon\.png|href="favicon\.png"[^>]*rel="icon""#).unwrap();
            let rel_re2 = Regex::new(r#"type="image/png"[^>]*href="favicon\.png""#).unwrap();
            if !rel_re1.is_match(&content) && !rel_re2.is_match(&content) {
                if content.contains("</title>") {
                    content = content.replace("</title>", "</title>\n    <link rel=\"icon\" type=\"image/png\" href=\"favicon.png\" />\n    <link rel=\"apple-touch-icon\" href=\"favicon.png\" />");
                } else if content.contains("</head>") {
                    content = content.replace("</head>", "    <link rel=\"icon\" type=\"image/png\" href=\"favicon.png\" />\n    <link rel=\"apple-touch-icon\" href=\"favicon.png\" />\n</head>");
                }
            }

            if !content.contains("apple-touch-icon") {
                let re = Regex::new(r#"rel="icon" type="image/png" href="favicon\.png" */>"#).unwrap();
                content = re.replace(&content, "$0\n    <link rel=\"apple-touch-icon\" href=\"favicon.png\" />").to_string();
            }

            if content != original_content {
                fs::write(&index, &content).unwrap_or_default();
                println!("  {}: updated frontend/index.html icon links (PNG primary)", name);
                changed = true;
            }
        }

        if !changed {
            println!("ok   {} (already compliant)", name);
        } else {
            println!("fixed {}", name);
        }
    }
    fail
}

fn rg_fe(dir: &Path, pattern: &str) -> bool {
    let re = Regex::new(&format!("(?m){}", pattern)).unwrap();
    let walker = WalkBuilder::new(dir).build();

    for result in walker {
        if let Ok(entry) = result {
            if entry.file_type().map_or(false, |ft| ft.is_file()) && entry.path().extension().map_or(false, |e| e == "rs") {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if re.is_match(&content) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn count_fe(dir: &Path, pattern: &str) -> usize {
    let re = Regex::new(&format!("(?m){}", pattern)).unwrap();
    let walker = WalkBuilder::new(dir).build();

    let mut count = 0;
    for result in walker {
        if let Ok(entry) = result {
            if entry.file_type().map_or(false, |ft| ft.is_file()) && entry.path().extension().map_or(false, |e| e == "rs") {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    count += re.find_iter(&content).count();
                }
            }
        }
    }
    count
}

fn check_appshell_links(apps: Vec<String>) -> bool {
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

fn sync_web_ui(apps: Vec<String>) {
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

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    
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
