// use rand::seq::SliceRandom;
use std::collections::HashMap;
use url::Url;
use vercel_runtime::{Error, Request};

// bring blog into scope
pub mod blog;
pub mod blog_post;

// pub fn choose_starter() -> String {
//     let pokemons = vec!["Bulbasaur", "Charmander", "Squirtle", "Pikachu"];
//     let starter = pokemons.choose(&mut rand::thread_rng()).unwrap();
//     starter.to_string()
// }

pub fn req_url_parser(req: Request, query: &str) -> Result<Option<String>, Error> {
    let parsed_url = Url::parse(&req.uri().to_string()).map_err(|e| Error::from(e))?;
    let hash_query: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();
    let id_key = hash_query.get(query).map(|s| s.clone());
    Ok(id_key)
}

pub fn new_blog_post() -> blog::blog::BlogPost {
    return blog::blog::BlogPost::new(
        Some("1704202457-QVNO".to_string()),
        "A trip to Iceland".to_string(),
        "Iceland is a Nordic island country in the North Atlantic, with a population of 360,390 and an area of 103,000 km2 (40,000 sq mi), making it the most sparsely populated country in Europe. The capital and largest city is Reykjavík. Reykjavík and the surrounding areas in the southwest of the country are home to over two-thirds of the population. Iceland is volcanically and geologically active. The interior consists of a plateau characterised by sand and lava fields, mountains, and glaciers, and many glacial rivers flow to the sea through the lowlands. Iceland is warmed by the Gulf Stream and has a temperate climate, despite a high latitude just outside the Arctic Circle. Its high latitude and marine influence keep summers chilly, with most of the archipelago having a tundra climate.".to_string(),
        "Watson & Crick".to_string(),
        "2019-07-10T16:04:44.000Z".to_string(),
        true,
    );
}
