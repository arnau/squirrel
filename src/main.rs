use squirrel::config;
use squirrel::repositories::EventRepository;
use squirrel::services;

fn main() -> anyhow::Result<()> {
    let db = config::db_path();

    if !db.exists() {
        dbg!("mec");
    }

    let pool = services::starter::start(":memory:")?;
    // let pool = services::starter::start("./squirrel.db")?;

    let cat2021 = "/Users/arnau/kitchen/squirrel/playground/catalogue/2021_JC_Candanedo-v11.lrcat";
    let cat2019 = "/Users/arnau/kitchen/squirrel/playground/catalogue/2019_JC_Candanedo-v11.lrcat";

    // services::importer::import(&pool, cat2019)?;
    services::importer::import(&pool, cat2021)?;

    let events = EventRepository::head(&pool.get()?, 4)?;

    for event in events {
        dbg!(event);
    }

    Ok(())
}
