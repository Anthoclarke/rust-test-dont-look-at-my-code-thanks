use std::io::{self,Write};
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    completed: bool,
}

fn main() {
    let contents = match fs::read_to_string("tasks.json") {
        Ok(text) => text,
        Err(_) => String::from("[]"),
    };
    let mut tasks: Vec<Task> = match serde_json::from_str(&contents) {
        Ok(vec) => vec,
        Err(_) => Vec::new(),
    };
    loop {
        println!("\n1: Add task");
        println!("2: View tasks");
        println!("3: Remove task");
        println!("4: Toggle completion of task\n");
        match read_input("Choose an option: ").as_str() {
            "1" => {
                let task = Task {
                    name: read_input("Task name: "),
                    completed: false
                };
                tasks.push(task);
                write(&tasks);
            }
            "2" => {
                if tasks.is_empty() {
                    println!("No tasks in here yet!");
                }
                else {
                    show(&tasks);
                }
                write(&tasks);
            }
            "3" => {
                let max_index = tasks.len();
                if max_index == 0 {
                    println!("No tasks in here yet!");
                }
                else {
                    show(&tasks);
                    let input = read_input("\nChoose a task # to delete: ");
                    let index: usize = match input.parse() {
                        Ok(num) => num,
                        Err(_) => { 
                            println!("Invalid number");
                            continue;
                        }
                    };
                    if index < 1 || index > max_index {
                        println!("Invalid Number");
                    }
                    else {
                        println!("Removed {} from task list", tasks[index-1].name);
                        tasks.remove(index-1);
                    write(&tasks)
                    }
                }
            }
            "4" => {
                let max_index = tasks.len();
                if max_index == 0 {
                    println!("No tasks in here yet!");
                }
                else {
                    show(&tasks);
                    let input = read_input("\nChoose a task # to toggle: ");
                    let index: usize = match input.parse() {
                        Ok(num) => num,
                        Err(_) => { 
                            println!("Invalid number");
                            continue;
                        }
                    };
                    if index < 1 || index > max_index {
                        println!("Invalid task in list");
                    }
                    else {
                        let task = &mut tasks[index-1];
                        task.completed = !task.completed;
                    }
                }
                write(&tasks);
            }
            _ => {
                println!("\nSaving and exiting!");
                write(&tasks);
                break;
            }
        }
    }
}

fn write(list: &[Task]) {
    let json = serde_json::to_string_pretty(&list).unwrap();
    std::fs::write("tasks.json", json).unwrap();
    println!("List has been updated!");
}

fn show(tasks: &[Task]) {
    println!("");
    for (i, task) in tasks.iter().enumerate() {
        if i > 0 {
            println!("|\n| {} - {} ({})", i+1, task.name, if task.completed {"✅"} else {"❌"})
        }
        else {
            println!("| {} - {} ({})", i+1, task.name, if task.completed {"✅"} else {"❌"})
        }
    }
}


fn read_input(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();


    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
// test