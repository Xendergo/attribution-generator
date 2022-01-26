use std::{fs, io, path::PathBuf};

fn path_file() -> PathBuf {
    dirs::data_dir()
        .unwrap()
        .join("attribution-generator")
        .join("default.txt")
}

pub fn get_path() -> Option<String> {
    fs::read_to_string(path_file()).ok()
}

pub fn write_path(path: String) -> Result<(), io::Error> {
    fs::create_dir_all(path_file().parent().unwrap())?;
    fs::write(path_file(), path)
}

pub fn get_directory_data(path: PathBuf) -> Result<Vec<(String, String)>, io::Error> {
    let files = fs::read_dir(path)?;

    let mut ret = Vec::new();

    for maybe_file in files {
        let file = maybe_file?;

        let maybe_name = file.file_name();

        let name = maybe_name.to_string_lossy();

        if name.ends_with(".attribution.txt") {
            ret.push((
                name.strip_suffix(".attribution.txt").unwrap().to_owned(),
                fs::read_to_string(file.path())?,
            ))
        }
    }

    ret.sort();

    Ok(ret)
}
