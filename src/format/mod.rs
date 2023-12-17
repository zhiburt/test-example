use std::{
    fs,
    io::{Read, Result},
    path::Path,
};

pub mod markdown;

pub fn parse(path: &Path) -> Result<Vec<Code>> {
    let file = fs::OpenOptions::new().read(true).open(path)?;

    match path.extension() {
        Some(ext) => {
            if ext == "md" {
                markdown::parse(file)
            } else {
                parse_file(file)
            }
        }
        None => parse_file(file),
    }
}

fn parse_file(mut file: fs::File) -> Result<Vec<Code>> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    Ok(vec![Code::new(buf, vec![], Target::Build)])
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Code {
    pub text: String,
    pub include: Vec<String>,
    pub target: Target,
}

impl Code {
    pub fn new(text: String, include: Vec<String>, target: Target) -> Self {
        Self {
            text,
            include,
            target,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Target {
    Build,
    Run,
}
