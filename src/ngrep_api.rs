use crate::qpeek::qpeek_w;
use crate::util::Peek;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use indexmap::IndexMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::stdout;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn sensitive_matches(
    flag: Option<String>,
    query: &str,
    file_name: &Vec<String>,
) -> std::io::Result<()> {
    for f_name in file_name {
        let path = Path::new(&f_name);
        //Open the path in read only mode which returns a io::Result<File>
        let file = File::open(path)?;

        let reader = BufReader::new(&file);
        //println!("file: {:?}", file);
        // println!("flag: {:?}", flag.as_deref());
        //Read the content of the file
        if flag.as_deref().is_none() {
            //Case sensitive match
            for l in reader.lines() {
                let line_content = l?;
                if line_content.contains(query) && file_name.len() > 1 {
                    println!("{:?}: {}", f_name, line_content);
                }
                if line_content.contains(query) && file_name.len() == 1 {
                    println!("Matched line: {}", line_content);
                }
            }
        } else if flag.as_deref() == Some("-n") {
            //Return line numbers along with the line using -n flag
            //Read the content of the file
            for (i, l) in reader.lines().enumerate() {
                let line_content = l?;
                if line_content.contains(query) && file_name.len() > 1 {
                    println!("{:?}: {}", f_name, line_content);
                    break;
                }
                if line_content.contains(query) && file_name.len() == 1 {
                    //let line_content = l?;
                    println!("{}: {}", i + 1, line_content);
                    break;
                }
            }
        } else if flag.as_deref() == Some("-ni") {
            for (i, l) in reader.lines().enumerate() {
                let line_content = l?;
                let line_vec: Vec<&str> = line_content.split_whitespace().collect();
                for entry in line_vec {
                    //println!("entry: {}", entry);
                    if query.eq_ignore_ascii_case(entry) && file_name.len() > 1 {
                        println!("{:?}: {}: {}", f_name, i + 1, line_content);
                        break;
                    }
                    if query.eq_ignore_ascii_case(entry) && file_name.len() == 1 {
                        println!("{}: {}", i + 1, line_content);
                        break;
                    }
                }
            }
        } else {
            //Case insensitive match
            for l in reader.lines() {
                let line_content = l?;
                let line_vec: Vec<&str> = line_content.split_whitespace().collect();
                for entry in line_vec {
                    //println!("entry: {}", entry);
                    if query.eq_ignore_ascii_case(entry) && file_name.len() > 1 {
                        println!("{:?}: {}", f_name, line_content);
                        break;
                    }
                    if query.eq_ignore_ascii_case(entry) && file_name.len() == 1 {
                        println!("Matched line: {}", line_content);
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn recursive_search(
    query: &str,
    file_name: &Vec<String>,
    flag: Option<String>,
) -> color_eyre::Result<()> {
    for f_name in file_name {
        let current_directory = env::current_dir().expect("couldn't find directory");
        let path_to_check = Path::new(&f_name);
        // //open the path in read only mode which returns a io::Result<File>
        // let file = File::open(path)?;
        // println!("file: {:?}", file);

        //quick peek logic
        let mut peek_vec: Vec<Peek> = Vec::new();

        //Match for recursive_search
        if path_to_check == Path::new(".")
            || path_to_check.canonicalize().ok() == Some(current_directory.clone())
                && flag.as_deref() != Some("-qpeek")
        {
            //Recursive search engine using -r flag
            for entry in WalkDir::new(&current_directory)
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
                        if line.contains(query)
                            && let Ok(relative_path) = full_path.strip_prefix(&base)
                        {
                            let fname = Some(relative_path.to_path_buf());
                            let mut cvec = Vec::new();
                            cvec.push(line.clone());

                            //Instance of a singel peek
                            let peek = Peek {
                                file_name: fname,
                                content_vec: cvec,
                            };
                            if let Some(names) = &peek.file_name.as_deref() {
                                println!("./{}: {}", names.display(), line);
                            }

                            peek_vec.push(peek);
                        }
                    }
                }
            }
        } else {
            eprintln!("Invalid partent folder detected");
        }

        if flag.as_deref() == Some("-qpeek") {
            // Terminal Setup
            enable_raw_mode()?;
            let mut stdout = stdout();
            execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?; // enter TUI mode
            let backend = CrosstermBackend::new(stdout);
            let terminal = Terminal::new(backend)?;

            // Run app

            //Formatting the peek vec
            let mut formatted_map: IndexMap<Option<PathBuf>, Vec<String>> = IndexMap::new();
            for items in peek_vec {
                formatted_map
                    .entry(items.file_name)
                    .and_modify(|existing_content| {
                        existing_content.extend(items.content_vec.clone());
                    })
                    .or_insert(items.content_vec);
            }
            let filtered_vec: Vec<Peek> = formatted_map
                .into_iter()
                .map(|(fname, content)| Peek {
                    file_name: fname,
                    content_vec: content,
                })
                .collect();
            println!("filtered_vec: {:?}", filtered_vec);
            let result = qpeek_w(terminal, filtered_vec, query);

            // Restore terminal
            disable_raw_mode()?;
            execute!(std::io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

            return result;
        }
    }

    Ok(())
}
