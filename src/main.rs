use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    process::Command,
};

fn clear() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/c")
            .arg("cls")
            .status()
            .expect("Gagal");
    } else {
        Command::new("clear").status().expect("Gagal");
    }
}

struct Task {
    description: String,
    status: char,
}

impl Task {
    fn new(description: &str) -> Task {
        Task {
            description: description.to_string(),
            status: '❌',
        }
    }
}

fn data() -> HashMap<i32, Task> {
    let mut list = HashMap::new();
    list.insert(1, Task::new("Tambahkan disini..."));
    list
}
fn print_tasks(list: &HashMap<i32, Task>) {
    let mut sort: Vec<i32> = list.keys().copied().collect();
    sort.sort();
    let mut i = 0;
    for key in sort {
        if let Some(task) = list.get(&key) {
            i += 1;
            println!("{}. {} [{}]", i, task.description, task.status);
        }
    }
}
fn check(list: &HashMap<i32, Task>, nolist: i32) -> bool {
    list.contains_key(&nolist)
}

fn main() {
    let mut list = data();
    loop {
        clear();
        println!("==========---AppTodo---==========");
        println!("-------------Welcome-------------\n");
        print_tasks(&list);
        println!("\n[+] untuk menambah list");
        print!("pilih salah satu (0 untuk Exit) : ");
        stdout().flush().expect("Gagal");
        let mut choice = String::new();
        stdin().read_line(&mut choice).expect("Gagal");

        if choice.trim() == "+" {
            clear();
            print!("Masukkan deskripsi task baru: ");
            stdout().flush().expect("Failed to flush stdout");

            let mut new_description = String::new();
            stdin()
                .read_line(&mut new_description)
                .expect("Failed to read line");

            let new_task = Task::new(new_description.trim());
            let new_key = list.len() as i32 + 1;
            list.insert(new_key, new_task);
        }
        if let Ok(choice) = choice.trim().parse::<i32>() {
            if choice == 0 {
                clear();
                println!("cancel");
                break;
            }
            if check(&list, choice) {
                println!("1. Edit");
                println!("2. Delete");
                println!("3. Done");
                println!("0. Cancel");

                print!("Pilih Opsi : ");
                stdout().flush().expect("Gagal");
                let mut option = String::new();
                stdin().read_line(&mut option).expect("gagal");
                match option.trim() {
                    "1" => {
                        clear();
                        print!("Masukan descripsi baru :  ");
                        stdout().flush().expect("Gagal baca");
                        let mut new_description = String::new();
                        stdin().read_line(&mut new_description).expect("gagal");
                        if let Some(task) = list.get_mut(&choice) {
                            task.description = new_description.trim().to_string();
                        }
                    }
                    "2" => {
                        clear();
                        list.remove(&choice);
                        let mut new_map = HashMap::new();
                        let mut new_key = 1;
                        for (_old_key, task) in list {
                            new_map.insert(new_key, task);
                            new_key += 1;
                        }
                        list = new_map;
                        println!("Task {} berhasil dihapus ", choice);
                    }
                    "3" => {
                        clear();
                        if let Some(task) = list.get_mut(&choice) {
                            task.status = '✅';
                            println!("Status task {} berhasil diubah.", choice);
                        }
                    }
                    "0" => {
                        clear();
                        println!("Cancel");
                    }
                    _ => {
                        clear();
                        println!("Opsi tidak valid");
                    }
                }
            } else {
                clear();
                println!("List tidak ditemukan");
            }
        }
    }
}
