use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    fn new() -> Self {
        TaskList { tasks: Vec::new() }
    }

    fn add_task(&mut self, title: String) {
        let new_id = (self.tasks.len() as u32) + 1;
        self.tasks.push(Task {
            id: new_id,
            title,
            completed: false,
        });
        println!("Завдання було додано.");
    }

    fn show_tasks(&self) {
        for task in &self.tasks {
            println!(
                "[{}] {} - {}",
                task.id,
                task.title,
                if task.completed {
                    "Виконане"
                } else {
                    "Не виконане"
                }
            );
        }
    }

    fn delete_task(&mut self, task_id: u32) {
        self.tasks.retain(|task| task.id != task_id);
        println!("Завдання було видалено.");
    }

    fn edit_task(&mut self, task_id: u32, new_title: String) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == task_id) {
            task.title = new_title;
            println!("Завдання було змінено.");
        } else {
            println!("Вказане завдання не знайдено");
        }
    }

    fn complete_task(&mut self, task_id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == task_id) {
            task.completed = true;
            println!("Завдання було відмічено як виконане.");
        } else {
            println!("Вказане завдання не знайдено.");
        }
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self)?;
        println!("Завдання було збережено.");
        Ok(())
    }

    fn load_from_file(filename: &str) -> io::Result<Self> {
        if !Path::new(filename).exists() {
            return Ok(TaskList::new());
        }
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let task_list = serde_json::from_reader(reader)?;
        Ok(task_list)
    }
}

fn main() -> io::Result<()> {
    let mut task_list = TaskList::load_from_file("tasks.json")?;
    loop {
        println!("Оберіть наказ: ");
        println!("1. Додати завдання.");
        println!("2. Відобразити завдання.");
        println!("3. Вилучити завдання");
        println!("4. Змінити завдання");
        println!("5. Відмітити як виконане.");
        println!("6. Завершити роботу.");

        let mut command = String::new();
        io::stdin().read_line(&mut command)?;
        let command = command.trim();

        match command {
            "1" => {
                println!("Впишіть назву завдання: ");
                let mut title = String::new();
                io::stdin().read_line(&mut title)?;
                task_list.add_task(title.trim().to_string());
            }
            "2" => {
                task_list.show_tasks();
            }
            "3" => {
                println!("Вкажіть число завдання, що хочете вилучити: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id)?;
                let id: u32 = id.trim().parse().unwrap_or(0);
                task_list.delete_task(id);
            }
            "4" => {
                println!("Вкажіть число завдання, що хочете змінити: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id)?;
                let id: u32 = id.trim().parse().unwrap_or(0);
                println!("Введіть нову назву: ");
                let mut new_title = String::new();
                io::stdin().read_line(&mut new_title)?;
                task_list.edit_task(id, new_title.trim().to_string());
            }
            "5" => {
                println!("Введіть число завдання, що хочете відмітити, як виконане: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id)?;
                let id: u32 = id.trim().parse().unwrap_or(0);
                task_list.complete_task(id);
            }
            "6" => {
                task_list.save_to_file("tasks.json")?;
                println!("Завершення");
                break;
            }
            _ => {
                println!("Невідомий наказ, введіть повторно.");
            }
        }
    }
    Ok(())
}
