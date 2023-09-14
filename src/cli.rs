use std::fs::File;
use std::path::Path;
use inquire::Select;
use crate::file_explorer::FileExplorer;

extern crate log;


pub fn fileExplorer() {

    // TODO: fix choosing of drive and make less nested match cases
    let options = vec![
        "List files",
        "List files recursively",
        "Get size stats of drive",
        "Exit",
    ];

    let ans = Select::new("RustyPC", options).prompt();

     match ans {
        Ok(choice)  => parse_command(choice),
        Err(_) => println!("There was an error, please try again"),
    }

    fn parse_command(choice: &str) {
        let root_dir = Path::new("C:\\Users\\edi");
        match choice {
            "List files recursively" => FileExplorer::display_size( FileExplorer::get_folder_size_threaded(root_dir).unwrap()),
            // get first element of options
            "Get size stats of drive" => choose_drive_to_analyze(FileExplorer::get_all_drive_letters()),
            &_ => println!("This is still WIP!")
        }
    }

    fn choose_drive_to_analyze(driv_answer: Vec<char>) {
        let ans = Select::new("Drives to choose from", driv_answer).prompt();
        let mut driver = "".to_string();
        match ans {
            Ok(drive)  => driver = drive.to_string(),
            Err(_) => println!("There was an error, please try again"),
        }
        driver = driver + ":\\";
        let path = Path::new(&driver);
        FileExplorer::display_size( FileExplorer::get_folder_size_threaded(path).unwrap())
    }
}

