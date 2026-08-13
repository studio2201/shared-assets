use std::fs;
use regex::Regex;

use crate::utils::{get_apps, get_md5, RED_CHECK_MD5};

pub fn check_app_icons(apps: Vec<String>) -> bool {
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

pub fn ensure_app_icons(apps: Vec<String>) -> bool {
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
