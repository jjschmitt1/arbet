const axios = require('axios');
const apiKey = process.env.ODDS_API_KEY;

async function fetchData(sport) {
      try {
        // replace with the correct api link
        const response = await axios.get(`https://api.the-odds-api.com/v4/sports/${sport}/odds/?regions=us&markets=h2h&apiKey=${apiKey}&oddsFormat=american&bookmakers=draftkings%2Cfanduel%2Cbetmgm%2Cespnbet%2Chardrockbet`);
        return response.data;
      } catch (error) {
        console.error('Error fetching API data!!');
        throw error;
      }
}

module.exports = { fetchData };