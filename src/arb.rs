pub enum RetVals{
    ArbAvail(ArbInfo),
    NoArb,
}

pub struct ArbInfo {
    pub stake_one_precentage: f64,
    pub stake_two_percentage: f64,
    pub profit_percentage: f64,
}

pub fn calc_implied_prob(line: &i32) -> f64 {
    let odds:i32 = *line;
    if odds > 0 {
        100.0 / (odds as f64 + 100.0)
    }
    else{
        (-odds as f64) / ((-odds as f64) + 100.0)
    }
}

pub fn calc_arb(line_one: &i32, line_two: &i32) -> RetVals {
    let prob_one: f64 = calc_implied_prob(line_one);
    let prob_two: f64 = calc_implied_prob(line_two);
    let total_prob: f64 = prob_one + prob_two;

    if total_prob < 1.0 {
        let stake_one: f64 = (1.0 / prob_one) / (1.0 / prob_one + 1.0 / prob_two);
        let stake_two: f64 = 1.0 - stake_one;

        let profit_percentage: f64 = (1.0 / total_prob - 1.0) * 100.0;

        let arb_data: ArbInfo = ArbInfo {
            stake_one_precentage: stake_one,
            stake_two_percentage: stake_two,
            profit_percentage: profit_percentage,
        };

        RetVals::ArbAvail(arb_data)
    }
    else{
        RetVals::NoArb   
    }
}