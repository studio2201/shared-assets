use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use md5::{Md5, Digest};
use regex::Regex;
use ignore::WalkBuilder;

pub const RED_CHECK_MD5: &str = "b38828f8820f79d0865ef3d530567fc0";

pub fn get_apps(apps_args: Vec<String>) -> Vec<PathBuf> {
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

pub fn get_md5(path: &Path) -> Option<String> {
    let content = fs::read(path).ok()?;
    let mut hasher = Md5::new();
    hasher.update(&content);
    let hash = hasher.finalize();
    Some(hash.iter().map(|b| format!("{:02x}", b)).collect::<String>())
}

pub fn rg_fe(dir: &Path, pattern: &str) -> bool {
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

pub fn count_fe(dir: &Path, pattern: &str) -> usize {
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

pub fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
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
