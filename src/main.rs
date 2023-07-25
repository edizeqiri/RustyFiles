use std::path::Path;
use std::path::PathBuf;
use std::thread;
use walkdir::WalkDir;

fn main() {
    let root_dir = "C:\\Users\\edi"; // Replace with the directory you want to traverse
    get_folder_size_threaded(Path::new(root_dir)).unwrap();

    /* let bytes = get_folder_size(Path::new(root_dir)).unwrap();
    display_size(bytes);*/
}

fn list_files_recursive(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            println!("{}", path.display());
        }
    }
    Ok(())
}

fn get_folder_size(path: &Path) -> Result<u64, Box<dyn std::error::Error>> {
    let mut size = 0;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let metadata = entry.metadata()?;
        if metadata.is_file() {
            size += metadata.len();
        }
    }
    Ok(size)
}

fn display_size(size: u64) {
    let bytes = size / 1024;

    match bytes {
        bytes if bytes < 1024 => println!("Folder size: {:.2} KB", bytes),
        bytes if bytes < 1024 * 1024 => println!("Folder size: {:.2} MB", bytes as f64 / 1024.0),
        bytes if bytes < 1024 * 1024 * 1024 => println!("Folder size: {:.2} GB", bytes as f64 / 1024.0 / 1024.0),
        _ => println!("Folder size: {:.2} TB", bytes as f64 / 1024.0 / 1024.0 / 1024.0),
    }
}

fn get_folder_size_threaded(wurzel: &Path) -> Result<u64, Box<dyn std::error::Error>> {
    let root_dir = wurzel;
    let mut stack: Vec<PathBuf> = Vec::new();

    for ding in WalkDir::new(root_dir).max_depth(1) {
        stack.push(PathBuf::from(ding.as_ref().unwrap().path().display().to_string()));
        println!("{}", ding.unwrap().path().display());
    }
    let mut sizes: Vec<u64> = Vec::new();
    while !stack.is_empty() {
        let path = stack.pop().unwrap();
        if path.is_dir() {
            if path != PathBuf::from(root_dir) {
                let thread_handle = thread::spawn(move || {
                    println!("{}", path.display());
                    let tmp: u64 = get_folder_size(&path).unwrap();
                    display_size(tmp);
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
    display_size(result);
    Ok(result)
}

