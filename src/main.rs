use std::fs;
use std::fs::{ReadDir};
use std::io::Error;

fn main() {
    // take input from args
    let args: Vec<String> = std::env::args().collect();

    dbg!(&args);

    if args.len() > 2 {
        match args[2].as_str() {
            "echo" => {
                // if no input has provided after echo, print empty string
                if args.len() == 4 {
                    println!("{}", args[args.len() - 1])
                } else {
                    println!()
                }
            }
            "cat" => {
                if args.len() == 4 {
                    let file_path = args[args.len() - 1].as_str();
                    // print the file
                    let file_result = fs::read_to_string(file_path);

                    match file_result {
                        Ok(file_string) => {
                            println!("{}", file_string)
                        }
                        Err(e) => {
                            println!("{}", e)
                        }
                    }
                }
            }
            "ls" => {
                // if file path is provided use that file path
                let mut file_path = "./";
                if args.len() == 4 {
                    file_path = args[args.len() - 1].as_str();
                }
                let path_result: Result<ReadDir, Error> = fs::read_dir(file_path);

                match path_result {
                    Ok(files) => {
                        for file in files {
                            let file_name = file.unwrap();
                            let file_name_path = file_name.path();
                            let final_file_trimmed = file_name_path.to_str()
                                .expect("unable to get file name").replace("./", "");
                            let final_file_arr: Vec<&str> = final_file_trimmed.split("/").collect();
                            let final_file_name = final_file_arr[final_file_arr.len() - 1];
                            if file_name_path.is_dir() {
                                print!("\x1b[34m{}/\x1b[0m \t", final_file_name)
                            } else {
                                print!("{} \t", final_file_name)
                            }
                        }
                        println!("\r");
                    }
                    Err(e) => {
                        println!("{}", e)
                    }
                }
            }
            "find" => {
                // find [PATH] [OPTIONS] [VALUES]
                // simple file search
                let mut res_vec = vec![];
                file_read("./", "main.rs", &mut res_vec);
                println!("{:?}", res_vec)
            }
            "grep" => {}
            // if a command is not implemented, run it on default $SHELL level
            _ => println!("unknown command")
        }
    }
}

// DFS approach to search for a file name by calling it recursively
fn file_read(path: &str, name: &str, res_arr: &mut Vec<String>) {

    // loop through the files till it matches the name
    let file_bytes = fs::read_dir(path).expect("error reading file from path {path}");
    for file in file_bytes {
        let file_path = file.unwrap().path();
        if file_path.is_dir() {
            file_read(file_path.to_str().unwrap(), name, res_arr)
        } else {
            let file_display = file_path.to_str().expect("error converting to str");
            let final_file_trimmed = file_display.replace("./", "");
            let final_file_arr: Vec<&str> = final_file_trimmed.split("/").collect();
            let file_name = final_file_arr[final_file_arr.len() - 1];
            if file_name == name {
                res_arr.push(file_display.to_string())
            }
        }
    }
}
