use std::io::{self,Write};
use rand::Rng;

fn main () {
    let mut win1 = 0;
    let mut win2 = 0;
    let mut draw = 0;
    let win: [[usize;3];8] = [[0,1,2],[3,4,5],[6,7,8],[0,3,6],[1,4,7],[2,5,8],[0,4,8],[2,4,6]];
    let mut grid: [Option<char>;9] = [None;9];
    println!("###############");
    println!("#  1 | 2 | 3  #");
    println!("# ---|---|--- #");
    println!("#  4 | 5 | 6  #");
    println!("# ---|---|--- #");
    println!("#  7 | 8 | 9  #");
    println!("###############");
    println!("");
    println!("Tic Tac Toe!");
    println!("\nChoisis ton identifiant (ex.: X, O, @, *, etc.)\n");
    let p1 = player_symbol(1);
    let p2 = player_symbol(2);
    loop {
        println!("");
        let mut rng = rand::thread_rng();
        let first;
        let second;
        if rng.gen_bool(0.5) {
            first = p1;
            second = p2;
        }
        else {
            first = p2;
            second = p1;
        }
        println!("Ordre des tours:\n1: {}\n2: {}", first, second);
        show_board(&grid);
        let mut number: u32 = 0;
        while number < 9{
            if &number % 2 == 0 {
                place(1,first, &mut grid);
            }
            else {
                place(2,second, &mut grid);
            }
            number += 1;
            show_board(&grid);
            match check_for_win(&grid,win) {

                Some(char) if char == first => {
                    println!("\nJ1 ({}) GAGNE!", char);
                    break;
                }

                Some(char) if char == second => {
                    println!("\nJ2 ({}) GAGNE!", char);
                    
                    break;
                }

                _ => {
                    if number == 9 {
                        println!("\nÉgalité!");
                        draw += 1;
                        break;
                    }
                }
            };
        };
        println!("")
        match read_input("Enter pour rejouer, Q pour arrêter: ").to_lowercase().as_str() {
            "q" => { break; },
            _ => {}
        }
    }
}



fn player_symbol(num: usize) -> char {
    loop {
        let plyrnum = format!("J{num}: ");
        let player_char = read_input(&plyrnum);
        match player_char.len() {
            0 => {
                println!("IDÉALEMENT on entrerait qqch")
            },

            1 => {
                return player_char.chars().next().unwrap()
            },

            _ => {
                println!("Yo j'écris PAS un dictionnaire complet, UN symbole c'est vraiment pas difficile")
            }
        }
    }
}

fn read_input(text: &str) -> String{
    print!("{}", text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn show_board(grid: &[Option<char>]) {
    println!("");
    for (i,line) in grid.chunks(3).enumerate() {
        let l = line[0].unwrap_or(' ');
        let m = line[1].unwrap_or(' ');
        let r = line[2].unwrap_or(' ');
        println!(" {} | {} | {}", l, m, r);
        if i < 2 {
            println!("---|---|---");
        }

    };
}

fn place(turn_num: u8, plr_char: char, grid: &mut [Option<char>]) {
    let prompt = format!("\nJ{} ({}): Quelle case? ", turn_num, plr_char);
    let cell_num: usize = loop {
        if let Ok(cell) = read_input(&prompt).parse::<usize>() && (1..=9).contains(&cell) {
            if &grid[cell-1] != &None {
                println!("Cette case est déjà occupée");
                continue;
            }
            else {
                break cell;
            }
        }
        else {
            println!("IDEALEMENT on entrerait la bonne affaire");
            continue;
        }
        
    };
    grid[&cell_num-1] = Some(plr_char);
}

fn check_for_win(grid: &[Option<char>], patterns: [[usize;3];8]) -> Option<char> {
    for [a,b,c] in patterns {
        if grid[a] != None && grid[b] == grid[a] && grid[c] == grid[b] {
            return grid[a]
        }
    };
    return None
}
// test