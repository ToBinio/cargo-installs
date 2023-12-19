use crate::util::crates::installed;

pub fn list() -> anyhow::Result<()> {
    let crates = installed()?;

    for crate_data in crates {
        println!("{}", crate_data.name);
    }

    Ok(())
}
