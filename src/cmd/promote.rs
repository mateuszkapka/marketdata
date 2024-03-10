use clap::ValueEnum;
use crate::{paths::{nas, scratch}, utils::rsync};

#[derive(Debug, ValueEnum, Clone)]
pub enum PromoteTarget{
    Aggs,
    Ticks
}

pub fn promote(target: &PromoteTarget){
    match target{
        PromoteTarget::Aggs =>{
            rsync(
                scratch::NORMALISED_OUTPUT_PATH,
                format!("{}:{}", nas::NAS_HOSTNAME, nas::NAS_NORMALISED_OUTPUT_PATH).as_str());

            rsync(
                scratch::SYMBOLOGY_OUTPUT_PATH,
                format!("{}:{}", nas::NAS_HOSTNAME, nas::NAS_SYMBOLOGY_OUTPUT_PATH).as_str());
        },
        PromoteTarget::Ticks => {
            rsync(
                scratch::WSE_TICK_FILES_PATH,
                format!("{}:{}", nas::NAS_HOSTNAME, nas::NAS_WSE_TICK_FILES_PATH).as_str()
            );
            rsync(
                scratch::NASDAQ_TICK_FILES_PATH,
                format!("{}:{}", nas::NAS_HOSTNAME, nas::NAS_NASDAQ_TICK_FILES_PATH).as_str()
            );
        }
    }
}