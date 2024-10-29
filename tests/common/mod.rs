use std::fs;
use std::path::Path;

pub fn ensure_db_existance(db_name: &str) -> bool {
    Path::new(db_name).exists()
}

pub fn remove_db(db_name: &str) -> Result<(), std::io::Error> {
    fs::remove_dir(db_name)
}
