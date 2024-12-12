mod avl_tree;

use crate::avl_tree::AVLTree;
use std::io::{self, Write};


fn main() {
    let mut tree: AVLTree<i32, String> = AVLTree::new();
    println!("Добро пожаловать в консольный интерфейс AVL-дерева!");
    println!("Доступные команды:");
    println!("  insert <ключ> <значение>  - Вставить элемент");
    println!("  remove <ключ>             - Удалить элемент");
    println!("  find <ключ>               - Найти значение по ключу");
    println!("  display                   - Вывести все элементы в порядке возрастания ключей");
    println!("  exit                      - Выйти из программы");

    loop {
        print!("\nВведите команду: ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        if let Err(err) = io::stdin().read_line(&mut input) {
            println!("Ошибка чтения ввода: {}", err);
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            println!("Пустая команда. Пожалуйста, введите команду.");
            continue;
        }

        match parts[0].to_lowercase().as_str() {
            "insert" => {
                if parts.len() < 3 {
                    println!("Недостаточно аргументов для команды insert. Использование: insert <ключ> <значение>");
                    continue;
                }
                let key: i32 = match parts[1].parse() {
                    Ok(k) => k,
                    Err(_) => {
                        println!("Некорректный ключ. Ключ должен быть целым числом.");
                        continue;
                    }
                };

                let value = parts[2..].join(" ");
                tree.insert(key, value);
                println!("Элемент с ключом {} вставлен/обновлен.", key);
            }
            "remove" => {
                if parts.len() != 2 {
                    println!(
                        "Недостаточно аргументов для команды remove. Использование: remove <ключ>"
                    );
                    continue;
                }
                let key: i32 = match parts[1].parse() {
                    Ok(k) => k,
                    Err(_) => {
                        println!("Некорректный ключ. Ключ должен быть целым числом.");
                        continue;
                    }
                };
                if tree.remove(&key) {
                    println!("Элемент с ключом {} удален.", key);
                } else {
                    println!("Элемент с ключом {} не найден.", key);
                }
            }
            "find" => {
                if parts.len() != 2 {
                    println!(
                        "Недостаточно аргументов для команды find. Использование: find <ключ>"
                    );
                    continue;
                }
                let key: i32 = match parts[1].parse() {
                    Ok(k) => k,
                    Err(_) => {
                        println!("Некорректный ключ. Ключ должен быть целым числом.");
                        continue;
                    }
                };
                match tree.find(&key) {
                    Some(value) => println!("Значение для ключа {}: {}", key, value),
                    None => println!("Элемент с ключом {} не найден.", key),
                }
            }
            "display" => {
                println!("Элементы AVL-дерева (в порядке возрастания ключей):");
                tree.inorder_traversal(|k, v| println!("  Ключ: {}, Значение: {}", k, v));
            }
            "exit" => {
                println!("Йоу, вассап, заходите еще!");
                break;
            }
            _ => {
                println!(
                    "Неизвестная команда: {}. Пожалуйста, используйте одну из доступных команд.",
                    parts[0]
                );
            }
        }
    }
}
