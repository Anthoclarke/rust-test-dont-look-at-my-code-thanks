use std::io::{self,Write};


struct Task {
    name: String,
    completed: bool,
}
impl Task {
    fn add_task() -> Task {  
        let name = read_input("Task name: ");
        Task {
            name,
            completed: false,
        }
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    loop {
        println!("\n1: Add Task");
        println!("2: Remove Task");
        println!("3: Change Task Status");
        println!("4: View Tasks");
        println!("Other: Exit");
        match read_input("Choose an option: ").as_str() {
            "1" => {
                let task = Task::add_task();     
                tasks.push(task)           
            },

            "2" => {
                show_tasks(&tasks);
                let num= read_input("Enter task #: ").parse::<usize>().unwrap();
                tasks.remove(num+1);
            },

            "3" => println!("3"),

            "4" => { show_tasks(&tasks); },
            _ => break
        } 

    }
}

fn show_tasks(tasks: &Vec<Task>) {
    println!("");
    for (num, task) in tasks.iter().enumerate() {
        if num > 0 {
            println!("|\n| {}: {} ({})", num+1, &task.name, if task.completed {"✔️"} else {"❌"});
        }
        else {
            println!("| {}: {} ({})", num+1, &task.name, if task.completed {"✔️"} else {"❌"});
        }
    }
}

fn read_input(text: &str) -> String {
    print!("\n{}", text);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}