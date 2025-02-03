use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

// use lfs_core::*;
use harana_common::serde::{Deserialize, Serialize};

// Given a byte buffer it will infer the mime type based on the magic number signature
pub fn infer_mimetype(buf: &[u8]) -> &str {
    infer::get(&buf).unwrap().mime_type()
}

#[derive(Deserialize, Serialize)]
pub struct FileObject {
    path: String,
    file_name: String,
    mime_type: Option<String>,
    extension: Option<String>,
    is_directory: bool,
    is_file: bool,
    accessed: SystemTime,
    created: SystemTime,
    modified: SystemTime,
    read_only: bool,
    size: u64,
}

#[derive(Deserialize, Serialize)]
pub struct Mount {
    id: String,
    disk: String,
    filesystem: String,
    filesystem_type: String,
    mount_point: String,
    available: u64,
    used: u64,
    size: u64,
    percentage: u8,
}

pub fn copy_directory<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        let src: PathBuf = working_path.components().skip(input_root).collect();

        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };

        if fs::metadata(&dest).is_err() {
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(file_name) => {
                        let dest_path = dest.join(file_name);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn list_directory<'a>(base: &Path, entries: &'a mut Vec<FileObject>, recursive: bool) -> &'a mut Vec<FileObject> {
    let reader = match fs::read_dir(&base) {
        Ok(x) => x,
        Err(_) => return entries,
    };

    for entry in reader {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();

        entries.push(
            FileObject {
                path: entry.path().to_str().unwrap().into(),
                file_name: String::from(entry.file_name().to_str().unwrap()),
                mime_type: None,
                extension: entry.path().extension().map(|x| String::from(x.to_str().unwrap())),
                is_directory: entry.metadata().unwrap().is_dir(),
                is_file: entry.metadata().unwrap().is_file(),
                accessed: entry.metadata().unwrap().accessed().unwrap(),
                created: entry.metadata().unwrap().created().unwrap(),
                modified: entry.metadata().unwrap().modified().unwrap(),
                read_only: entry.metadata().unwrap().permissions().readonly(),
                size: entry.metadata().unwrap().len(),
            }
        );

        if file_type.is_dir() {
            list_directory(entry.path().as_path(), entries, recursive);
        }
    }
    entries
}

// pub fn list_mounts() -> &[Mount] {
//     lfs_core::read_mounts()?
//         .filter(|m| m.stats.as_ref().size > 0)
//         .map(|m| Mount {
//             id: m.info.id,
//             disk: m.disk.as_ref().map_or("", |d| d.disk_type()),
//             filesystem: &m.info.fs,
//             filesystem_type: &m.info.fs_type,
//             mount_point: m.info.mount_point.to_string_lossy(),
//             available: m.stats.as_ref().available(),
//             used: m.stats.as_ref().used(),
//             size: m.stats.as_ref().size(),
//             percentage: 100.0 * m.stats.as_ref().use_share
//         })
//         .collect();
// }