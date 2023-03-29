struct Task {
    id: u32,
    action: String,
    completed: bool,
}

fn create_task(action: String, tasks: &mut Vec<Task>) -> () {
    tasks.push(Task {
        id: tasks.len() as u32,
        action: action,
        completed: false,
    })
}

fn display_tasks(vec: &Vec<Task>, list_view: bool) -> () {
    println!("\n\nAll tasks:\n");
    if list_view {
        for task in vec {
            println!(
                "{}.Task: {}Completed {}\n",
                task.id, task.action, task.completed
            );
        }
    } else {
        for task in vec {
            println!("Task: {}Completed {}\n", task.action, task.completed);
        }
    }
}

fn mark_task_completed(vec: &mut Vec<Task>, id: u32) -> () {
    for task in vec.iter_mut() {
        if task.id == id {
            task.completed = true
        }
    }
}

fn main() {
    use std::io::{stdin, stdout, Write};
    let mut tasks: Vec<Task> = Vec::new();
    println!("Hello Welcome to the Todo App\n\n Please Select an option");
    println!("1. Create a new task");
    println!("2. Display all tasks");
    println!("3. Mark a task as completed");
    println!("4. Exit");

    loop {
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Enter a valid input");
        if input.trim() == "1" {
            println!("Enter the task you want to add");
            let _ = stdout().flush();
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Enter a valid input");
            create_task(input, &mut tasks);
        } else if input.trim() == "2" {
            display_tasks(&tasks, false);
        } else if input.trim() == "3" {
            println!("Enter the task you want to mark as completed");
            display_tasks(&tasks, true);
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Enter a valid input");
            match input.trim().parse::<u32>() {
                Ok(id) => mark_task_completed(&mut tasks, id),
                Err(_) => println!("Please enter a valid input"),
            }
        } else if input.trim() == "4" {
            println("Exiting the app");
            break;
        } else {
            println!("Please enter a valid input");
        }
        println!("\n\n1. Create a new task");
        println!("2. Display all tasks");
        println!("3. Mark a task as completed");
        println!("4. Exit");
    }
}
