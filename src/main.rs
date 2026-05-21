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
    pwd: Option<String>,
}

fn raw_match(query: &str, file_name: &str) -> std::io::Result<()> {
    let path = Path::new(file_name);
    //let display = path.display();
    //print!("path: {:?}\n", display);

    //Open the path in read only mode which returns a io::Result<File>
    let file = File::open(path)?;
    println!("file: {:?}", file);

    //Read the content of the file
    let reader = BufReader::new(&file);

    //Case sensitive match
    for l in reader.lines() {
        let line_content = l?;
        if line_content.contains(query) {
            println!("matched line: {}", line_content);
        }
    }

    Ok(())
}

fn search_file(query_arg: &str, file_name: &str) -> std::io::Result<()> {
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
    //Path of the current-directory
    let path_string: String = match env::current_dir() {
        Ok(path_buf) => path_buf.to_string_lossy().into_owned(),
        Err(_) => String::from("Fallback/Default/Path"),
    };

    //Reading the input and storing it in a vector of string slices
    let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        println!("({}, {})", i, arg);
    }

    //Arg validation
    //if args.len() < 5 {
    //    eprintln!("Number of argument is not satisfied");
    //    return Ok(());
    //}

    if args[1].starts_with('-') {
        let flag: Option<String> = Some(args[1].clone());
        let query = &args[2];
        let file_name = &args[3];
        let mut pwd: Option<String> = None;
        match flag.as_deref() {
            Some("-n") => pwd = None,
            Some("-i") => pwd = None,
            Some("-r") => pwd = Some(path_string),
            _ => eprintln!("Flag didn;t match"),
        }
        let command = Command {
            flag: flag.clone(),
            query: query.to_string(),
            file_name: file_name.to_string(),
            pwd: pwd.clone(),
        };
        search_file(&command.query, &command.file_name)?;
    } else {
        let flag: Option<String> = None;
        let query = &args[1];
        let file_name = &args[2];
        let pwd: Option<String> = None;
        let command = Command {
            flag: flag.clone(),
            query: query.to_string(),
            file_name: file_name.to_string(),
            pwd: pwd.clone(),
        };
        raw_match(&command.query, &command.file_name)?;
    }

    Ok(())
}
