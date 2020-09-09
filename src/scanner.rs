use rayon::prelude::*;
use std::{fs, io, path};

pub(crate) fn scan(
    environment: &str,
    base_path: &str,
    file_name: &str,
) -> Result<(Vec<String>, Vec<String>), io::Error> {
    let paths: Vec<path::PathBuf> = fs::read_dir(base_path)?
        .filter_map(|res| {
            let path = res.map(|ent| ent.path());
            match path.as_ref() {
                Ok(p)
                    if p.to_string_lossy()
                        .split("/")
                        .all(|a| a.chars().nth(0).map_or(false, |c| c != '.')) =>
                {
                    path.ok()
                }
                _ => None,
            }
        })
        .collect();

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

pub(crate) fn scan_concurrent(
    environment: &str,
    base_path: &str,
    file_name: &str,
) -> Result<(Vec<String>, Vec<String>), io::Error> {
    let paths: Vec<path::PathBuf> = fs::read_dir(base_path)?
        .filter_map(|res| {
            let path = res.map(|ent| ent.path());
            match path.as_ref() {
                Ok(p)
                    if p.to_string_lossy()
                        .split("/")
                        .all(|a| a.chars().nth(0).map_or(false, |c| c != '.')) =>
                {
                    path.ok()
                }
                _ => None,
            }
        })
        .collect();

    let files = paths
        .par_iter()
        .filter_map(|path| path.to_str().map(|str| str.to_owned()))
        .collect::<Vec<String>>();

    let contents = paths
        .into_par_iter()
        .map(|mut path| {
            path.push(format!("{}/{}", environment, file_name));
            fs::read_to_string(path).unwrap_or("".to_string())
        })
        .collect::<Vec<String>>();

    Ok((files, contents))
}
