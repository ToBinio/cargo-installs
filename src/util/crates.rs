use crate::util::crates::Origen::{Local, Remote};
use crate::util::sparse::get_highest_version;
use anyhow::anyhow;
use home::cargo_home;
use serde::Deserialize;
use std::collections::HashMap;
use std::{fs, thread};

pub fn get_installed() -> anyhow::Result<Vec<CrateData>> {
    let path = cargo_home()?.join(".crates.toml");

    let file = fs::read_to_string(path)?;
    let parse: RawCrates = toml::from_str(&file)?;

    let mut handles = vec![];

    for (text, _bins) in parse.v1 {
        let handle = thread::spawn(move || CrateData::from_definition(&text));

        handles.push(handle);
    }

    let mut crates = Vec::with_capacity(handles.len());
    for handle in handles {
        crates.push(
            handle
                .join()
                .map_err(|_| anyhow!("could not join threads"))??,
        );
    }

    Ok(crates)
}

#[derive(Debug)]
pub struct CrateData {
    pub name: String,
    pub version: String,
    pub origen: Origen,
}

impl CrateData {
    pub fn is_latest_version(&self) -> bool {
        match &self.origen {
            Local => true,
            Remote { latest_version } => &self.version == latest_version,
        }
    }

    pub fn from_definition(definition: &str) -> anyhow::Result<Self> {
        let mut split = definition.split(' ');

        let name = split
            .next()
            .ok_or(anyhow!("could not parse name: \"{}\")", definition))?
            .to_string();
        let version = split
            .next()
            .ok_or(anyhow!("could not parse version: \"{}\")", definition))?
            .to_string();
        let origen = split
            .next()
            .ok_or(anyhow!("could not parse origen: \"{}\")", definition))?
            .to_string();
        let origen = Origen::from_definition(&name, &origen)?;

        Ok(CrateData {
            name,
            version,
            origen,
        })
    }
}

#[derive(Debug)]
pub enum Origen {
    Local,
    Remote { latest_version: String },
}

impl Origen {
    pub fn from_definition(crate_name: &str, definition: &str) -> anyhow::Result<Self> {
        if definition.starts_with("(path+") {
            Ok(Local)
        } else {
            let latest_version = get_highest_version(crate_name)?;
            Ok(Remote { latest_version })
        }
    }
}

#[derive(Deserialize, Debug)]
struct RawCrates {
    pub v1: RawCrateData,
}

type RawCrateData = HashMap<String, Vec<String>>;
