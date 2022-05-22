use squirrel::config;
use squirrel::services;

fn main() -> anyhow::Result<()> {
    let db = config::db_path();

    if !db.exists() {
        dbg!("mec");
    }

    let pool = services::starter::start(":memory:")?;

    // // let cat2021 = "/Users/arnau/kitchen/squirrel/playground/catalogue/2021_JC_Candanedo-v11.lrcat";
    let cat2019 = "/Users/arnau/kitchen/squirrel/playground/catalogue/2019_JC_Candanedo-v11.lrcat";

    services::importer::import(&pool, cat2019)?;

    Ok(())
}
