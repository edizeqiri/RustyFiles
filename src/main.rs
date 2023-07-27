mod file_explorer;
mod cli;

use env_logger;
use log::{debug, info, trace, warn};

use file_explorer::FileExplorer;

fn main() {
    env_logger::init();
    debug!("Starting RustyPC");

    let root_dir = "C:\\Users\\edi"; // Replace with the directory you want to traverse
    debug!("{}",root_dir);

    //FileExplorer::get_folder_size_threaded(Path::new(root_dir)).unwrap();

    cli::fileExplorer();
}


