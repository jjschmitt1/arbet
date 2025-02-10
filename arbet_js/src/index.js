const { fetchData, fetchSports } = require('./apiClient');
const { calculate_arbitrage } = require('./rustBridge');
const { changeJSONFormat } = require('./formatJSON');
const fs = require('fs');

async function main() {

    const args = process.argv.slice(2);

    const activeSports_response = await fetchSports();

    const activeSports = activeSports_response.data.map((game) => game.key);

    const sportsFlags = {
        "-nfl": "americanfootball_nfl",
        "-ncaaf": "americanfootball_ncaaf",
        "-ncaam": "basketball_ncaab",
        "-ncaaw": "basketball_wncaab",
        "-mlb": "baseball_mlb",
        "-euroleague": "basketball_euroleague",
        "-nba": "basketball_nba",
        "-boxing": "boxing_boxing",
        "-nhl": "icehockey_nhl",
        "-mma": "mma_mixed_martial_arts",
        "-prem": "soccer_epl",
        "-championship": "soccer_efl_champ",
        "-bundes": "soccer_germany_bundesliga",
        "-bundes2": "soccer_germany_bundesliga2",
        "-3liga": "soccer_germany_liga3",
        "-lig1": "soccer_france_ligue_one",
        "-seriea": "soccer_italy_serie_a",
        "-eredivisie": "soccer_netherlands_eredivisie",
        "-laliga": "soccer_spain_la_liga",
        "-champions_league": "soccer_uefa_champs_league",
        "-europa_league": "soccer_uefa_europa_league",
        "-conf_league": "soccer_uefa_europa_conference_league"
    };

    let selectedSports = [];

    for (let i = 0; i < args.length; i++){
        if (args[i] in sportsFlags){
            if(activeSports.includes(sportsFlags[args[i]])){
                selectedSports.push(sportsFlags[args[i]]);
            }
            else{
                console.log(`Sport ${sportsFlags[args[i]]} is not active!`);
            }
        }
        else if(args[i] === "-help"){
            helpMenu();
            return;
        }
        else{
            console.log("Invalid flag detected. Use `-help` to see valid flags.");
            return;
        }
    }

    if(selectedSports.length === 0){
        console.log("No sport selected. Please select one or more sports.");
        return;
    }

    // make calls to api for all of the sports
    const api_promises = selectedSports.map(sport => fetchData(sport));

    // wait for all promises 
    const api_results = await Promise.all(api_promises);

    // flatten into a single promise
    const flat_api_results = api_results.flat();

    // change flat api results into the correct JSON format for Rust to handle
    const final_api_results = changeJSONFormat(flat_api_results);

    // must have all promises resolved before sending data to rust
    const arb_return = calculate_arbitrage(final_api_results);

    // handle arbitrage return
    if (arb_return.length === 0 ){
        console.log("No arbitrage found! Try again later.");
    }
    else{
        // sort the return array by sport type
        arb_return.sort((a, b) => a.sport_type.localeCompare(b.sport_type));

        for(var i = 0; i < arb_return.length; i++){
            // for each game
            console.log(`Arbitrage opportunity ${i + 1} in ${arb_return[i].sport_type}:`);
            console.log(`Bet type: ${arb_return[i].bet_type}`);
            console.log(`Start Time: ${arb_return[i].start_time}`);
            console.log(`Place ${arb_return[i].home_team.stake.toFixed(2)}% of your money on ${arb_return[i].home_team.name} at ${arb_return[i].home_team.odds} on ${arb_return[i].home_team.book}`);
            console.log(`Place ${arb_return[i].away_team.stake.toFixed(2)}% of your money on ${arb_return[i].away_team.name} at ${arb_return[i].away_team.odds} on ${arb_return[i].away_team.book}`);
            console.log(`This opportunity yields an ROI of ${arb_return[i].roi.toFixed(2)}%\n`);
        }
    }


    const callsLeft = await fetchSports();
    const requests_remaining = callsLeft.headers.get('x-requests-remaining');
    saveApiCallsLeft(requests_remaining);
    
}

async function saveApiCallsLeft(callsLeft) {
    fs.writeFile('.remaining_calls', JSON.stringify({ value: callsLeft }), (err) => {
        if (err) {
            console.log("Failed to save remaining api calls data");
        }
    });
}

function helpMenu() {
    console.log("Welcome to arbet! Below you will find a list of appropriate flags that can be used:")
    console.log(`   -nfl: NFL 
    -ncaaf: NCAA Football

    -ncaam: NCAA Mens Basketball

    -ncaaw: NCAA Womens Basketball

    -mlb: MLB

    -euroleague: Euroleague Basketball

    -nba: NBA

    -boxing: Boxing

    -nhl: NHL

    -mma: MMA

    -prem: English Premier Leauge Soccer

    -championship: English Championship Soccer (2nd Division)

    -bundes: German Bundesliga Soccer

    -bundes2: German Bundesliga 2 Soccer

    -3liga: German 3.Liga Soccer

    -lig1: French Ligue Un Soccer

    -seriea: Italian Serie A Soccer

    -eredivisie: Dutch Eredivisie Soccer

    -laliga: Spanish La Liga Soccer

    -champions_league: UEFA Champions League Soccer

    -europa_league: UEFA Europa League Soccer

    -conf_league: UEFA Conference League Soccer

    -help: access to this help page

    The books we currently check are DraftKings, Fanduel, MGM, ESPN Bet and Hardrock!
    `);
}

main();