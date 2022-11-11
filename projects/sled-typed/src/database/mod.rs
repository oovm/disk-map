use std::{
    fmt::{Debug, Formatter},
    path::Path,
};

use sled::{Config, Db};

use crate::{DiskMap, Result};

mod display;
mod iter;

/// # Arguments
///
/// * `path`:
///
/// returns: Result<Database, DiskMapError>
///
/// # Examples
///
/// ```
/// use sled_typed::Database;
/// ```
pub struct Database {
    inner: Db,
}

impl Drop for Database {
    fn drop(&mut self) {
        self.inner.flush().ok();
    }
}

impl Database {
    /// # Arguments
    ///
    /// * `path`:
    ///
    /// returns: Result<Database, DiskMapError>
    ///
    /// # Examples
    ///
    /// ```
    /// use sled_typed::Database;
    /// ```
    pub fn open(path: &Path) -> Result<Self> {
        let database = Config::default() //
            .use_compression(true)
            .path(path)
            .flush_every_ms(Some(1000))
            .open()?;
        Ok(Self { inner: database })
    }

    /// Open or create a typed document
    ///
    /// # Arguments
    ///
    /// * `name`:
    ///
    /// returns: Result<DiskMap<K, V>, DiskMapError>
    ///
    /// # Examples
    ///
    /// ```
    /// use sled_typed::Database;
    /// ```
    pub fn document<K, V>(&self, name: &str) -> Result<DiskMap<K, V>> {
        let tree = self.inner.open_tree(name)?;
        Ok(DiskMap::from(tree))
    }

    /// Drop some named [`DiskMap`]
    ///
    /// # Arguments
    ///
    /// * `name`:
    ///
    /// returns: bool
    ///
    /// # Examples
    ///
    /// ```
    /// use sled_typed::Database;
    /// ```
    pub fn drop(&self, name: &str) -> bool {
        self.inner.drop_tree(name).is_ok()
    }
    /// List all [`DiskMap`]
    pub fn list(&self) -> Vec<String> {
        self.inner //
            .tree_names()
            .into_iter()
            .map(|f| unsafe { String::from_utf8_unchecked(f.to_vec()) })
            .collect()
    }
    /// Returns the on-disk size of the storage files for this database.
    pub fn size_on_disk(&self) -> Result<u64> {
        Ok(self.inner.size_on_disk()?)
    }
    /// Asynchronously flushes all dirty IO buffers and calls fsync.
    ///
    /// If this succeeds, it is guaranteed that all previous writes will be recovered if the system crashes.
    ///
    /// Returns the number of bytes flushed during this call.
    ///
    /// Flushing can take quite a lot of time, and you should measure the performance impact of using it on realistic sustained workloads running on realistic hardware.
    pub async fn flush(&self) -> Result<usize> {
        Ok(self.inner.flush_async().await?)
    }
}
