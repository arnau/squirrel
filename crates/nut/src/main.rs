use nut::config;
use nut::repositories::EventRepository;
use nut::services;

fn main() -> anyhow::Result<()> {
    // let db = config::db_path();
    // config::create_data_dir()?;
    // let pool = services::starter::start(db.to_str().unwrap())?;


    let db_location = "/Users/arnau/Library/ApplicationSupport/net.seachess.squirrel/squirrel.db";
    // let pool = services::starter::start(":memory:")?;
    let pool = services::starter::start(db_location)?;

    let cat2021 = "/Users/arnau/kitchen/squirrel/playground/catalogue/2021_JC_Candanedo-v11.lrcat";
    let cat2019 = "/Users/arnau/kitchen/squirrel/playground/catalogue/2019_JC_Candanedo-v11.lrcat";

    services::importer::import(&pool, cat2019)?;
    services::importer::import(&pool, cat2021)?;

    // let events = EventRepository::head(&pool.get()?, 4)?;
    //
    // for event in events {
    //     dbg!(event);
    // }

    // let s = services::navigator::find_by_catalogue_path(&pool, "/2019/20181204_Coco_and_Eve/Export_hi_res/Coco_And_Eve31221-Edit.jpg")?;
    // TODO: If no trailing path it should normalise. If not exists, error.
    // let s = services::navigator::get_path(&pool, "/2019/20181204_Coco_and_Eve/Export_hi_res/")?;
    let s = services::navigator::locate_ground(&pool)?;

    println!("{}", serde_json::to_string(&s)?);

    Ok(())
}
