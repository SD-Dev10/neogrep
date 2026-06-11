use std::env;
use std::path::Path;
mod ngrep_api;
mod qpeek;
mod util;
use crate::ngrep_api::{recursive_search, sensitive_matches};
use crate::util::Command;

//Command format structure
fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    if args[1] == "-r"
        || args[1] == "-qpeek" && args[1] != "-i" && args[1] != "-n" && args[1] != "-ni"
    {
        let flag: Option<String> = Some(args[1].clone());
        let query = &args[2];
        //let file_name = &args[3];
        let mut command = Command {
            flag: flag.clone(),
            query: query.to_string(),
            file_name: Vec::new(),
        };
        for (i, entry) in args.iter().enumerate() {
            if i > 0 && Path::new(&entry).is_file() {
                command.file_name.push(entry.clone());
            } else if entry.contains(".") {
                command.file_name.push(entry.clone());
            }
        }
        recursive_search(&command.query, &command.file_name, command.flag)?;
    } else {
        if args.len() == 3 {
            let mut command = Command {
                flag: None,
                query: args[1].clone(),
                file_name: Vec::new(),
            };
            for (i, entry) in args.iter().enumerate() {
                if i > 0 && Path::new(&entry).is_file() {
                    command.file_name.push(entry.clone());
                }
            }
            sensitive_matches(command.flag, &command.query, &command.file_name)?;
        } else if Path::new(&args[2]).is_file() && Path::new(&args[3]).is_file() {
            let mut command = Command {
                flag: None,
                query: args[1].clone(),
                file_name: Vec::new(),
            };
            for (i, entry) in args.iter().enumerate() {
                if i > 0 && Path::new(&entry).is_file() {
                    command.file_name.push(entry.clone());
                }
            }
            sensitive_matches(command.flag, &command.query, &command.file_name)?;
        } else {
            let mut command = Command {
                flag: Some(args[1].clone()),
                query: args[2].clone(),
                file_name: Vec::new(),
            };
            for (i, entry) in args.iter().enumerate() {
                if i > 0 && Path::new(&entry).is_file() {
                    command.file_name.push(entry.clone());
                }
            }
            sensitive_matches(command.flag, &command.query, &command.file_name)?;
        }
    }

    Ok(())
}
