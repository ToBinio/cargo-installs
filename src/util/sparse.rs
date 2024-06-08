use color_eyre::eyre;
use color_eyre::eyre::eyre;
use crates_index::{Crate, SparseIndex};

fn update_index(name: &str) -> eyre::Result<()> {
    let index = SparseIndex::new_cargo_default()?;

    let request: ureq::Request = index.make_cache_request(name)?.into();
    let response = request.call()?;

    index.parse_cache_response(name, response.into(), true)?;

    Ok(())
}

fn get_crate(name: &str) -> eyre::Result<Crate> {
    let index = SparseIndex::new_cargo_default()?;
    let krate = index.crate_from_cache(name)?;

    Ok(krate)
}

pub fn get_highest_version(name: &str) -> eyre::Result<String> {
    update_index(name)?;
    let krate = get_crate(name)?;

    Ok(krate
        .highest_normal_version()
        .map(|version| version.version().to_string())
        .ok_or(eyre!("could not find version for \"{}\" found", name))?
        .clone())
}
