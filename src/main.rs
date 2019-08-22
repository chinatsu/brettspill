extern crate reqwest;
extern crate roxmltree;
extern crate chrono;


use chrono::prelude::*;
mod call;
use call::*;


fn main() -> Result<(), Box<std::error::Error>> {
    //let call: String = Thing::new()
    //    .with_id(174430)
    //    .with_id(167791)
    //    .with_id(173346)
    //    .with_type(ThingType::Boardgame)
    //    .no_stats()
    //    .no_versions()
    //    .no_historical()
    //    .no_marketplace()
    //    .no_ratingcomments()
    //    .no_videos()
    //    .to_string();
    //println!("{}", call);
    // https://www.boardgamegeek.com/xmlapi2/thing?id=174430,167791,173346&type=boardgame&versions=0&videos=0&stats=0&historical=0&marketplace=0&ratingcomments=0&page=1&pagesize=10
    //let resp: String = reqwest::get(&call)?.text()?;
    //let doc = match roxmltree::Document::parse(&resp) {
    //    Ok(doc) => doc,
    //    Err(e) => {
    //        println!("Error: {}.", e);
    //        return Ok(());
    //    },
    //};

    let collection: String = Collection::new("kyrremann".to_string())
        .modified_since(Utc.ymd(2014, 7, 8))
        .set_filter(CollectionArgument::PreviouslyOwned, true)
        .to_string();

    println!("{}", collection);
    //for node in doc.descendants() {
    //    if node.is_element() && node.tag_name().name() == "name" && node.attributes()[0].value() == "primary" {
    //        println!("Name: {}", node.attributes()[2].value());
    //    }
        // Name: Gloomhaven
        // Name: Terraforming Mars
        // Name: 7 Wonders Duel
    //};
    Ok(())
}
