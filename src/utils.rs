use std::{fs, io::{stderr, stdout, Error}, process::Command};

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn rsync(from: &str, to: &str){
    println!("syncing data from {} to {}", from, to);
    Command::new("rsync")
        .args(["-arc", 
        from,
        to])
        .output()
        .expect("Unable to run rsync");
}
