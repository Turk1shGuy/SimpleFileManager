use std::{fs, path::PathBuf};

pub fn read_dir(dir_path: String) -> Result<(), String> {
    let entries = fs::read_dir(dir_path.clone()).unwrap();

    println!("{}/", dir_path);
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            println!("  {}", path.to_str().unwrap());
        } else if path.is_dir() {
            println!("  {}/", path.to_str().unwrap());
        }
    }

    return Ok(());
}

pub fn copy_file(file_path: String, dest_path: String) -> Result<(), String> {
    fs::copy(file_path, dest_path).unwrap();
    return Ok(());
}

pub fn move_filedir(file_path: String, dest_path: String) -> Result<(), String> {
    fs::rename(file_path, dest_path).unwrap();
    return Ok(());
}

pub fn del_file(file_path: String) -> Result<(), String> {
    fs::remove_dir(file_path).unwrap();
    return Ok(());
}

pub fn new_file(file_path: String) -> Result<(), String> {
    match fs::File::create(file_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn search_file(start_path: String, name: String) -> Result<(), String> {
    let name = name.to_ascii_lowercase();
    let entries: fs::ReadDir = fs::read_dir(&start_path).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_file()&& path.file_name().unwrap().to_str().unwrap().to_ascii_lowercase() == name {
            println!("{}", path.to_str().unwrap());
        } else if path.is_dir() {
            
            search_sub_dir(path, name.clone());

        }
    }

    Ok(())
}

fn search_sub_dir(path: PathBuf, name: String) {
    if let Err(e) = search_file(path.to_str().unwrap().to_string(), name) {
        eprintln!("Error searching in subdirectory: {}", e);
    }
}

pub fn move_sub_dir(dir: &mut String) {
    let current_dir = dir.clone();
    let entries = fs::read_dir(current_dir).unwrap();

    println!("Subdirectories:");
    let mut i = 1;
    let mut subdirs: Vec<String> = Vec::new();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            println!("{} - {}", i, path.to_str().unwrap());
            subdirs.push(path.to_str().unwrap().to_string());
            i += 1;
        }
    }

    println!("Enter the number of the subdirectory to move into:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: usize = input.trim().parse().expect("Please type a number!");

    if input > 0 && input <= subdirs.len() {
        *dir = subdirs[input - 1].clone();
        println!("Moved into: {}", dir);
    } else {
        println!("Invalid input");
    }
}

pub fn move_up_dir(dir: &mut String) {
    let path = PathBuf::from(dir.clone());
    let parent_dir = path.parent().unwrap();

    if parent_dir != path.as_path() {
        *dir = parent_dir.to_str().unwrap().to_string();
        println!("Moved up to: {}", dir);
    } else {
        println!("Already in root directory");
    }
}
