use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::io::stdin;

#[derive(Serialize, Deserialize)]
struct Task {
    title: String,
    is_check: bool,
    description: String,
}

fn main() {
    let mut tasks: Vec<Task>;
    let content_file = read_file();
    let json_value: Result<Vec<Task>, serde_json::Error> = serde_json::from_str(&content_file);

    match json_value {
        Ok(json_tasks) => {
            tasks = json_tasks;
        }
        Err(_) => {
            tasks = Vec::new();
        }
    }

    loop {
        let mut option = String::new();

        println!("---------- Bienvenido al gestor de tareas en CLI ----------");
        println!("Escribe la opción que desees:");
        println!("1. Agregar una tarea");
        println!("2. Listar tareas");
        println!("3. Salir");

        stdin()
            .read_line(&mut option)
            .expect("Error al leer la entrada");

        let option = option.trim().parse::<i32>().unwrap_or(0);

        match option {
            1 => {
                let mut title = String::new();
                let mut description = String::new();

                println!("Ingresa el título de la tarea: ");
                stdin()
                    .read_line(&mut title)
                    .expect("Error al leer la entrada");
                println!("Ingresa la descripción de la tarea: ");
                stdin()
                    .read_line(&mut description)
                    .expect("Error al leer la entrada");

                let task: Task = Task {
                    title,
                    is_check: false,
                    description,
                };

                tasks.push(task);

                let serialized = serde_json::to_string(&tasks).unwrap();

                fs::write("task_list.json", serialized).unwrap();
            }
            2 => {
                let mut task_number = 0;
                for task in tasks.iter() {
                    task_number += 1;
                    println!("+----------- Tarea #{} -----------+", task_number);
                    println!("Título: {}", task.title);
                    println!("Descripción: {}", task.description);
                    println!("+--------------------------------+");
                }
            }
            3 => {
                break;
            }
            _ => {
                println!("Opción inválida")
            }
        }
    }
}

fn read_file() -> String {
    let content = fs::read_to_string("task_list.json");

    match content {
        Ok(content) => {
            return content;
        }
        Err(_) => {
            fs::write("task_list.json", "").unwrap();
            return String::from("");
        }
    }
}
