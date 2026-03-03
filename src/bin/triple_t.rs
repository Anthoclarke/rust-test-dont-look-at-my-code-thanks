use std::io::{self,Write};
use rand::Rng;
struct Game {
    grid: [Option<char>;9],
    p1: char,
    p2: char,
    turn: bool,
}

impl Game {

    fn new() -> Self {
        Self {
            grid: [None;9],
            p1: set_char("J1: "),
            p2: set_char("J2: "),
            turn: randomize_first_turn(),
        }
    }
    fn show_board(&self) {
        for (i,chunk) in self.grid.chunks(3).enumerate() {
            let l =  chunk[0].unwrap_or(' ');
            let m =  chunk[1].unwrap_or(' ');
            let r =  chunk[2].unwrap_or(' ');
            println!(" {} | {} | {} ", l, m, r);
            if i < 2 {
                println!("---|---|---");
            }
        }
    }
    fn place(&mut self) {
        loop {
            if let Ok(cell) = read_input("Jn° de case: ").parse::<usize>() {
                
            }
        }
    }
}

fn main() {
    let game = Game::new();
    game.show_board();
    println!("{}",read_input("yup: "))
}


fn read_input(prompt: &str) -> String {
    print!("{}",prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn set_char(prompt: &str, ) -> char {
    loop {
        if let Ok(char) = read_input(prompt).parse::<char>() {
            return char;
        }
        else {
            println!("Yo genre une seule affaire j'écris pas tout ça");
        }
    }
}

fn randomize_first_turn() -> bool {
    if rng.gen_bool(0.5) {
        return true
    }
    else {
        return false
    }
}