use std::io::{self,Write};
use rand::Rng;

struct Game {
    grid: [Option<char>;9],
    p1: char,
    p2: char,
    turn: bool,
    turns: u8
}

impl Game {

    fn new() -> Self {
        Self {
            grid: [None;9],
            p1: set_char("J1: "),
            p2: set_char("J2: "),
            turn: randomize_first_turn(),
            turns: 0,
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
        let text = format!("J{} ({}) # de case: ", if self.turn {1} else {2}, if self.turn {self.p1} else {self.p2});
        let cell: usize = loop {
            if let Ok(cell) = read_input(&text).parse::<usize>() && (1..=9).contains(&cell) {
                if self.grid[cell-1] == None {
                    break cell;
                }
                else {
                    println!("Case déjà occupée");
                    continue;
                }
            }
            else {
                println!("Yo un numero entre 1 et 9 c'est jamais si difficile");
            }
        };
        self.grid[&cell-1] = Some(if self.turn {self.p1} else {self.p2});
        self.turn = !self.turn;
        self.show_board();
    }
    fn check_for_win(&self) -> bool {
        let patterns: [[usize; 3]; 8] = [[0,1,2],[0,3,6],[0,4,8],[3,4,5],[6,7,8],[1,4,7],[2,5,8],[2,4,6]];
        for [a,b,c] in patterns {
            if self.grid[a] != None && self.grid[b] == self.grid[a] && self.grid[c] == self.grid[b] {
                if let Some(char) = self.grid[a] {
                    println!("Joueur {} ({}) Gagne!", if self.turn {1} else {2}, char);
                    return true;
                }
            }
        }
        return false;
    }
}

fn main() {
    let mut game = Game::new();
    game.show_board();
    while game.turns < 9 {
        game.turns += 1;
        game.place();
        if game.check_for_win() {
            break;
        }
        else {
            if game.turns == 9 {
                println!("Égalité.");
            }
        }
    }
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
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        return true
    }
    else {
        return false
    }
}