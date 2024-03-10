use core::panic;
use std::{fs::{self, metadata}, io::Error};

pub fn ensure_folder(path: &str) -> Result<(), Error>{
    match fs::metadata(path){
        Ok(metadata) => {
            if !metadata.is_dir(){
                return Err(Error::new(std::io::ErrorKind::Other,format!("Error, {} is not a directory", path)))
            }
        },
        Err(_err) => ()
        
    }

    println!("Creating folder {}", path);
    fs::create_dir_all(path).map_err(|e| Error::new(std::io::ErrorKind::Other, format!("Unable to create {}: {}", path, e)))
}