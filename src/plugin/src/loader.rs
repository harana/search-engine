use std::{fs, io};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use harana_common::serde_yml;
use harana_common::anyhow::{Context, Result};
use harana_common::tokio::io::copy;
use harana_common::uuid::Uuid;
use harana_common::zip::ZipArchive;
use crate::plugin::Plugin;

struct Loader {}

impl Loader {

    fn download_and_extract(url: String, base_dir: &Path) -> Result<PathBuf> {

        // Generate a random subdirectory name
        let random_dir_name = Uuid::new_v4().to_string();
        let extract_dir = base_dir.join(random_dir_name);

        // Create the subdirectory
        fs::create_dir_all(&extract_dir)
            .with_context(|| format!("Failed to create directory at {:?}", extract_dir))?;

        // Download the zip file
        let mut response = reqwest::blocking::get(&url)
            .with_context(|| format!("Failed to GET from {}", url))?;
        let mut zip_content = Vec::new();
        response.copy_to(&mut zip_content)
            .with_context(|| format!("Failed to read content from {}", url))?;

        // Create a cursor to read the zip content
        let reader = io::Cursor::new(zip_content);

        // Extract the zip file
        let mut archive = ZipArchive::new(reader)
            .with_context(|| "Failed to read zip archive")?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .with_context(|| format!("Failed to read file {} from zip", i))?;
            let outpath = extract_dir.join(file.mangled_name());

            if (&*file.name()).ends_with('/') {
                fs::create_dir_all(&outpath)
                    .with_context(|| format!("Failed to create directory at {:?}", outpath))?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)
                            .with_context(|| format!("Failed to create directory at {:?}", p))?;
                    }
                }
                let mut outfile = File::create(&outpath)
                    .with_context(|| format!("Failed to create file at {:?}", outpath))?;
                copy(&mut file, &mut outfile)
                    .with_context(|| format!("Failed to write file at {:?}", outpath))?;
            }
        }

        Ok(extract_dir)
    }

    fn load_plugin(base_dir: &Path) -> Result<Plugin> {
        let file_path = base_dir.join("plugin.yaml");

        let mut file = File::open(&file_path)
            .with_context(|| format!("Failed to open plugin.yaml at {:?}", file_path))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .with_context(|| format!("Failed to read plugin.yaml at {:?}", file_path))?;

        let plugin: Plugin = serde_yml::from_str(&contents)
            .with_context(|| format!("Failed to deserialize plugin.yaml at {:?}", file_path))?;

        Ok(plugin)
    }
}