const axios = require('axios');
const apiKey = process.env.ODDS_API_KEY;

async function fetchData() {
      try {
        // replace with the correct api link
        const response = await axios.get('https://jsonplaceholder.typicode.com/posts/1');
        return response.data;
      } catch (error) {
        console.error('Error fetching API data:', error.message);
        throw error;
      }
    // const testJsonData = [
    //     {
    //         "sport_type": "Amercian Football",
    //         "home_team": "LA Rams",
    //         "away_team": "Minnesota Vikings",
    //         "moneyline": [
    //             {
    //                 "book": "Draftkings",
    //                 "home_odds": 120,
    //                 "away_odds": -142
    //             },
    //             {
    //                 "book": "Fanduel",
    //                 "home_odds": 122,
    //                 "away_odds": -144
    //             },
    //             {
    //                 "book": "MGM",
    //                 "home_odds": 127,
    //                 "away_odds": -147
    //             }
    //         ]
    //     },
    //     {
    //         "home_team": "Kansas City Chiefs",
    //         "away_team": "Buffalo Bills",
    //         "moneyline": [
    //             {
    //                 "book": "Draftkings",
    //                 "home_odds": 100,
    //                 "away_odds": -120
    //             },
    //             {
    //                 "book": "Fanduel",
    //                 "home_odds": -102,
    //                 "away_odds": -118
    //             }
    //         ]
    //     },
    //     {
    //         "home_team": "New York Giants",
    //         "away_team": "Philadelphia Eagles",
    //         "moneyline": [
    //             {
    //                 "book": "Draftkings",
    //                 "home_odds": 135,
    //                 "away_odds": -155
    //             },
    //             {
    //                 "book": "Fanduel",
    //                 "home_odds": 140,
    //                 "away_odds": -160
    //             }
    //         ]
    //     },
    //     {
    //         "home_team": "Dallas Cowboys",
    //         "away_team": "San Francisco 49ers",
    //         "moneyline": [
    //             {
    //                 "book": "Draftkings",
    //                 "home_odds": 100,
    //                 "away_odds": -120
    //             },
    //             {
    //                 "book": "Fanduel",
    //                 "home_odds": 105,
    //                 "away_odds": -125
    //             }
    //         ]
    //     },
    //     {
    //         "home_team": "Green Bay Packers",
    //         "away_team": "Chicago Bears",
    //         "moneyline": [
    //             {
    //                 "book": "Draftkings",
    //                 "home_odds": 200,
    //                 "away_odds": -250
    //             },
    //             {
    //                 "book": "Fanduel",
    //                 "home_odds": 210,
    //                 "away_odds": -260
    //             },
    //             {
    //                 "book": "MGM",
    //                 "home_odds": 220,
    //                 "away_odds": -270
    //             }
    //         ]
    //     }
    // ]
    

    // return testJsonData;
}

module.exports = { fetchData };