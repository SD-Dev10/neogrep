use std::env;
use std::fs::File;
//use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
//use std::path::PathBuf;
use walkdir::WalkDir;

//Command format structure
struct Command {
    flag: Option<String>,
    query: String,
    file_name: String,
}

fn sensitive_matches(flag: Option<String>, query: &str, file_name: &str) -> std::io::Result<()> {
    let path = Path::new(file_name);
    //let display = path.display();
    //print!("path: {:?}\n", display);

    //Open the path in read only mode which returns a io::Result<File>
    let file = File::open(path)?;
    println!("file: {:?}", file);
    println!("flag: {:?}", flag.as_deref());
    //Read the content of the file
    if flag.as_deref().is_none() {
        //Case sensitive match
        let reader = BufReader::new(&file);
        for l in reader.lines() {
            let line_content = l?;
            if line_content.contains(query) {
                println!("matched line: {}", line_content);
            }
        }
    } else if flag.as_deref() == Some("-ni") {
        let path = Path::new(&file_name);
        let file = File::open(path)?;
        let reader = BufReader::new(&file);
        for (i, l) in reader.lines().enumerate() {
            let line_content = l?;
            let line_vec: Vec<&str> = line_content.split_whitespace().collect();
            for entry in line_vec {
                //println!("entry: {}", entry);
                if query.eq_ignore_ascii_case(entry) {
                    println!("{}: {}", i + 1, line_content);
                    break;
                }
            }
        }
    } else {
        //Case insensitive match
        let path = Path::new(&file_name);
        let file = File::open(path)?;
        let reader = BufReader::new(&file);
        for l in reader.lines() {
            let line_content = l?;
            let line_vec: Vec<&str> = line_content.split_whitespace().collect();
            for entry in line_vec {
                //println!("entry: {}", entry);
                if query.eq_ignore_ascii_case(entry) {
                    println!("matched line: {}", line_content);
                    break;
                }
            }
        }
    }

    Ok(())
}

fn recursive_search_wih_linenum(query_arg: &str, file_name: &str) -> std::io::Result<()> {
    let path = Path::new(file_name);
    let current_directory = env::current_dir().expect("couldn't find directory");
    //open the path in read only mode which returns a io::Result<File>
    let file = File::open(path)?;
    println!("file: {:?}", file);

    match file_name {
        //Recursive search engine using -r flag
        "." => {
            for entry in WalkDir::new(current_directory)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                //println!("{}", entry.path().display());
                if entry.file_type().is_file() {
                    let open_entry = File::open(entry.path())?;
                    let reader = BufReader::new(&open_entry);
                    let full_path = entry.path();
                    let base = env::current_dir()?;
                    for line in reader.lines().flatten() {
                        if line.contains(query_arg)
                            && let Ok(relative_path) = full_path.strip_prefix(&base)
                        {
                            println!("./{}: {}", relative_path.display(), line);
                        }
                    }
                }
            }
        }
        _ => {
            //Return line numbers along with the line using -n flag
            //Read the content of the file
            let reader = BufReader::new(&file);
            for (i, l) in reader.lines().enumerate() {
                let line_content = l?;
                if line_content.contains(query_arg) {
                    //let line_content = l?;
                    println!("{}: {}", i + 1, line_content);
                }
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    //Reading the input and storing it in a vector of string slices
    let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        println!("({}, {})", i, arg);
    }

    //Arg validation
    if args.len() < 3 {
        eprintln!("Number of argument is not satisfied");
        return Ok(());
    }

    if args[1].starts_with('-') && args[1] != "-i" && args[1] != "-ni" {
        let flag: Option<String> = Some(args[1].clone());
        let query = &args[2];
        let file_name = &args[3];
        let command = Command {
            flag: flag.clone(),
            query: query.to_string(),
            file_name: file_name.to_string(),
        };
        recursive_search_wih_linenum(&command.query, &command.file_name)?;
    } else {
        if args.len() == 3 {
            let command = Command {
                flag: None,
                query: args[1].clone(),
                file_name: args[2].clone(),
            };
            sensitive_matches(command.flag, &command.query, &command.file_name)?;
        } else {
            let command = Command {
                flag: Some(args[1].clone()),
                query: args[2].clone(),
                file_name: args[3].clone(),
            };
            sensitive_matches(command.flag, &command.query, &command.file_name)?;
        }
    }

    Ok(())
}
