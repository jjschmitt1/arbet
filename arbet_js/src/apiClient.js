const axios = require('axios');
const apiKey = process.env.ODDS_API_KEY;

// Date math
const currDate = new Date().toISOString().split('T')[0];
const commenceFrom = currDate + "T12:00:00Z";
const tmrw = new Date()
tmrw.setDate(tmrw.getDate() + 1);
const commenceTo = tmrw.toISOString().split('T')[0] + "T07:00:00Z";

// regex to replace : with "%3A" in html request
const commenceFromFormat = commenceFrom.replace(/:\s*/g, "%3A");
const commenceToFormat = commenceTo.replace(/:\s*/g, "%3A");

async function fetchData(sport) {
      try {
        // replace with the correct api link
        const response = await axios.get(`https://api.the-odds-api.com/v4/sports/${sport}/odds/?regions=us&markets=h2h&apiKey=${apiKey}&oddsFormat=american&bookmakers=draftkings%2Cfanduel%2Cbetmgm%2Cespnbet%2Chardrockbet&commenceTimeFrom=${commenceFromFormat}&commenceTimeTo=${commenceToFormat}`);
        return response.data;
      } catch (error) {
        console.error('Error fetching API data!!');
        throw error;
      }
}

module.exports = { fetchData };