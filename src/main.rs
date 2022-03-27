#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    // println!("Input a miner address: ");
    // io::stdout().flush();
    // io::stdin().read_line(&mut miner_addr);
    ask_console("Input a miner address: ", &mut miner_addr);

    // println!("Difficulty: ");
    // io::stdout().flush();
    // io::stdin().read_line(&mut difficulty);
    ask_console("Input the Difficulty: ", &mut difficulty);

    let diff = difficulty.trim().parse::<u32>().expect("We need an integer");

    println!("Generating genesis block!");

    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu: ");
        println!("1. New Transaction");
        println!("2. Mine block");
        println!("3. Change difficulty");
        println!("4. Change reward");
        println!("5. Exit");

        io::stdout().flush().unwrap();
        choice.clear();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting");
                process::exit(0);
            },
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                // println!("Ender sender address: ");
                // io::stdout().flush();
                // io::stdin().read_line(&mut sender);
                ask_console("Ender sender address: ", &mut sender);

                // println!("Ender receiver address: ");
                // io::stdout().flush();
                // io::stdin().read_line(&mut receiver);
                ask_console("Ender receiver address:: ", &mut receiver);

                // println!("Ender amount: ");
                // io::stdout().flush();
                // io::stdin().read_line(&mut amount);
                ask_console("Ender amount: ", &mut amount);

                let res = chain.new_transaction(sender.trim().to_string(), receiver.trim().to_string(), amount.trim().parse().unwrap());

                match res {
                    true => println!("transaction added"),
                    false => println!("transaction failed")
                }
            },
            2 => {
                println!("Generating block");

                let res = chain.generate_new_block();

                match res {
                    true => println!("block generated"),
                    false => println!("block failed")
                }
            },
            3 => {
                let mut new_diff = String::new();

                // println!("Ender the new difficulty: ");
                // io::stdout().flush();
                // io::stdin().read_line(&mut new_diff);
                ask_console("Ender the new difficulty: ", &mut new_diff);

                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());

                match res {
                    true => println!("difficulty updated"),
                    false => println!("difficulty failed")
                }
            },
            4 => {
                let mut new_reward = String::new();

                // println!("Ender the new reward: ");
                // io::stdout().flush();
                // io::stdin().read_line(&mut new_reward);
                ask_console("Ender the new reward: ", &mut new_reward);

                let res = chain.update_reward(new_reward.trim().parse().unwrap());

                match res {
                    true => println!("reward updated"),
                    false => println!("reward failed")
                }
            },
            _ => println!("Invalid choice")
        };
    }

    fn ask_console(msg: &str, value: &mut String) {
        println!("{}", msg);
        io::stdout().flush().unwrap();
        io::stdin().read_line(value).unwrap();
    }
}
