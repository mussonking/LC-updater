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
    println!("[Logic] Building reqwest client for download...");
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true) // Allow self-signed/dev certificates just in case
        .build()
        .map_err(|e| {
            println!("[Logic] Error building client: {}", e);
            e.to_string()
        })?;
        
    let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
    let download_url = if manifest.download_url.contains('?') {
        format!("{}&cb={}", manifest.download_url, ts)
    } else {
        format!("{}?cb={}", manifest.download_url, ts)
    };
    
    println!("[Logic] Sending GET request to {}...", download_url);
    let zip_res = client.get(&download_url).send().await.map_err(|e| {
        println!("[Logic] Request failed: {}", e);
        e.to_string()
    })?;
    
    let status = zip_res.status();
    println!("[Logic] HTTP Status for zip download: {}", status);
    if !status.is_success() {
        return Err(format!("Erreur de téléchargement: HTTP {}. L'URL de l'archive ZIP ({}) ne retourne pas de fichier valide.", status, manifest.download_url));
    }
    
    println!("[Logic] Reading bytes from response...");
    let bytes = zip_res.bytes().await.map_err(|e| {
        println!("[Logic] Error reading bytes: {}", e);
        e.to_string()
    })?;
    println!("[Logic] Downloaded {} bytes of zip archive.", bytes.len());
    
    let reader = std::io::Cursor::new(bytes);
    let mut archive = ZipArchive::new(reader).map_err(|e| {
        println!("[Logic] ZipArchive decode error: {}", e);
        e.to_string()
    })?;
    
    let tmp_app_dir = app_dir.with_extension("tmp_update");
    if tmp_app_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_app_dir);
    }
    fs::create_dir_all(&tmp_app_dir).map_err(|e| {
        println!("[Logic] Error creating tmp dir: {}", e);
        e.to_string()
    })?;

    println!("[Logic] Zip archive contains {} files. Extracting to {:?}...", archive.len(), tmp_app_dir);
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| {
             println!("[Logic] Error reading file index {}: {}", i, e);
             e.to_string()
        })?;
        let outpath = match file.enclosed_name() {
            Some(path) => tmp_app_dir.join(path),
            None => {
                println!("[Logic] Skipping file at index {} due to unsafe enclosed name", i);
                continue;
            }
        };
        
        if file.name().ends_with('/') {
            // println!("[Logic] Creating directory: {:?}", outpath);
            fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).map_err(|e| e.to_string())?;
                }
            }
            // println!("[Logic] Extracting file: {:?}", outpath);
            let mut outfile = fs::File::create(&outpath).map_err(|e| {
                println!("[Logic] Failed to create extracted file {:?}: {}", outpath, e);
                e.to_string()
            })?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| {
                println!("[Logic] Failed to write extracted file {:?}: {}", outpath, e);
                e.to_string()
            })?;
        }
    }
    
    println!("[Logic] Extraction to tmp dir successful. Swapping directories...");
    
    // Attempt to move the old active directory out of the way
    let old_app_dir = app_dir.with_extension("old");
    if old_app_dir.exists() {
        let _ = fs::remove_dir_all(&old_app_dir); // Best effort cleanup
    }
    
    if app_dir.exists() {
        fs::rename(&app_dir, &old_app_dir).map_err(|e| {
            println!("[Logic] Error renaming active app_dir to old: {}", e);
            format!("Failed to move old version: {}", e)
        })?;
    }
    
    // Rename the new tmp directory to the active directory
    fs::rename(&tmp_app_dir, &app_dir).map_err(|e| {
        println!("[Logic] Error renaming tmp app_dir to active: {}", e);
        format!("Failed to apply new version: {}", e)
    })?;
    
    let version_file = app_dir.join("version.txt");
    println!("[Logic] Writing version {} to {:?}", manifest.version, version_file);
    fs::write(&version_file, &manifest.version).map_err(|e| {
        println!("[Logic] Error writing version file: {}", e);
        e.to_string()
    })?;
    
    println!("[Logic] Extracted {} files successfully and swapped active directory!", archive.len());
    Ok(())
}

pub fn get_local_version(app_dir: &PathBuf) -> String {
    let version_file = app_dir.join("version.txt");
    if version_file.exists() {
        fs::read_to_string(&version_file).unwrap_or_default().trim().to_string()
    } else {
        String::new()
    }
}
