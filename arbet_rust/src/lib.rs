use wasn_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

// Structs to hold incoming json
#[derive(Serialize, Deserialize, Debug)]
pub struct Spread {
    pub book: String,
    pub home_odds: String,
    pub home_spread: String,
    pub away_odds: String,
    pub away_spread: String,
}

// moneyline struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Moneyline {
    pub book: String,
    pub home_odds: String,
    pub away_odds: String,
}

// game struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub home_team: String,
    pub away_team: String,
    pub spread: Vec<Spread>,
    pub moneyline: Vec<Moneyline>,
}

pub type Games = Vec<Game>;
pub type BooksSearched = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub name: String,
    pub book: String,
    pub odds: String,
    pub stake: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArbReturns {
    pub arb_avail: String,
    pub bet_type: String,
    pub home_team: Team,
    pub away_team: Team,
    pub roi: String,
}

pub type ReturnData = Vec<ArbReturns>;

#[wasm_bindgen]
pub fn process_odds(data: &str, books_searched: &str) -> String {

    // parse incoming json data
    let betting_data: Games = serde_json::from_str(data).unwrap();

    // get list of books searched, sent from json
    let books: BooksSearched = serde_json::from_str(books_searched).unwrap();

    let arb_ret: ReturnData = vec![];

    // betting data holds all of the information we need about the games
    // now need to go through for each game and see if there is arb avail
    for (i, game_log) in betting_data.iter().enumerate(){
        // for each game, find best odds between the 3 sites
        let curr_game: Game = game_log.get(i);

        // deal with finding best spread for each team
        let curr_game_spread: Vec<Spread> = curr_game.spread;

        // find best home odds
        let max_home_odds = curr_game_spread
            .iter()
            .filter_map(|s| s.home_odds.parse::<f64>().ok())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

        match max_home_odds {
            Some(max) => println!("Max home odds: {}", max),
            None => println!("No valid home odds found."),
        }


        let curr_game_moneyline: Vec<Moneyline> = curr_game.moneyline;


        // send the best odds to the calc_arb function, which can be moved into this file

        // if arb is avail, create ArbVals instance 

        // push ArbReturns to arb_ret
    }



}

