const wasm = require('../pkg');

// sends data to rust and communicates response back to index
function calculate_arbitrage(betting_data) {
    const res = wasm.process_odds(JSON.stringify(betting_data));
    return JSON.parse(res);
}

module.exports = { calculate_arbitrage };