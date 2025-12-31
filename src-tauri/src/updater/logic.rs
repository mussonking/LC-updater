use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use zip::ZipArchive;
use reqwest;

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateManifest {
    pub version: String,
    pub download_url: String,
}

pub async fn download_and_install(app_dir: PathBuf, manifest: UpdateManifest) -> Result<(), String> {
    let client = reqwest::Client::new();
    let zip_res = client.get(&manifest.download_url).send().await.map_err(|e| e.to_string())?;
    let bytes = zip_res.bytes().await.map_err(|e| e.to_string())?;
    
    let reader = std::io::Cursor::new(bytes);
    let mut archive = ZipArchive::new(reader).map_err(|e| e.to_string())?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = match file.enclosed_name() {
            Some(path) => app_dir.join(path),
            None => continue,
        };
        
        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).map_err(|e| e.to_string())?;
                }
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }
    }
    
    let version_file = app_dir.join("version.txt");
    fs::write(version_file, &manifest.version).map_err(|e| e.to_string())?;
    
    Ok(())
}

pub fn get_local_version(app_dir: &PathBuf) -> String {
    let version_file = app_dir.join("version.txt");
    if version_file.exists() {
        fs::read_to_string(&version_file).unwrap_or_default()
    } else {
        String::new()
    }
}
