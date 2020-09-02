use std::{fs, io};

pub(crate) fn scan(
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
