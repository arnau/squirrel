use squirrel::{services::{importer, self}, entities::Storage};

fn main() -> anyhow::Result<()> {
    let pool = Storage::memory()?;
    services::starter::setup(&pool.get()?)?;

    // let cat2021 = "/Users/arnau/kitchen/squirrel/playground/catalogue/2021_JC_Candanedo-v11.lrcat";
    let cat2019 = "/Users/arnau/kitchen/squirrel/playground/catalogue/2019_JC_Candanedo-v11.lrcat";

    importer::import(&pool, cat2019)?;

    Ok(())
}
