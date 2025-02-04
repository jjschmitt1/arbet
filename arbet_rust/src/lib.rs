use wasm_bindgen:: { prelude::wasm_bindgen, JsValue };
use serde::{Deserialize, Serialize};
use core::f64;
use web_sys::console;

// used for returns from calc_arb function
pub enum RetVals{
    ArbAvail(ArbInfo),
    NoArb,
}

// lets calc_arb communicate percentages to caller
pub struct ArbInfo {
    pub home_stake_percentage: f64,
    pub away_stake_percentage: f64,
    pub profit_percentage: f64,
}

// moneyline struct
#[derive(Serialize, Deserialize, Debug)]
struct Moneyline {
    pub book: String,
    pub home_odds: i32,
    pub away_odds: i32,
}

// game struct
#[derive(Serialize, Deserialize, Debug)]
struct Game {
    pub sport_type: String,
    pub start_time: String,
    pub home_team: String,
    pub away_team: String,
    // pub spread: Vec<Spread>,
    pub moneyline: Vec<Moneyline>,
}

type Games = Vec<Game>;

#[derive(Serialize, Deserialize, Debug)]
struct Team {
    pub name: String,
    pub book: String,
    pub spread: f64,
    pub odds: i32,
    pub stake: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct ArbReturns {
    pub sport_type: String,
    pub start_time: String,
    pub bet_type: String,
    pub home_team: Team,
    pub away_team: Team,
    pub roi: f64,
}

type ReturnData = Vec<ArbReturns>;

#[wasm_bindgen]
pub fn process_odds(data: &str) -> String {
    // parse incoming json data
    let betting_data: Games;
    if let Ok(games) = serde_json::from_str::<Games>(data) {
        // console::log_1(&JsValue::from_str("Data parsed successfully"));
        betting_data = games;
    }
    else{
        let error_msg = format!("Error parsing JSON: {:?}", serde_json::from_str::<Games>(data).err());
        console::log_1(&JsValue::from_str(&error_msg));
        panic!("Couldn't parse data");
    }


    let mut arb_ret: ReturnData = vec![];

    // betting data holds all of the information we need about the games
    // now need to go through for each game and see if there is arb avail
    for (_, game_log) in betting_data.iter().enumerate(){
        // for each game, find best odds between the 3 sites
        let curr_game: &Game = game_log;

        // handle moneyline arbitrage

        // get max home odds
        let max_home_ml_odds = curr_game.moneyline
            .iter()
            .enumerate()
            .map(|(idx, ml)| (ml.home_odds, idx))
            .max_by(|a, b| a.0.cmp(&b.0));

        // get max away odds
        let max_away_ml_odds = curr_game.moneyline
            .iter()
            .enumerate()
            .map(|(idx, ml)| (ml.away_odds, idx))
            .max_by(|a, b| a.0.cmp(&b.0));

            // send ml arbitrage 
            match (max_home_ml_odds, max_away_ml_odds){
                (Some((home_odds, home_idx)), Some((away_odds, away_idx))) => {
    
                    match calc_arb(&home_odds, &away_odds){
                        RetVals::ArbAvail(data) => {
                            // create home team
                            let home_team : Team = Team {
                                name: curr_game.home_team.clone(),
                                book: curr_game.moneyline
                                    .get(home_idx)
                                    .map(|s| s.book.clone())
                                    .unwrap_or_else(||"Unknown".to_string()),
                                spread: 0.0,
                                odds: home_odds,
                                stake: (data.home_stake_percentage * 100.0)
                            };
    
                            let away_team : Team = Team {
                                name: curr_game.away_team.clone(),
                                book: curr_game.moneyline
                                    .get(away_idx)
                                    .map(|s| s.book.clone())
                                    .unwrap_or_else(||"Unknown".to_string()),
                                spread: 0.0,
                                odds: away_odds,
                                stake: (data.away_stake_percentage * 100.0)
                            };
    
                            let arb_opp : ArbReturns = ArbReturns {
                                sport_type: curr_game.sport_type.clone(),
                                start_time: curr_game.start_time.clone(),
                                bet_type: "Moneyline".to_string(),
                                home_team: home_team,
                                away_team : away_team,
                                roi: data.profit_percentage
                            };
    
                            arb_ret.push(arb_opp);
    
                        }
                        _ => {}
                    }
                }
                _ => {}
      
            }
        
    }

    /*
        Enter Spread Arb here
    */

    serde_json::to_string(&arb_ret).unwrap()

}


// func for getting implient probability
fn calc_implied_prob(line: &i32) -> f64 {
    let odds:i32 = *line;
    if odds > 0 {
        100.0 / (odds as f64 + 100.0)
    }
    else{
        (-odds as f64) / ((-odds as f64) + 100.0)
    }
}

// func to see if arb is available or not
pub fn calc_arb(home_line: &i32, away_line: &i32) -> RetVals {
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
