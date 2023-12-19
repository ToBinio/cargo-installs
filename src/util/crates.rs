use anyhow::anyhow;
use home::cargo_home;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

pub fn installed() -> anyhow::Result<Vec<CrateData>> {
    let path = cargo_home()?.join(".crates2.json");

    let file = fs::read_to_string(path)?;

    let parse: RawCrates = serde_json::from_str(&file)?;

    Ok(parse
        .installs
        .into_iter()
        .filter_map(|data| data.try_into().ok())
        .collect())
}

pub struct CrateData {
    pub name: String,
    pub version: String,
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

        Ok(CrateData {
            name: split
                .next()
                .ok_or(anyhow!("could not parse name"))?
                .to_string(),
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
