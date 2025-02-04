const wasm = require('../pkg');

function calculate_arbitrage(betting_data) {
    //console.log(JSON.stringify(betting_data));
    const res = wasm.process_odds(JSON.stringify(betting_data));
    return JSON.parse(res);
}

module.exports = { calculate_arbitrage };