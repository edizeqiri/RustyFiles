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
            "Get size stats of drive" => FileExplorer::display_size( FileExplorer::get_folder_size_threaded(root_dir).unwrap()),
            &_ => todo!()
        }
    }
}

