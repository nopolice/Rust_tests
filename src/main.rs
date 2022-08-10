// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;

// fn main() {
//     println!("guess the number");
//     let secret_number = rand::thread_rng().gen_range(1..101);
//     // println!("Your secret number: {}",secret_number);
//    loop { 
//         println!("input your guess...");
//         let mut guess = String::new();

//          io::stdin()
//           .read_line(&mut guess)
//         .expect("Failed to read the line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         println!("You guessed {}", guess);

//         match guess.cmp(&secret_number) {
//         Ordering::Less => println!("Too small!"),
//         Ordering::Greater => println!("Too big!"),
//         Ordering::Equal => { println!("You win!");
//         break;
//             }
//      }
//     }   
// }

// fn main() {
//     let mut v = Vec::new();

//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);
// }

// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     println!("The third element is {}", third);

//     match v.get(2) {
//         Some(third) => println!("The third element is {}", third),
//         None => println!("There is no third element."),
//     }
// }
// fn main() {
//     let mut v = vec![100, 32, 57];
//     for i in &mut v {
//         *i += 50;
//         println!("{}", i);
//     }
// }
// // 
// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

// let row = vec![
//     SpreadsheetCell::Int(3),
//     SpreadsheetCell::Text(String::from("blue")),
//     SpreadsheetCell::Float(10.12),
// ];
// // 
// fn main() {
//     use std::collections::HashMap;

//     let text = "hello world wonderful world";

//     let mut map = HashMap::new();

//     for word in text.split_whitespace() {
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }

//     println!("{:?}", map);
// }
// 
// use std::collections::HashMap;

// fn main() {
//     let mut v = vec![1,14,55,34,222,14,120,9,7,15,15,15];
//     v.sort();
//     println!("vec={:?}",v);
//     let len = v.len();
//     // let _i = 0..len;
//     let mut map = HashMap::new();
//         for i in &v
//         {
//         let count = map.entry(i).or_insert(0);
//         *count += 1;
//     }
//     for (key, value) in &map {
//     if value >= &2 
//     {
//         println!("это значение встречается чаще 2 раз: {key}");
//     }
// }
//         println!("{:?}", map);
//         println!("vector length is {}",len);
//         let sum: usize = v.iter().sum();
//         let average = sum / len;
//         let median = &v[len/2];
//        println!("sum {} average {} median {}",sum,average,median);
// }
// 

// Преобразуйте строку в кодировку "поросячьей латыни" (Pig Latin), 
// где первая согласная каждого слова перемещается в 
// конец и к ней добавляется окончание "ay". Например "first" 
// в поросячьей латыни станет "irst-fay". Если слово начинается 
// на гласную, то в конец слова добавляется суффикс 
// "hay" ("apple" становится "apple-hay"). 
// Помните о деталях работы с кодировкой UTF-8!


// fn main() { 
//     let phrase = "Преобразуйте строку в кодировку ололо блаблаы йеее";
//     let vec_from_phrase: Vec<&str> = phrase.split(' ').collect();
//     let mut result = String::new();
//     let constontants = "бвгджзйклмнпПрстфхцчшщ";
//     let vowels = "аоэеиыуёюя";
//         for word in &vec_from_phrase {
//             for c in constontants.chars() 
//             {
//             if word.starts_with(c) 
//                 {
//                 let check = word.trim_start_matches(c);
//                 let remake = check.to_owned()+&(c.to_string())+"ау";
//                 result.push_str(&(' '.to_string() +&remake));
//                 }
//             }
//             for v in vowels.chars() 
//             {
//                 if word.starts_with(v) 
//                 {
//                 result.push_str(&(' '.to_string() + &word.to_string()+"фау"));
//                 }
//             }
//         }
//         println!("{}", result);
//     }

// Используя хеш-карту и векторы, создайте текстовый интерфейс позволяющий пользователю 
// добавлять имена сотрудников к названию отдела компании. 
// Например, "Add Sally to Engineering" или "Add Amir to Sales". 
// Затем позвольте пользователю получить список всех людей из отдела 
// или всех людей в компании отсортированным в алфавитном порядке по отделам.
// use std::io;
// use std::collections::HashMap;

// fn main() {  
//     struct Person {
//         name: String,
//         department: String,
//     }
//     let mut data = HashMap::from(Person::name,Person::department);
//     // let mut spiska = HashMap::from([(&name,&department)]);
//     loop {
//             println!("      
//     Хотите добавить сотрудника? - Y ,
//     Список сотрудников в алф.порядке- L , 
//     Найти всех из одного отдела - S, 
//     Выход - E");
//     let mut choice = String::new();
//          io::stdin()
//         .read_line(&mut choice);
//         println!("Вы ввели {}",choice);
//             match choice.trim() 
//                 {
//                 "Y" => Person::add_person(),
//                 "L" => Person::list_by_alph(),
//                 "S" => Person::by_department(),
//                 "E" |"e"  => { println!("Вам всего доброго хорошего настроения держитесь здесь...");
//                 // }
//                 break; }
//                 _ => println!("Неверная команда"), 
//                 };
//     } 
//         impl Person { 
//     fn add_person(&self) -> <String, String> {
//         // let mut name = String::new();
//         // let mut department = String::new();
//         // let mut data = HashMap::new();
//         println!("Введите имя сотрудника:");  
//          io::stdin()
//         .read_line(&mut self.name)
//         .expect("Failed to read the line");
//         println!("Введите название отдела:");    
//         io::stdin()
//         .read_line(&mut self.department)
//         .expect("Failed to read the line");
//         // data.insert(String::from(name),String::from(department));
//         // for (name, department) in &data {
//             println!("Вы добавили сотрудника: {} в отдел: {}", self.name, self.department);
//         // }
//         // println!("{:?}",data);
//         }
//     fn list_by_alph() {
//         println!("по алфавиту");
//         }
//     fn by_department() {
//         println!("отдел");
//     }
//     }
// }

    


// tut!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!


// Например, "Add Sally to Engineering" или "Add Amir to Sales". 
// Затем позвольте пользователю получить список всех людей из отдела 
// или всех людей в компании отсортированным в алфавитном порядке по отделам.
// _____________________________________________________________

// }
// add_person(&mut data);


// }
// }
// }
// use std::io;
// use std::collections::HashMap;

// // let mut data = HashMap::new();

// struct Person {
//     name: String,
//     department: String,
// }
// impl Person {
//     fn add_person(&self) {
//         // let name = "Imya";
//         // let department = "Otdel";
//         // data.insert(String::from(name),String::from(department));
//         println! ("{} {}",self.name,self.department)
//     }
// }
//     fn main() {
//         let mut person = Person{name: "Hania".to_string(), department: 23.to_string()};
//         println!("\n :");
//         person.add_person();
//     }



// struct Person {
//     name: String,
//     age: u32
//   }
  
//   // Implementing functionality on the Person struct with the impl keyword
//   impl Person{
//     // This method is used to introduce a person
//     fn introduction(&self) {
//       println!("Hello! My name is {} and I am {} years old.", self.name, self.age);
//     }
  
//     // This method updates the age of the person on their birthday
//     fn birthday(&mut self){
//       self.age = self.age + 1
//     }
//   }
  
//   fn main() {
//     // Instantiating a mutable Person object
//     let mut person = Person{name: "Hania".to_string(), age: 23};
    
//     // person introduces themself before their birthday
//     println!("Introduction before birthday:");
//     person.introduction();
    
//     // person ages one year on their birthday
//     person.birthday();
    
//     // person introduces themself after their birthday
//     println!("\nIntroduction after birthday:");
//     person.introduction();
//   }
// use std::io;
// use std::collections::HashMap;
// fn main() {
//         // let mut data = HashMap::new();
//         // data.insert(String::from(name.trim()),String::from(department.trim()));
//         loop {
//         println!("Хотите добавить сотрудника? - Y ,
//     Список сотрудников в алф.порядке- L , 
//     Найти всех из одного отдела - S, 
//     Выход - E");
//     let mut choice = String::new();
//         io::stdin()
//         .read_line(&mut choice);
//         println!("Вы ввели {}",choice);
//             match choice.trim() 
//             {
//                 "Y" |"y" => add_person(),
//                 // "L" => list_by_alph(),
//                 // "S" => by_department(),
//                 "E" |"e"  => { println!("Вам всего доброго хорошего настроения держитесь здесь...");
//                 break; }
//                 _ => println!("Неверная команда"), 
//             };
//         }

//         fn add_person() {
//         let mut data = HashMap::new();
//         let mut name = String::new();
//             println!("Введите имя сотрудника:");  
//              io::stdin()
//             .read_line(&mut name)
//             .expect("Failed to read the line");
//             println!("Введите название отдела:");    
//         let mut department = String::new();
//             io::stdin()
//             .read_line(&mut department)
//             .expect("Failed to read the line");
//             data.insert(String::from(name.trim()),String::from(department.trim()));
//         //     for (name, department) in &data {
//                 println!("Вы добавили сотрудника: {} в отдел: {}", name, department);
//                 println!("{:?}", data);
//                 //         }
//             }
//             println!("Вы добавили сотрудника: {} в отдел: {}", name, department);
//     }
// fn main() {
//         let mut scores = HashMap::new();
//         another_func();
//         // scores.insert(String::from("Blue"), 10);
// fn another_func() {
//     scores.insert(String::from("Blue"), 10);
// }

//     for (key, value) in &scores {
//         println!("{}: {}", key, value);
//     }
// // }
// #![allow(unused)]
// fn main() {
// use std::collections::HashMap;
// use std::io;
// // random_stat_buff(mut key: String,mut value: String);
// let mut player_stats = HashMap::new();
// // let mut key = String::new();
// // let mut value = String::new();
// fn random_stat_buff() -> String {
//         let mut key = String::new();
//                     println!("Введите имя сотрудника:");  
//                      io::stdin()
//                     .read_line(&mut key);
//                     key.trim().to_string();
//                 //     .push_str(key)            
        
//         let mut value = String::new();
//                         println!("Введите название отдела:");
//                         io::stdin()
//                         .read_line(&mut value);
//                         value.trim().to_string()
//         //                 .push_str(&value);

                    
// }

// // insert a key only if it doesn't already exist
// // println!(" ________ {}_{}",key,value);

// // insert a key using a function that provides a new value only if it
// // doesn't already exist
// player_stats.entry("privet").or_insert_with(random_stat_buff);
// // player_stats.insert(key, value);

// println!("{:?}",player_stats)
// }
// use std::collections::HashMap;

// fn add(h: &mut HashMap<&str, &str>) {
//         h.insert("foo","bar");
// }
// fn main() {
//         let mut h: HashMap<&str, &str> = HashMap::new();
//         add(&mut h);
//         println!("{:?}", h);
// }
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