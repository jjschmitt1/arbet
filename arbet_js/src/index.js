const { fetchData } = require('./apiClient');
const { calculate_arbitrage } = require('./rustBridge');
const { fs } = require('fs');

async function main() {

    const args = process.argv.slice(2);

    for(let i = 0; i < process.args.length; i++){
        switch (args[i]){
            case "-nfl":
                // handle nfl case
                break;
            case "-ncaaf":
                // handle ncaaf case
                break;
            case "-ncaam":
                // handle case for ncaam (mens bball)
                break;
            case "-ncaaw":
                // handle case for ncaaw (womesn bball)
                break;
            case "-mlb":
                // handle case for mlb
                break;
            case "-euroleague":
                // handle case for euroleague bball
                break;
            case "-nba":
                // handle nba case
                break;
            case "-boxing":
                // handle boxing case (can remove later)
                break;
            case "-nhl":
                // handle nhl case
                break;
            case "-mma":
                // handle mma case
                break;
            case "-prem":
                // handle prem case
                break;
            case "-championship":
                // handle championship call
                break;
            case "-bundes":
                // handle bundesliga case
                break;
            case "-bundes2":
                // handle bundesliga 2 case
                break;
            case "-bundes3":
                // handle bundesliga 3 case
                break;
            case "-lig1":
                // handle french first div case
                break;
            case "-seriea":
                // handle serie A case
                break;
            case "-eredivisie":
                // handle eredivise case
                break;
            case "-laliga":
                // handle la liga case
                break;
            case "-champions_league":
                // handle uefa champs
                break;
            case "-europa_league":
                // handle europa league
                break;
            case "-conf_league":
                // handle conference league
                break;
            case "-help":
                // handle help case
                helpMenu();
                return;
            default:
                console.log("Invalid flag. See 'node index.js -help' for correct flags");
                return;
        }
    }

    const api_return = await fetchData();
    const arb_return = calculate_arbitrage(api_return);

    // handle arbitrage return
    if (arb_return.length === 0 ){
        console.log("No arbitrage found! Try again later.\n");
    }
    else{
        for(var i = 0; i < arb_return.length; i++){
            // for each game
            console.log(`Arbitrage opportunity ${i + 1} in ${arb_return[i].sport_type}:`);
            console.log(`Bet type: ${arb_return[i].bet_type}`);
            console.log(`Place ${arb_return[i].home_team.stake.toFixed(2)}% of your money on ${arb_return[i].home_team.name} at ${arb_return[i].home_team.odds} on ${arb_return[i].home_team.book}`);
            console.log(`Place ${arb_return[i].away_team.stake.toFixed(2)}% of your money on ${arb_return[i].away_team.name} at ${arb_return[i].away_team.odds} on ${arb_return[i].away_team.book}`);
            console.log(`This opportunity yields an ROI of ${arb_return[i].roi.toFixed(2)}%\n`);
        }
    }


    // sometime later, add functionality to track number of api calls remaining
    
}

async function saveApiCallsLeft(callsLeft) {
    fs.writeFile('.remaining_calls', JSON.stringify({ value: callsLeft }));
}

function helpMenu() {
    console.log("Welcome to arbet! Below you will find a list of appropriate flags that can be used:")
    console.log(`-nfl: NFL 
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

    -bundes3: German 3.Liga Soccer

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