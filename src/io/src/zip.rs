use std::fs::File;
use std::io::prelude::*;
use std::iter::Iterator;
use std::path::Path;

use harana_common::walkdir::WalkDir;
use harana_common::zip::result::{ZipError, ZipResult};
use harana_common::zip::write::{FileOptions, FullFileOptions, SimpleFileOptions};
use harana_common::zip::{CompressionMethod, ZipWriter};


pub fn zip_directory(from_path: &Path, to_path: &Path, method: Option<&str>, _strip_prefix: bool) -> ZipResult<()> {
    if !from_path.is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let method = match method {
        Some("deflate") => CompressionMethod::Deflated,
        Some("bzip2") => CompressionMethod::Bzip2,
        Some("none") => CompressionMethod::Stored,
        _ => CompressionMethod::Deflated
    };

    let zip_file = File::create(&Path::new(to_path)).unwrap();
    let mut zip = ZipWriter::new(zip_file);
    let options = FullFileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    let walkdir = WalkDir::new(from_path);
    let it = walkdir.into_iter().filter_map(|e| e.ok());

    for entry in it {
        let path = entry.path();

        // FIXME
        let prefix = "";
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {:?} as {:?} ...", path, name);
            let options = FullFileOptions::default()
                .compression_method(method)
                .unix_permissions(0o755);
            #[allow(deprecated)]
                zip.start_file_from_path(path, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if name.as_os_str().len() != 0 {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {:?} as {:?} ...", path, name);
            let options = SimpleFileOptions::default();
            #[allow(deprecated)]
                zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}
