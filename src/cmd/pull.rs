use crate::paths::nas::NAS_HOSTNAME;
use crate::utils::rsync;

use crate::{paths::nas::NAS_ENV_FOLDER, utils::ensure_folder};

fn ensure_env(){
    ensure_folder("/scratch/data").unwrap();
    ensure_folder("/scratch/normalised_data/").unwrap();
    ensure_folder("/scratch/symbology_data").unwrap();
    ensure_folder("/scratch/aggregate_data").unwrap();
}

pub fn pull(){
    ensure_env();

    rsync(format!("{}:{}",NAS_HOSTNAME, NAS_ENV_FOLDER).as_str(), "/scratch/data");
}