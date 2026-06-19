use std::path::PathBuf;

pub struct Command {
    pub flag: Option<String>,
    pub query: String,
    pub file_name: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Peek {
    pub file_name: Option<PathBuf>,
    pub content_vec: Vec<String>,
}
