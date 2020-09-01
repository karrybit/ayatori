use std::{fs, io};

mod lexer;
mod model;
mod parser;
mod token_type;

pub fn scan(
    environment: &str,
    base_path: &str,
    file_name: &str,
) -> Result<(Vec<String>, Vec<String>), io::Error> {
    let paths = fs::read_dir(base_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let files = paths
        .iter()
        .filter_map(|path| path.to_str().map(|str| str.to_owned()))
        .collect::<Vec<String>>();

    let contents = paths
        .into_iter()
        .map(|mut path| {
            path.push(format!("{}/{}", environment, file_name));
            fs::read_to_string(path).unwrap_or("".to_string())
        })
        .collect::<Vec<String>>();

    Ok((files, contents))
}

pub fn parse_services(files: Vec<String>, contents: Vec<String>) -> Vec<model::Service> {
    let service_names = files
        .into_iter()
        .map(|file| {
            let splited = file.split('/').last();
            splited.map_or("", |s| s.trim_matches('"')).to_owned()
        })
        .collect::<Vec<String>>();
    let resources = contents
        .into_iter()
        .map(|content| parse(content))
        .collect::<Vec<Vec<model::Resource>>>();
    service_names
        .into_iter()
        .zip(resources.into_iter())
        .map(|(name, resources)| model::Service::new(name, resources))
        .collect::<Vec<model::Service>>()
}

fn parse(content: String) -> Vec<model::Resource> {
    if content.is_empty() {
        return vec![];
    }
    let mut lexer = lexer::Lexer::new(content);
    let mut parser = parser::Parser::new(&mut lexer);
    let resources = parser.parse_resources();
    resources
}
