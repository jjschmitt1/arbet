const axios = require('axios');
const apiKey = process.env.ODDS_API_KEY;

// Date math
const currDate = new Date().toISOString().split('T')[0];
const commenceFrom = currDate + "T12%3A00%3A00Z";
const tmrw = new Date()
tmrw.setDate(tmrw.getDate() + 1);
const commenceTo = tmrw.toISOString().split('T')[0] + "T07%3A00%3A00Z";

// fetches arbitrage for the next day in sports. Maybe switch to indef? not sure yet
async function fetchData(sport) {
      try {
        const response = await axios.get(`https://api.the-odds-api.com/v4/sports/${sport}/odds/?regions=us&markets=h2h&apiKey=${apiKey}&oddsFormat=american&bookmakers=draftkings%2Cfanduel%2Cbetmgm%2Cespnbet%2Chardrockbet&commenceTimeFrom=${commenceFrom}&commenceTimeTo=${commenceTo}`);
        return response.data;
      } catch (error) {
        console.error('Error fetching API data!!');
        throw error;
      }
}

// returns list of actives sports per the api
async function fetchSports() {
    try {
        const response = await axios.get(`https://api.the-odds-api.com/v4/sports/?apiKey=${apiKey}`);
        return response;
    }
    catch (error){
        console.error("Error getting sports data");
        throw error;
    }
}

module.exports = { fetchData, fetchSports };