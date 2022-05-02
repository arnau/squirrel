use squirrel::services::importer;

fn main() -> anyhow::Result<()> {
    // let cat2021 = "/Users/arnau/kitchen/squirrel/playground/catalogue/2021_JC_Candanedo-v11.lrcat";
    let cat2019 = "/Users/arnau/kitchen/squirrel/playground/catalogue/2019_JC_Candanedo-v11.lrcat";

    importer::import(cat2019)?;

    Ok(())
}
