const axios = require('axios');
const apiKey = process.env.ODDS_API_KEY;

async function fetchData(sport) {
      try {
        // replace with the correct api link
        const response = await axios.get(`https://api.the-odds-api.com/v4/${sport}/upcoming/odds/?regions=us&markets=h2h&apiKey=${apiKey}&oddsFormat=american&bookmakers=draftkings,fanduel,betmgm,espnbet,hardrockbet`);
        return response.data;
      } catch (error) {
        console.error('Error fetching API data:', error.message);
        throw error;
      }
}

module.exports = { fetchData };