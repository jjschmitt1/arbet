
function changeJSONFormat(originalJSON) {

    const transformedDataList = originalJSON.map(originalData => ({
        sport_type: originalData.sport_title,
        start_time: originalData.commence_time,
        home_team: "LA Rams",  // Hardcoded change
        away_team: "Minnesota Vikings", // Hardcoded change
        moneyline: originalData.bookmakers.map(bookmaker => {
            // Find the "h2h" market
            const market = bookmaker.markets.find(m => m.key === "h2h");
            if (!market) return null;
    
            // Extract home and away odds
            const homeOutcome = market.outcomes.find(o => o.name === originalData.home_team);
            const awayOutcome = market.outcomes.find(o => o.name === originalData.away_team);
    
            return {
                book: bookmaker.title,
                home_odds: homeOutcome ? homeOutcome.price : null,
                away_odds: awayOutcome ? awayOutcome.price : null
            };
        }) 
    }));

    return transformedDataList;
    
}

module.exports = { changeJSONFormat } ;