use std::{thread, time::Duration, io::{self,Write}};
use rand::Rng;
fn main() {
    let mut score_ai = 0;
    let mut score_me = 0;
    let mut noscore = 0;
    loop {
        println!("\n\n1: Rock");
        println!("2: Paper");
        println!("3: Scissors");
        if let Ok(item) = read_input("\nChoose one: ").parse::<usize>() {
            if item <= 3 && item > 0 {
                match play(item) {
                    0 => {
                        println!("You lost!");
                        score_ai += 1;
                        },
                    1 => {
                        println!("You win!");
                        score_me += 1;
                    },
                    2 => {
                        println!("Draw.");
                        noscore += 1;
                    },
                    _ => {}
                };
                println!("\nScore: \nYou:{}\nAI:{}\nNone:{}", score_me, score_ai, noscore);
                match read_input("\nPress anything to replay, Q to quit: ").to_lowercase().as_str() {
                    "q" => { break; },
                    _ => {}
                }
            }
            else {
                println!("Invalid input");
            }
        }
        else {
            println!("Invalid input.");
        }
    };
}

fn read_input(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn play(item_me: usize) -> usize{
    let rps: [&str;3] = ["Rock", "Paper", "Scissors"];
    println!("You picked {}", rps[item_me-1]);
    println!("Your opponent picks..\n");
    thread::sleep(Duration::from_secs(2));
    let mut rng = rand::thread_rng(); 
    let item_ai = rng.gen_range(1..=3);
    println!("{}!\n",rps[item_ai-1]);
    return compare(item_me-1, item_ai-1)
}

fn compare(item_me: usize, item_ai: usize) -> usize {
    if item_me == item_ai {
        return 2
    }
    else if item_ai > item_me && item_me != 1 && item_ai != 3 {
        return 0
    }
    else if item_me > item_ai && item_ai != 1 && item_me != 3 {
        return 1
    }
    else if item_me == 1 && item_ai == 3 {
        return 1
    }
    else {
        return 0
    }
}
// test