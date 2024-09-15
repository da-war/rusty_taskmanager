use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::env;
use std::fmt;

#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: usize, description: String) -> Task {
        Task {
            id,
            description,
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.completed { "✓" } else { " " };
        write!(f, "[{}] Task {}: {}", status, self.id, self.description)
    }
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let id = self.tasks.len() + 1;
        let task = Task::new(id, description);
        self.tasks.push(task);
    }

    fn complete_task(&mut self, id: usize) -> Result<(), String> {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                task.complete();
                Ok(())
            }
            None => Err("Task not found".to_string()),
        }
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            println!("{}", task);
        }
    }

    fn save_tasks(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        for task in &self.tasks {
            writeln!(file, "{}|{}|{}", task.id, task.description, task.completed)?;
        }
        Ok(())
    }

    fn load_tasks(&mut self, filename: &str) -> io::Result<()> {
        let file = OpenOptions::new().read(true).open(filename)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 3 {
                let id = parts[0].parse::<usize>().unwrap_or(0);
                let description = parts[1].to_string();
                let completed = parts[2].parse::<bool>().unwrap_or(false);

                let mut task = Task::new(id, description);
                if completed {
                    task.complete();
                }
                self.tasks.push(task);
            }
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut manager = TaskManager::new();
    let filename = "tasks.txt";

    // Load existing tasks from file
    manager.load_tasks(filename).ok();

    if args.len() > 1 {
        match args[1].as_str() {
            "add" => {
                if args.len() < 3 {
                    println!("Usage: add <task description>");
                } else {
                    manager.add_task(args[2..].join(" "));
                }
            }
            "complete" => {
                if args.len() < 3 {
                    println!("Usage: complete <task id>");
                } else {
                    let id = args[2].parse::<usize>().unwrap_or(0);
                    if let Err(e) = manager.complete_task(id) {
                        println!("Error: {}", e);
                    }
                }
            }
            "list" => {
                manager.list_tasks();
            }
            _ => {
                println!("Invalid command. Use 'add', 'complete', or 'list'.");
            }
        }
    } else {
        println!("Usage: <command> [arguments]");
        println!("Commands:");
        println!("  add <task description>  - Add a new task");
        println!("  complete <task id>      - Mark a task as completed");
        println!("  list                    - List all tasks");
    }

    // Save tasks to file before exiting
    manager.save_tasks(filename)
}
