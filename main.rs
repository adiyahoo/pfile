
use std::env; 
use std::fs;
use std::fs::File;
use std::process::exit;

fn check_duplicate_folder(folder_name: &str) -> bool {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    
    if let Ok(entries) = fs::read_dir(&current_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(entry_name) = entry.file_name().to_str() {
                    if entry_name.eq(folder_name) {
                        return true; // The folder name already has the same
                    }
                }
            }
        }
    }
    
    false // No folder names are the same
}

fn check_args(args: &Vec<String>) { 
    if args.len() < 3 || args[1].is_empty() || args[2].is_empty() {
        eprintln!("Using: pfile <command> <name_file>");
        exit(0);
    } else {
        check_command(&args);  // Check command for match logic
    }  
}

fn check_command(args: &Vec<String>) { 
   match args.get(1) { 
     Some(command) => { 
        match command.as_str() { 
            "cfolder" => create_folder(args),
            "cfile" => create_file(args),
            "delete" => delete_data(args),
            _ => println!("Unknown command"),
        }
     }
     None => println!("No command provided"),
   }
}

fn create_folder(args: &Vec<String>) {
    if let Some(name_file) = args.get(2) {
        let name_file: String = name_file.clone();
        if check_duplicate_folder(&name_file) {
            println!("The folder or file name already has the same name");
        } else {
            println!("Creating folder: {}", name_file);
            match std::fs::create_dir(&name_file) {
                Ok(_) => println!("Folder created successfully"),
                Err(e) => eprintln!("Failed to create folder: {}", e),
            }
        }
    }
}

fn create_file(args: &Vec<String>) { 
    if let Some(name_file) = args.get(2) {
        let name_file: String = name_file.clone();
        if check_duplicate_folder(&name_file) {
            println!("The folder or file name already has the same name");
        } else {
            println!("Creating file: {}", name_file);
            File::create(name_file).expect("Failed to create file");
        }
    }
}

fn read_entry () {
    if let Ok(current_dir) = env::current_dir() {
        if let Ok(entries) = fs::read_dir(current_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(entry_name) = entry.file_name().to_str() {
                        println!("{}", entry_name);
                    }
                }
            }
        }
    }
}

fn delete_data(args: &Vec<String>) {
    if let Some(name_file) = args.get(2) {
        let name_file: String = name_file.clone();
        if let Ok(metadata) = fs::metadata(&name_file) {
            if metadata.is_file() {
                if let Err(e) = fs::remove_file(&name_file) {
                    eprintln!("Failed to delete file: {}", e);
                } else {
                    println!("File deleted: {}", name_file);
                }
            } else if metadata.is_dir() {
                if let Err(e) = fs::remove_dir_all(&name_file) {
                    eprintln!("Failed to delete directory: {}", e);
                } else {
                    println!("Directory deleted: {}", name_file);
                }
            } else {
                eprintln!("Invalid entry: {}", name_file);
            }
        } else {
            eprintln!("Failed to access entry: {}", name_file);
        }
    } else {
        eprintln!("Usage: delete_data <name_file>");
    }
}

fn main() { 
    let args: Vec<String> = env::args().collect();

    if args.get(1) == Some(&String::from("help")) { 
        eprintln!("Using, pfile cfolder <name_folder>, for create folder");
        eprintln!("Using, pfile cfile <name_file_with_extension>, for create file");
        eprintln!("Using, pfile read, for read all folder / file");
        eprintln!("Using, pfile delete <name/path>, for delete folder / file");
        exit(0);
    } 
    
    if args.get(1) == Some(&String::from("read")) { 
        read_entry();
        exit(0);
    }

    check_args(&args);
}