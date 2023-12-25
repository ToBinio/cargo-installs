use crate::util::sparse::get_highest_version;
use anyhow::anyhow;
use home::cargo_home;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::{fs, thread};

pub fn get_installed() -> anyhow::Result<Vec<CrateData>> {
    let path = cargo_home()?.join(".crates2.json");

    let file = fs::read_to_string(path)?;

    let parse: RawCrates = serde_json::from_str(&file)?;

    let (sender, receiver) = channel::<Result<CrateData, anyhow::Error>>();

    for data in parse.installs {
        let sender = sender.clone();
        thread::spawn(move || {
            let result = data.try_into();
            sender.send(result).expect("TODO: panic message");
        });
    }

    drop(sender);

    Ok(receiver
        .iter()
        .filter_map(|data| data.ok())
        .collect())
}

#[derive(Debug)]
pub struct CrateData {
    pub name: String,
    pub version: String,
    pub newest_version: String,
    pub origen: String,
    pub version_req: Option<String>,
    pub bins: Vec<String>,
    pub features: Vec<String>,
    pub all_features: bool,
    pub no_default_features: bool,
    pub profile: String,
    pub target: String,
    pub rustc: String,
}

impl TryFrom<(String, RawCrateData)> for CrateData {
    type Error = anyhow::Error;

    fn try_from((name, data): (String, RawCrateData)) -> Result<Self, Self::Error> {
        let mut split = name.split(' ');

        let name = split
            .next()
            .ok_or(anyhow!("could not parse name"))?
            .to_string();

        Ok(CrateData {
            newest_version: get_highest_version(&name)?,
            name,
            version: split
                .next()
                .ok_or(anyhow!("could not parse name"))?
                .to_string(),
            origen: split
                .next()
                .ok_or(anyhow!("could not parse name"))?
                .to_string(),
            version_req: data.version_req,
            bins: data.bins,
            features: data.features,
            all_features: data.all_features,
            no_default_features: data.no_default_features,
            profile: data.profile,
            target: data.target,
            rustc: data.rustc,
        })
    }
}

#[derive(Deserialize, Debug)]
struct RawCrates {
    pub installs: HashMap<String, RawCrateData>,
}

#[derive(Deserialize, Debug)]
struct RawCrateData {
    pub version_req: Option<String>,
    pub bins: Vec<String>,
    pub features: Vec<String>,
    pub all_features: bool,
    pub no_default_features: bool,
    pub profile: String,
    pub target: String,
    pub rustc: String,
}
