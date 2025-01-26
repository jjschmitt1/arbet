use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

// used for returns from calc_arb function
enum RetVals{
    ArbAvail(ArbInfo),
    NoArb,
}

// lets calc_arb communicate percentages to caller
struct ArbInfo {
    pub home_stake_percentage: f64,
    pub away_stake_percentage: f64,
    pub profit_percentage: f64,
}

// Structs to hold incoming json
#[derive(Serialize, Deserialize, Debug)]
struct Spread {
    pub book: String,
    pub home_odds: String,
    pub home_spread: String,
    pub away_odds: String,
    pub away_spread: String,
}

// moneyline struct
#[derive(Serialize, Deserialize, Debug)]
struct Moneyline {
    pub book: String,
    pub home_odds: String,
    pub away_odds: String,
}

// game struct
#[derive(Serialize, Deserialize, Debug)]
struct Game {
    pub home_team: String,
    pub away_team: String,
    pub spread: Vec<Spread>,
    pub moneyline: Vec<Moneyline>,
}

type Games = Vec<Game>;
//type BooksSearched = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
struct Team {
    pub name: String,
    pub book: String,
    pub spread: String,
    pub odds: String,
    pub stake: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ArbReturns {
    pub bet_type: String,
    pub home_team: Team,
    pub away_team: Team,
    pub roi: String,
}

type ReturnData = Vec<ArbReturns>;

#[wasm_bindgen]
pub fn process_odds(data: &str) -> String {

    // parse incoming json data
    let betting_data: Games = serde_json::from_str(data).unwrap();

    // get list of books searched, sent from json
    //let books: BooksSearched = serde_json::from_str(books_searched).unwrap();

    let mut arb_ret: ReturnData = vec![];

    // betting data holds all of the information we need about the games
    // now need to go through for each game and see if there is arb avail
    for (_, game_log) in betting_data.iter().enumerate(){
        // for each game, find best odds between the 3 sites
        let curr_game: &Game = game_log;

        // find best home odds
        let max_home_odds = curr_game.spread
            .iter()
            .enumerate()
            .filter_map(|(index, s)| {
                s.home_odds.parse::<f64>()
                    .ok()
                    .map(|odds| (odds, index))
            })
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));

        match max_home_odds {
            Some((odds, _)) => println!("Max home odds: {}", odds),
            None => println!("No valid home odds found."),
        }

        let max_away_odds = curr_game.spread
        .iter()
        .enumerate()
        .filter_map(|(index, s)| {
            s.away_odds.parse::<f64>()
                .ok()
                .map(|odds| (odds, index)) 
        })
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));

        match max_away_odds {
            Some((odds, _)) => println!("Max away odds: {}", odds),
            None => println!("No valid away odds found."),
        }

        // -- correctly getting the both max odds --

        // send the best odds to the calc_arb function, 
        match (max_home_odds, max_away_odds){
            (Some((home_odds, home_idx)), Some((away_odds, away_idx))) => {

                match calc_arb(&home_odds, &away_odds){
                    RetVals::ArbAvail(data) => {
                        // create home team
                        let home_team : Team = Team {
                            name: curr_game.home_team.clone(),
                            book: curr_game.spread
                                .get(home_idx)
                                .map(|s| s.book.clone())
                                .unwrap_or_else(||"Unknown".to_string()),
                            spread: curr_game.spread
                                .get(home_idx)
                                .map(|s| s.home_spread.clone())
                                .unwrap_or_else(|| "Unknown".to_string()),
                            odds: home_odds.to_string(),
                            stake: (data.home_stake_percentage * 100.0).to_string()
                        };

                        let away_team : Team = Team {
                            name: curr_game.away_team.clone(),
                            book: curr_game.spread
                                .get(away_idx)
                                .map(|s| s.book.clone())
                                .unwrap_or_else(||"Unknown".to_string()),
                            spread: curr_game.spread
                                .get(away_idx)
                                .map(|s| s.away_spread.clone())
                                .unwrap_or_else(|| "Unknown".to_string()),
                            odds: away_odds.to_string(),
                            stake: (data.away_stake_percentage * 100.0).to_string()
                        };

                        let arb_opp : ArbReturns = ArbReturns {
                            bet_type: "Spread".to_string(),
                            home_team: home_team,
                            away_team : away_team,
                            roi: data.profit_percentage.to_string()
                        };

                        arb_ret.push(arb_opp);

                    }
                    _ => {}
                }
            }
            _ => {}
  
        }
    }

    serde_json::to_string(&arb_ret).unwrap()

}

// func for getting implient probability
fn calc_implied_prob(line: &f64) -> f64 {
    let odds:f64 = *line;
    if odds > 0.0 {
        100.0 / (odds as f64 + 100.0)
    }
    else{
        (-odds as f64) / ((-odds as f64) + 100.0)
    }
}

// func to see if arb is available or not
fn calc_arb(home_line: &f64, away_line: &f64) -> RetVals {
    let prob_home: f64 = calc_implied_prob(home_line);
    let prob_away: f64 = calc_implied_prob(away_line);
    let total_prob: f64 = prob_home + prob_away;

    if total_prob < 1.0 {
        let stake_home: f64 = prob_home / total_prob;
        let stake_away: f64 = 1.0 - stake_home;

        let profit_percentage: f64 = (1.0 / total_prob - 1.0) * 100.0;

        let arb_data: ArbInfo = ArbInfo {
            home_stake_percentage: stake_home,
            away_stake_percentage: stake_away,
            profit_percentage: profit_percentage,
        };

        RetVals::ArbAvail(arb_data)
    }
    else{
        RetVals::NoArb   
    }
}
