use std::fmt::{Debug, Formatter};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use harana_common::bincode::Options;
use harana_common::sled;
use harana_common::tantivy::directory::{FileHandle, FileSlice, MmapDirectory, WatchCallback, WatchHandle, WritePtr};
use crate::encrypted_dir::{EncryptedMmapDirectory, PBKDF_COUNT};
use harana_common::tantivy::Directory;
use harana_common::tantivy::directory::error::{DeleteError, OpenReadError, OpenWriteError};

use harana_common::anyhow::{Error, Result};
use harana_common::serde::Serialize;
use harana_common::log::debug;
use crate::cr32_hash;

static WATCHED_MANAGED_FILE: &str = ".managed.json";
static WATCHED_META_FILE: &str = "meta.json";

/// The sub-directory where any lnx-metadata is stored for the index.
static METASTORE_INNER_ROOT: &str = "metadata";

/// The sub-directory where Tantivy's data is stored.
///
/// This maintains compatibility with any Tantivy directory.
static DATA_INNER_ROOT: &str = "data";

#[derive(Debug)]
pub enum OpenType {
    Dir(PathBuf, String),
    TempFile,
}

impl OpenType {
    pub fn exists(&self) -> bool {
        match self {
            Self::TempFile => true,
            Self::Dir(path, _) => path.exists(),
        }
    }
}

/// A wrapper around a EncryptedMmapDirectory but using sled to provide
/// the atomic write/read interface.
///
/// The only difference is Tantivy's special `meta.json` and `managed.json` is
/// ignored and kept mount to the mmap directory.
/// This same logic is applied to watching files, only those aforementioned files
/// are watched.
#[derive(Clone)]
pub struct SledBackedDirectory {
    inner: Box<dyn Directory>,
    conn: sled::Db,
}

impl SledBackedDirectory {
    /// Creates or opens the given path.
    ///
    /// If OpenType::Dir(p) is set the system will ensure the directories are
    /// created and ensured.
    ///
    /// If OpenType::TempFile is set the system will create a temporary structure,
    /// normally for testing.
    pub fn new_with_root(path: &OpenType) -> Result<Self> {
        let (conn, inner): (sled::Db, Box<dyn Directory>) = match path {
            OpenType::Dir(path, passphrase) => {
                std::fs::create_dir_all(path)?;
                std::fs::create_dir_all(path.join(DATA_INNER_ROOT))?;

                (
                    sled::Config::new()
                        .mode(sled::Mode::HighThroughput)
                        .path(path.join(METASTORE_INNER_ROOT))
                        .open()?,
                    Box::new(EncryptedMmapDirectory::open_or_create(path.join(DATA_INNER_ROOT), passphrase.as_str(), PBKDF_COUNT)?),
                )
            },
            OpenType::TempFile => (
                sled::Config::new()
                    .mode(sled::Mode::HighThroughput)
                    .temporary(true)
                    .open()?,
                Box::new(MmapDirectory::create_from_tempdir()?),
            ),
        };

        Ok(Self { inner, conn })
    }
}

impl Debug for SledBackedDirectory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SledBackedDirectory")
    }
}

impl Directory for SledBackedDirectory {
    fn get_file_handle(
        &self,
        path: &Path,
    ) -> core::result::Result<Arc<dyn FileHandle>, OpenReadError> {
        self.inner.get_file_handle(path)
    }

    fn delete(&self, path: &Path) -> core::result::Result<(), DeleteError> {
        self.inner.delete(path)
    }

    fn exists(&self, path: &Path) -> core::result::Result<bool, OpenReadError> {
        self.inner.exists(path)
    }

    fn open_read(&self, path: &Path) -> std::result::Result<FileSlice, OpenReadError> {
        self.inner.open_read(path)
    }

    fn open_write(&self, path: &Path) -> core::result::Result<WritePtr, OpenWriteError> {
        self.inner.open_write(path)
    }

    // TODO add-back #[instrument(name = "directory-atomic-reader", level = "debug", skip(self))]
    fn atomic_read(&self, path: &Path) -> core::result::Result<Vec<u8>, OpenReadError> {
        // Special case handling for Tantivy's file watchlist.
        if let Some(name) = path.file_name() {
            if name == WATCHED_MANAGED_FILE || name == WATCHED_META_FILE {
                debug!("using inner atomic read due to special file {:?}", &name);
                return self.inner.atomic_read(path);
            }
        }

        debug!("using sled backed atomic read");
        let value = self.conn.get(cr32_hash(path).to_string())
            .map_err(|e| {
                match e {
                    sled::Error::CollectionNotFound(_) =>
                        OpenReadError::FileDoesNotExist(path.to_path_buf()),
                    sled::Error::Unsupported(_) =>
                        OpenReadError::IoError {
                            io_error: Arc::from(std::io::Error::new(
                                ErrorKind::InvalidData,
                                "Metastore has been used in a un-supported way",
                            )),
                            filepath: path.to_path_buf(),
                        },
                    sled::Error::ReportableBug(e) =>
                        panic!("Failed to perform operation due to unexpected error: {}, Please report this as a bug.", e),
                    sled::Error::Io(e) =>
                        OpenReadError::IoError { io_error: Arc::from(e), filepath: path.to_path_buf() },
                    sled::Error::Corruption { at, .. } =>
                        OpenReadError::IoError {
                            io_error: Arc::from(std::io::Error::new(
                                ErrorKind::InvalidData,
                                format!(
                                    "Data corruption has been detected within the metastore system, Possible Info: {:?}",
                                    at,
                                )
                            )),
                            filepath: path.to_path_buf(),
                        },
                    #[allow(unreachable_patterns)]
                    _ => unreachable!(),
                }
            })?;

        value
            .map(|v| v.to_vec())
            .ok_or_else(|| OpenReadError::FileDoesNotExist(path.to_path_buf()))
    }

    // TODO add-back #[instrument(name = "directory-atomic-writer", level = "debug", skip(self, data))]
    fn atomic_write(&self, path: &Path, data: &[u8]) -> std::io::Result<()> {
        // Special case handling for Tantivy's file watchlist.
        if let Some(name) = path.file_name() {
            if name == WATCHED_MANAGED_FILE || name == WATCHED_META_FILE {
                debug!("using inner atomic write due to special file {:?}", &name);
                return self.inner.atomic_write(path, data);
            }
        }

        debug!("using sled backed atomic write");
        let id = cr32_hash(path).to_string();
        self.conn.insert(id, data)?;
        self.conn.flush()?;

        Ok(())
    }

    fn sync_directory(&self) -> std::io::Result<()> {
        self.inner.sync_directory()
    }

    fn watch(&self, watch_callback: WatchCallback) -> harana_common::tantivy::Result<WatchHandle> {
        self.inner.watch(watch_callback)
    }
}

/// A wrapper around the SledBackedDirectory providing serializer and loading
/// interfaces.
///
/// This is mostly just a sugar wrapper.
#[derive(Clone)]
pub struct StorageBackend {
    conn: SledBackedDirectory,
}

impl StorageBackend {
    pub fn using_conn(conn: SledBackedDirectory) -> Self {
        Self { conn }
    }

    #[inline]
    pub fn conn(&self) -> &sled::Db {
        &self.conn.conn
    }

    pub fn store_structure<T: Serialize>(
        &self,
        keyspace: &str,
        value: &T,
    ) -> Result<()> {
        let data = harana_common::bincode::options().with_big_endian().serialize(value)?;

        self.conn.atomic_write(keyspace.as_ref(), &data)?;
        Ok(())
    }

    pub fn load_structure(&self, keyspace: &str) -> Result<Option<Vec<u8>>> {
        let compressed = match self.conn.atomic_read(keyspace.as_ref()) {
            Ok(data) => data,
            Err(OpenReadError::FileDoesNotExist(_)) => return Ok(None),
            Err(e) => return Err(Error::from(e)),
        };

        Ok(Some(compressed))
    }
}

impl Debug for StorageBackend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("StorageBackend")
    }
}

#[cfg(test)]
mod tests {
    use harana_common::anyhow::Context;
    use harana_common::bincode::Options;
    use super::*;

    #[test]
    fn test_loading_and_unloading() -> Result<()> {
        let test_structure = vec!["foo", "bar"];

        let dir = SledBackedDirectory::new_with_root(&OpenType::TempFile)?;
        let storage = StorageBackend::using_conn(dir);
        storage.store_structure("test", &test_structure)?;
        if let Some(buffer) = storage.load_structure("test")? {
            let test_res: Vec<&str> = harana_common::bincode::options()
                .with_big_endian()
                .deserialize(&buffer)
                .context("failed to deserialize base type")?;

            assert_eq!(test_structure, test_res);
        };

        Ok(())
    }
}