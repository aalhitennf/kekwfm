use std::{fs::DirEntry, path::Path, result::Result};

use crate::KekwResult;

#[derive(Debug, Clone)]
pub struct DirectoryListingItem {
    pub filename: String,
    pub path: String,
    pub size_bytes: u64,
    pub extension: String,
    pub is_file: bool,
    pub is_dir: bool,
    pub is_symlink: bool,
    pub selected: bool,
}

impl TryFrom<DirEntry> for DirectoryListingItem {
    type Error = Box<dyn std::error::Error>;
    fn try_from(entry: DirEntry) -> Result<Self, Self::Error> {
        let filename = entry
            .file_name()
            .to_str()
            .map_or("Unknown".into(), ToString::to_string);
        let path = entry
            .path()
            .to_str()
            .map_or("Unknown".into(), ToString::to_string);
        let extension = if entry.path().is_dir() {
            "dir".into()
        } else {
            entry.path().extension().map_or("none".to_string(), |s| {
                s.to_str().map_or("error".into(), ToString::to_string)
            })
        };

        let metadata = entry.metadata()?;

        Ok(DirectoryListingItem {
            filename,
            path,
            size_bytes: metadata.len(),
            extension,
            is_file: metadata.is_file(),
            is_dir: metadata.is_dir(),
            is_symlink: metadata.is_symlink(),
            selected: false,
        })
    }
}

fn stringify_io_error(x: std::io::Error) -> String {
    format!("error code: {}", x)
}

#[derive(Debug, PartialEq)]
pub enum FileSorting {
    Alphabetical,
    Size,
    Extension,
    None,
}

#[derive(Debug, Default)]
pub struct DirectoryListing {
    pub items: Vec<DirectoryListingItem>,
    pub metadata: DirectoryListingMetaData,
    pub parent: Option<String>,
}

// impl Default for DirectoryListing {
//     fn default() -> Self {
//         DirectoryListing {
//             items: vec![],
//             metadata: DirectoryListingMetaData::default(),
//             parent: None,
//         }
//     }
// }

#[derive(Debug)]
pub struct DirectoryListingMetaData {
    files: u64,
    folders: u64,
    symlinks: u64,
    size_bytes: u64,
    size_mib: f64,
}

impl Default for DirectoryListingMetaData {
    fn default() -> Self {
        DirectoryListingMetaData {
            files: 0,
            folders: 0,
            symlinks: 0,
            size_bytes: 0,
            size_mib: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct ReadDirOptions {
    pub sorting: FileSorting,
    pub reverse: bool,
    pub folders_first: bool,
    pub include_hidden: bool,
}

impl Default for ReadDirOptions {
    fn default() -> Self {
        ReadDirOptions {
            sorting: FileSorting::Alphabetical,
            reverse: false,
            folders_first: true,
            include_hidden: false,
        }
    }
}

pub fn read_directory_listing<P: AsRef<Path> + Copy>(
    path: P,
    options: &ReadDirOptions,
) -> KekwResult<DirectoryListing> {
    // let path = Path::new(&path);

    if path.as_ref().is_dir() {
        let mut items = read_directory_listing_items(path, options.include_hidden)?;

        match options.sorting {
            FileSorting::Alphabetical => items.sort_by_key(|k| k.filename.clone()),
            FileSorting::Size => items.sort_by_key(|k| k.size_bytes),
            FileSorting::Extension => items.sort_by_key(|k| k.extension.clone()),
            FileSorting::None => (),
        }

        if options.folders_first {
            items.sort_by_key(|k| !k.is_dir)
        }

        if options.reverse {
            items.reverse();
        }

        let metadata = extract_directory_metadata(&items);

        let parent = path
            .as_ref()
            .parent()
            .map(|p| p.to_str().unwrap().to_string());

        Ok(DirectoryListing {
            items,
            metadata,
            parent,
        })
    } else {
        Err("Path is not directory".into())
    }
}

fn read_directory_listing_items<P: AsRef<Path>>(
    path: P,
    include_hidden: bool,
) -> Result<Vec<DirectoryListingItem>, String> {
    let read_dir = std::fs::read_dir(path).map_err(stringify_io_error)?;

    let items = if include_hidden {
        read_dir
            .flatten()
            .flat_map(DirectoryListingItem::try_from)
            .collect::<Vec<DirectoryListingItem>>()
    } else {
        read_dir
            .flatten()
            .flat_map(DirectoryListingItem::try_from)
            .filter(|d| !d.filename.starts_with('.'))
            .collect::<Vec<DirectoryListingItem>>()
    };

    Ok(items)
}

fn extract_directory_metadata(list: &[DirectoryListingItem]) -> DirectoryListingMetaData {
    let metadata = list
        .iter()
        .fold(DirectoryListingMetaData::default(), |mut a, b| {
            if b.is_dir {
                a.folders += 1;
            }
            if b.is_file {
                a.files += 1;
            }
            if b.is_symlink {
                a.symlinks += 1;
            }
            a.size_bytes += b.size_bytes;
            a.size_mib += b.size_bytes as f64 / 1024.0 / 1024.0;
            a
        });
    metadata
}

pub fn read_dir_entries<P: AsRef<Path>>(path: &P) -> KekwResult<Vec<DirEntry>> {
    let read_dir = std::fs::read_dir(path).map_err(stringify_io_error)?;
    let items = read_dir.flatten().collect::<Vec<DirEntry>>();
    Ok(items)
}
