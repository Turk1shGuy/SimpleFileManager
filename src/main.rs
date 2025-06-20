use std::io::Write;

fn main() {
    let mut current_dir = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    loop {
        println!("\nFile Manager");
        println!("-----------");
        println!("1. List files and directories");
        println!("2. Copy file");
        println!("3. Move file/directory");
        println!("4. Delete file");
        println!("5. Crate New file");
        println!("6. Search file");
        println!("7. Move into subdirectory");
        println!("8. Move up directory");
        println!("9. Quit");

        println!("Current directory: {}", current_dir);
        print!("Enter your choice:");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: u32 = input.trim().parse().expect("Please type a number!");

        match input {
            1 => {
                fileexplorer::read_dir(current_dir.clone()).unwrap();
            }
            2 => {
                print!("Enter the source file path:");
                std::io::stdout().flush().unwrap();
                let mut src = String::new();
                std::io::stdin()
                    .read_line(&mut src)
                    .expect("Failed to read line");
                let src = src.trim().to_string();
                print!("Enter the destination file path:");
                std::io::stdout().flush().unwrap();
                let mut dest = String::new();
                std::io::stdin()
                    .read_line(&mut dest)
                    .expect("Failed to read line");
                let dest = dest.trim().to_string();
                fileexplorer::copy_file(src, dest).unwrap();
            }
            3 => {
                print!("Enter the source file/directory path:");
                std::io::stdout().flush().unwrap();
                let mut src = String::new();
                std::io::stdin()
                    .read_line(&mut src)
                    .expect("Failed to read line");
                let src = src.trim().to_string();
                print!("Enter the destination file/directory path:");
                std::io::stdout().flush().unwrap();
                let mut dest = String::new();
                std::io::stdin()
                    .read_line(&mut dest)
                    .expect("Failed to read line");
                let dest = dest.trim().to_string();
                fileexplorer::move_filedir(src, dest).unwrap();
            }
            4 => {
                print!("Enter the file path to delete:");
                std::io::stdout().flush().unwrap();
                let mut file = String::new();
                std::io::stdin()
                    .read_line(&mut file)
                    .expect("Failed to read line");
                let file = file.trim().to_string();
                fileexplorer::del_file(file).unwrap();
            }
            5 => {
                let mut buf = String::new();

                print!("File name: ");
                std::io::stdout().flush().unwrap();

                std::io::stdin()
                    .read_line(&mut buf)
                    .expect("Failed to read line");

                fileexplorer::new_file(buf).unwrap();
            }
            6 => {
                print!("Enter the file name to search:");
                std::io::stdout().flush().unwrap();
                let mut name = String::new();
                std::io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                let name = name.trim().to_string();
                fileexplorer::search_file(current_dir.clone(), name).unwrap();
            }
            7 => {
                fileexplorer::move_sub_dir(&mut current_dir);
            }
            8 => {
                fileexplorer::move_up_dir(&mut current_dir);
            }
            9 => {
                break;
            }
            _ => {
                print!("Invalid choice. Please choose a valid option.");
                std::io::stdout().flush().unwrap();
            }
        }
    }
}
