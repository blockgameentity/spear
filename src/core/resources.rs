use crate::config::spear;
use crate::constants::*;
use blake3;
use image;
use lazy_static::lazy_static;
use std::fs;

use std::sync::Mutex;
use winapi::um::memoryapi::{VirtualAlloc, VirtualProtect};
use winapi::um::winnt::{MEM_COMMIT, PAGE_READONLY, PAGE_READWRITE};

pub static REPLACEMENT_BG_DATA: &[u8] = include_bytes!("../../img/120.png");

lazy_static! {
    pub static ref REPLACEMENT_PIXELS: Vec<u8> = {
        let rgba = image::load_from_memory(REPLACEMENT_BG_DATA)
            .unwrap()
            .to_rgba8();
        rgba.into_raw()
            .chunks_exact(4)
            .flat_map(|chunk| [chunk[2], chunk[1], chunk[0], chunk[3]])
            .collect()
    };
}

pub static mut FAKE_BG_RESOURCE: *mut u8 = std::ptr::null_mut();

pub fn allocate_fake_resource() {
    unsafe {
        FAKE_BG_RESOURCE = VirtualAlloc(
            std::ptr::null_mut(),
            REPLACEMENT_BG_DATA.len(),
            MEM_COMMIT,
            PAGE_READWRITE,
        ) as *mut u8;
        if FAKE_BG_RESOURCE.is_null() {
            panic!("Failed to allocate fake resource");
        }
        std::ptr::copy_nonoverlapping(
            REPLACEMENT_BG_DATA.as_ptr(),
            FAKE_BG_RESOURCE,
            REPLACEMENT_BG_DATA.len(),
        );
        let mut old_protect = 0;
        VirtualProtect(
            FAKE_BG_RESOURCE as *mut _,
            REPLACEMENT_BG_DATA.len(),
            PAGE_READONLY,
            &mut old_protect,
        );
    }
}

lazy_static! {
    pub static ref PLAY_ICON_DATA: Mutex<Option<Vec<u8>>> = Mutex::new(None);
    pub static ref SETTINGS_ICON_DATA: Mutex<Option<Vec<u8>>> = Mutex::new(None);
    pub static ref FONT_DATA: Mutex<Option<Vec<u8>>> = Mutex::new(None);
}

#[allow(dead_code)]
pub fn analyze_data_cached(data: &[u8]) {
    if data.len() >= 4 {
        if &data[0..4] == b"\x89PNG" {
            let hash_str = blake3::hash(data).to_hex();
            let cache_dir = SPEAR_PATH.join("cache");
            fs::create_dir_all(&cache_dir).ok();
            if hash_str.as_str() == PLAY_ICON_HASH {
                *PLAY_ICON_DATA.lock().unwrap() = Some(data.to_vec());
                fs::write(cache_dir.join("play_icon.png"), data).ok();
            } else if hash_str.as_str() == SETTINGS_ICON_HASH {
                *SETTINGS_ICON_DATA.lock().unwrap() = Some(data.to_vec());
                fs::write(cache_dir.join("settings_icon.png"), data).ok();
            }
        } else if &data[0..4] == b"\x00\x01\x00\x00" || &data[0..4] == b"OTTO" {
            *FONT_DATA.lock().unwrap() = Some(data.to_vec());
            let cache_dir = SPEAR_PATH.join("cache");
            fs::create_dir_all(&cache_dir).ok();
            fs::write(cache_dir.join("font.ttf"), data).ok();
        }
    }
}

pub async fn peacock_download_release() -> Result<String, Box<dyn std::error::Error>> {
    use octocrab::Octocrab;
    use reqwest;
    use std::fs::{self, File};
    use zip::ZipArchive;

    let peacock_dir = SPEAR_PATH.join("peacock");
    fs::create_dir_all(&peacock_dir)?;

    let config = spear::load_spear_config();
    let mut repo_str = config.peacock_github_repo.clone();
    let slash_pos = repo_str.find('/').unwrap();
    let repo = repo_str.split_off(slash_pos + 1);
    let owner = repo_str;

    let octocrab = Octocrab::builder().build()?;
    let release = octocrab.repos(owner, repo).releases().get_latest().await?;
    log::info!("Latest release: {}", release.tag_name);

    std::fs::write(peacock_dir.join("version.txt"), &release.tag_name)?;

    for asset in &release.assets {
        if asset.name.to_lowercase().contains("linux") {
            log::info!("Skipping asset: {}", asset.name);
            continue;
        }
        log::info!("Downloading asset: {}", asset.name);
        let response = reqwest::get(asset.browser_download_url.clone()).await?;
        let bytes = response.bytes().await?;
        let mut file = File::create(peacock_dir.join(&asset.name))?;
        std::io::Write::write_all(&mut file, &bytes)?;
    }

    for entry in fs::read_dir(&peacock_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("zip") {
            log::info!("Extracting: {}", path.display());
            let file = File::open(&path)?;
            let mut archive = ZipArchive::new(file)?;
            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                let outpath = peacock_dir.join(file.name());
                if file.name().ends_with('/') {
                    fs::create_dir_all(&outpath)?;
                } else {
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            fs::create_dir_all(p)?;
                        }
                    }
                    let mut outfile = File::create(&outpath)?;
                    std::io::copy(&mut file, &mut outfile)?;
                }
            }
            fs::remove_file(&path)?;
        }

        for entry_result in fs::read_dir(&peacock_dir)? {
            let entry = entry_result?;
            if entry.file_type()?.is_dir() {
                if entry.file_name().to_str().unwrap().contains("Peacock-") {
                    let src_dir = entry.path();
                    fn copy_contents(
                        src: &std::path::Path,
                        dst: &std::path::Path,
                    ) -> std::io::Result<()> {
                        for entry in fs::read_dir(src)? {
                            let entry = entry?;
                            let src_path = entry.path();
                            let dst_path = dst.join(entry.file_name());
                            if entry.file_type()?.is_dir() {
                                fs::create_dir_all(&dst_path)?;
                                copy_contents(&src_path, &dst_path)?;
                            } else {
                                fs::copy(&src_path, &dst_path)?;
                            }
                        }
                        Ok(())
                    }
                    copy_contents(&src_dir, &peacock_dir)?;
                    fs::remove_dir_all(&src_dir)?;
                }
            }
        }
    }

    Ok(release.tag_name)
}
