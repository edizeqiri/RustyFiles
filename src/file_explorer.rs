use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use log::debug;
use walkdir::WalkDir;
extern crate winapi;

use winapi::um::fileapi::{GetLogicalDrives, GetDriveTypeW};
use winapi::um::winbase::{DRIVE_FIXED, DRIVE_REMOVABLE};
// TODO: Adjust this struct as it is trash right now
pub struct FileExplorer {
    root_dir: PathBuf,
}

impl FileExplorer{
    pub fn new(root_dir: &Path) -> FileExplorer {
        FileExplorer {
            root_dir: PathBuf::from(root_dir),
        }
    }
    
    // get a list of all files from the root directory
    fn list_files_recursive(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                println!("{}", path.display());
            }
        }
        Ok(())
    }
    
    // UNOPTIMIZED: get the size of the folder
    pub fn get_folder_size(path: &Path) -> Result<u64, Box<dyn std::error::Error>> {
        let mut size = 0;
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let metadata = entry.metadata()?;
            if metadata.is_file() {
                size += metadata.len();
            }
        }
        Ok(size)
    }

    pub fn display_size(size: u64) {
        let bytes = size / 1024;

        match bytes {
            bytes if bytes < 1024 => println!("Folder size: {:.2} KB", bytes),
            bytes if bytes < 1024 * 1024 => println!("Folder size: {:.2} MB", bytes as f64 / 1024.0),
            bytes if bytes < 1024 * 1024 * 1024 => println!("Folder size: {:.2} GB", bytes as f64 / 1024.0 / 1024.0),
            _ => println!("Folder size: {:.2} TB", bytes as f64 / 1024.0 / 1024.0 / 1024.0),
        }
    }
    
    // get the size of the root folder
    pub fn get_folder_size_threaded(wurzel: &Path) -> Result<u64, Box<dyn std::error::Error>> {
        let root_dir = wurzel;
        let mut stack: Vec<PathBuf> = Vec::new();

        for ding in WalkDir::new(root_dir).max_depth(1) {
            stack.push(PathBuf::from(ding.as_ref().unwrap().path().display().to_string()));
            debug!("{}", ding.unwrap().path().display());
        }
        let mut sizes: Vec<u64> = Vec::new();
        while !stack.is_empty() {
            let path = stack.pop().unwrap();
            if path.is_dir() {
                if path != PathBuf::from(root_dir) {
                    let thread_handle = thread::spawn(move || {
                        debug!("{}", path.display());
                        let tmp: u64 = FileExplorer::get_folder_size(&path).unwrap();
                        debug!("{}",tmp);
                        tmp
                    });
                    sizes.push(thread_handle.join().unwrap());
                } else {
                    continue;
                }
            }
        }

        let mut result = 0;
        for size in sizes {
            result += size;
        }
        debug!("{}",result);
        Ok(result)
    }

    pub fn get_top_folders() -> Vec<String> {
        let mut top_folders = Vec::new();
        top_folders
    }

    pub fn get_all_drive_letters() -> Vec<char> {
        let mut drives = vec![];
        unsafe {
            let logical_drives = GetLogicalDrives();
            if logical_drives == 0 {
                println!("Error getting logical drives.");
                return drives;
            }

            for drive_index in 0..26 {
                if (logical_drives & (1 << drive_index)) != 0 {
                    let drive_letter = (b'A' + drive_index as u8) as char;
                    drives.push(drive_letter);
                }
            }
            drives
        }
    }
}