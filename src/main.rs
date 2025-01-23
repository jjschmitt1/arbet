use std::io::{self, Write};

use arb::RetVals;
mod arb;

fn main() {
    println!("Welcome to arbet, your personal arbitrage betting finder!");

    // buffer for taking input
    let mut buf = String::new();

    // print prompt for first line
    print!("Line 1: ");
    io::stdout().flush().unwrap();

    // get input for first line
    io::stdin().read_line(&mut buf).expect("Error reading input");

    // cast to int
    let line_one: i32 = buf.trim().parse::<i32>().unwrap();

    // clear the buffer
    buf.clear();

    // prompt for second line
    print!("Line 2: ");
    io::stdout().flush().unwrap();

    // read second line
    io::stdin().read_line(&mut buf).expect("Error reading input");

    // cast second line
    let line_two: i32 = buf.trim().parse::<i32>().unwrap();

    // get result from calc_arb and print data accordingly
    match arb::calc_arb(&line_one, &line_two) {
        RetVals::ArbAvail(arb_data) => {
            println!("Arbitrage found! Here is some data about the opportunity:\nPlace {:.2}% on Line 1\nPlace {:.2}% on Line 2\nTotal Profit Percentage: {:.2}%",
            arb_data.stake_one_precentage * 100.0,
            arb_data.stake_two_percentage * 100.0,
            arb_data.profit_percentage,
            );

            // clear buffer
            buf.clear();

            // prompt user
            print!("Enter the amount of money you have available to wager: $");
            io::stdout().flush().unwrap();

            // get input into the buffer
            io::stdin().read_line(&mut buf).expect("Error reading wager input");

            let money_avail: f64 = buf.trim().parse::<f64>().unwrap();

            println!("With ${:.2}, you should place ${:.2} on Line 1 and ${:.2} on Line 2 for a {:.2}% profit!",
            money_avail,
            money_avail * arb_data.stake_one_precentage,
            money_avail * arb_data.stake_two_percentage,
            arb_data.profit_percentage,
            );

        },
        RetVals::NoArb => {
            println!("No arbitrage was found with the given lines.");
        }
    } 


}
