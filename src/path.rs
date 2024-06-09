use std::path::PathBuf;

pub fn save_directory() -> Result<PathBuf, std::io::Error> {
    let directories = directories::UserDirs::new()
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;
    let documents_path = directories
        .document_dir()
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;

    let save_directory_path = documents_path
        .join("Paradox Interactive")
        .join("Crusader Kings III")
        .join("save games");

    Ok(save_directory_path)
}

pub fn save_data() -> Result<PathBuf, std::io::Error> {
    let directories = directories::ProjectDirs::from("me.iyican", "Emre Iyican", "CK3-SaveScummer")
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;
    let path = directories.data_dir().to_owned();

    Ok(path)
}
