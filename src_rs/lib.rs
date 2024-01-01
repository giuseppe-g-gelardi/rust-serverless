use rand::seq::SliceRandom;
use std::collections::HashMap;
use url::Url;
use vercel_runtime::{Error, Request};

pub fn choose_starter() -> String {
    let pokemons = vec!["Bulbasaur", "Charmander", "Squirtle", "Pikachu"];
    let starter = pokemons.choose(&mut rand::thread_rng()).unwrap();
    starter.to_string()
}

pub fn req_url_parser(req: Request, query: &str) -> Result<Option<String>, Error> {
    // Parse the request URL
    let parsed_url = Url::parse(&req.uri().to_string()).map_err(|e| {
        // Handle URL parsing error here
        // For example, you can log the error and return an appropriate Error variant
        Error::from(e)
    })?;

    // Collect query parameters into a HashMap
    let hash_query: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();

    // Get the value associated with the specified query parameter
    let id_key = hash_query.get(query).map(|s| s.clone());

    Ok(id_key)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_choose_starter() {
        let starter = choose_starter();
        assert!(
            starter == "Bulbasaur"
                || starter == "Charmander"
                || starter == "Squirtle"
                || starter == "Pikachu"
        );
    }
}
