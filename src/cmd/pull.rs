use std::{fs, process::Command};

use crate::{parsers::parser::ParserType, paths::nas::NAS_ENV_FOLDER, utils::ensure_folder};

fn ensure_env(){
    ensure_folder("/scratch/data").unwrap();
    ensure_folder("/scratch/ensure_folder").unwrap();
    ensure_folder("/scratch/symbology_data").unwrap();
}

pub fn pull(){
    ensure_env();

    println!("Rsyncing nas envs to local /scratch...");
    Command::new("rsync")
        .args(["-arc", 
        format!("polandnas.synology.me:{}", NAS_ENV_FOLDER).as_str(),
        "/scratch/data"])
        .output()
        .expect("Unable to run rsync");
}

pub fn promote(market: &ParserType){

}