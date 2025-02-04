use std::io::{self, Write};

use arbet_rust::{ RetVals, calc_arb };
// use serde_json::{self, Value};

fn main() {
    println!("Welcome to arbet, your personal arbitrage betting finder!");

    // buffer for taking input
    let mut buf = String::new();

    // print prompt for first line
    print!("Line 1: ");
    io::stdout().flush().unwrap();

    // get input for first line
    io::stdin().read_line(&mut buf).expect("Error reading input");

    // cast to int
    let line_one: i32 = buf.trim().parse::<i32>().unwrap();

    // clear the buffer
    buf.clear();

    // prompt for second line
    print!("Line 2: ");
    io::stdout().flush().unwrap();

    // read second line
    io::stdin().read_line(&mut buf).expect("Error reading input");

    // cast second line
    let line_two: i32 = buf.trim().parse::<i32>().unwrap();

    // call into calc_arb
    let arb_res: RetVals = calc_arb(&line_one, &line_two);

    match arb_res {
        RetVals::ArbAvail(data) => {
            println!("There is arbitrage available!");
            println!("Place {}% of your stake on line 1", data.home_stake_percentage);
            println!("Place {}% of your stake on line 2", data.away_stake_percentage);
            println!("This results in a total ROI of {}%!", data.profit_percentage);
        }
        _ => {
            println!("Error retreiving arbitrage data.");
        }
    }



    // ------------------ Testing module below --------------------------
//     let test_json_data = r#"
// [
//     {
//         "home_team": "LA Rams",
//         "away_team": "Minnesota Vikings",
//         "moneyline": [
//             {
//                 "book": "Draftkings",
//                 "home_odds": "120",
//                 "away_odds": "-142"
//             },
//             {
//                 "book": "Fanduel",
//                 "home_odds": "122",
//                 "away_odds": "-144"
//             },
//             {
//                 "book": "MGM",
//                 "home_odds": "127",
//                 "away_odds": "-147"
//             }
//         ]
//     },
//     {
//         "home_team": "Kansas City Chiefs",
//         "away_team": "Buffalo Bills",
//         "moneyline": [
//             {
//                 "book": "Draftkings",
//                 "home_odds": "100",
//                 "away_odds": "-120"
//             },
//             {
//                 "book": "Fanduel",
//                 "home_odds": "-102",
//                 "away_odds": "-118"
//             }
//         ]
//     },
//     {
//         "home_team": "New York Giants",
//         "away_team": "Philadelphia Eagles",
//         "moneyline": [
//             {
//                 "book": "Draftkings",
//                 "home_odds": "135",
//                 "away_odds": "-155"
//             },
//             {
//                 "book": "Fanduel",
//                 "home_odds": "140",
//                 "away_odds": "-160"
//             }
//         ]
//     },
//     {
//         "home_team": "Dallas Cowboys",
//         "away_team": "San Francisco 49ers",
//         "moneyline": [
//             {
//                 "book": "Draftkings",
//                 "home_odds": "100",
//                 "away_odds": "-120"
//             },
//             {
//                 "book": "Fanduel",
//                 "home_odds": "105",
//                 "away_odds": "-125"
//             }
//         ]
//     },
//     {
//         "home_team": "Green Bay Packers",
//         "away_team": "Chicago Bears",
//         "moneyline": [
//             {
//                 "book": "Draftkings",
//                 "home_odds": "200",
//                 "away_odds": "-250"
//             },
//             {
//                 "book": "Fanduel",
//                 "home_odds": "210",
//                 "away_odds": "-260"
//             },
//             {
//                 "book": "MGM",
//                 "home_odds": "220",
//                 "away_odds": "-270"
//             }
//         ]
//     }
// ]

// "#;

// // TODO(jj): ---------- ^^ fix above json for arbitrage, gpt stinks --------------

// let arbitrage_opps: String = process_odds(&test_json_data);

// if arbitrage_opps.trim().is_empty() || arbitrage_opps.trim() == "[]" {
//     println!("There are no available arbitrage opportunities at this time.")
// }
// else{

//     match serde_json::from_str::<Value>(&arbitrage_opps){
//         Ok(parsed_json) => {
//             println!("{}", serde_json::to_string_pretty(&parsed_json).unwrap());
//         }
//         Err(_) => {
//             println!("There was an error parsing the JSON return.");
//         }
//     }
    
// }




}
