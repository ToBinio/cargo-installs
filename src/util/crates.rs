use crate::util::sparse::get_highest_version;
use anyhow::anyhow;
use home::cargo_home;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::{fs, thread};

pub fn get_installed() -> anyhow::Result<Vec<CrateData>> {
    let path = cargo_home()?.join(".crates.toml");

    let file = fs::read_to_string(path)?;
    let parse: RawCrates = toml::from_str(&file)?;

    //todo - simpler multithreading
    let (sender, receiver) = channel::<Result<CrateData, anyhow::Error>>();

    for (text, _bins) in parse.v1 {
        let sender = sender.clone();
        thread::spawn(move || {
            let result = text.try_into();
            sender.send(result).expect("something went wrong");
        });
    }

    drop(sender);

    Ok(receiver.iter().filter_map(|data| data.ok()).collect())
}

#[derive(Debug)]
pub struct CrateData {
    pub name: String,
    pub version: String,
    pub latest_version: String,
    pub origen: String,
}

impl CrateData {
    pub fn is_latest_version(&self) -> bool {
        self.version == self.latest_version
    }
}

impl TryFrom<String> for CrateData {
    type Error = anyhow::Error;

    fn try_from(full_name: String) -> Result<Self, Self::Error> {
        let mut split = full_name.split(' ');

        let name = split
            .next()
            .ok_or(anyhow!("could not parse name: \"{}\")", full_name))?
            .to_string();
        let version = split
            .next()
            .ok_or(anyhow!("could not parse version: \"{}\")", full_name))?
            .to_string();
        let origen = split
            .next()
            .ok_or(anyhow!("could not parse origen: \"{}\")", full_name))?
            .to_string();

        Ok(CrateData {
            latest_version: get_highest_version(&name)?,
            name,
            version,
            origen,
        })
    }
}

#[derive(Deserialize, Debug)]
struct RawCrates {
    pub v1: RawCrateData,
}

type RawCrateData = HashMap<String, Vec<String>>;
