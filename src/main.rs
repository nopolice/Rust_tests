// Используя хеш-карту и векторы, создайте текстовый интерфейс позволяющий пользователю 
// добавлять имена сотрудников к названию отдела компании. 
// Например, "Add Sally to Engineering" или "Add Amir to Sales". 
// Затем позвольте пользователю получить список всех людей из отдела 
// или всех людей в компании отсортированным в алфавитном порядке по отделам.

use std::io;
use std::collections::HashMap;

fn main() {
    let mut data = HashMap::new();
    loop { 
        let mut choice = String::new();      
    println!("Хотите добавить сотрудника? - Y ,
    Список сотрудников в алф.порядке- L , 
    Найти всех из одного отдела - S, 
    Выход - E");
         io::stdin()
        .read_line(&mut choice)
        .expect("Не ясен ввод");
        println!("Вы ввели {}",choice);
            match choice.trim() 
            {
                "Y" => add_person(&mut data),
                "L" => list_by_alph(& data),
                "S" => by_department(&mut data),
                "E" |"e"  => { println!("Вам всего доброго хорошего настроения держитесь здесь...")
                ; break },
                _ => {println! ("Неверная команда"); }, 
            }
        }
    fn add_person(data: &mut HashMap<String, String>) { 
        let mut name = String::new();
        println!("Введите имя сотрудника:");  
         io::stdin()
        .read_line(&mut name)
        .expect("Не ясен ввод");
        let mut department = String::new();
        println!("Введите название отдела:");
        io::stdin()
        .read_line(&mut department)
        .expect("Не ясен ввод");
        data.insert(String::from(name.trim().to_string()),String::from(department.trim().to_string()));
    }    
    fn list_by_alph(data: &HashMap<String, String>) {
        let mut names = Vec::new();
        for (name, department) in data {
        names.push(name);
        names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase())); 
    }
    // println!("{:?}", &names);
    for person in names {
        println!("{}",person);
    }
}
fn by_department(data: &mut HashMap<String, String>) {
    let mut department_check = String::new();
    println!("Введите название отдела:");  
         io::stdin()
        .read_line(&mut department_check)
        .expect("Не ясен ввод");
        department_check.trim().to_string();

        for (name, department) in data {
             if department == &department_check.trim().to_string() 
            {
            println!("В этом отделе: {name}");
            }
            // else 
            // {
            // println!("Не найдено в отделе {}", department_check);
            // }
        }
    }
    }