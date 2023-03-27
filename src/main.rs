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
//     let mut data = HashMap::new();
//     loop { 
//         let mut choice = String::new();      
//     println!("Хотите добавить сотрудника? - Y ,
//     Список сотрудников в алф.порядке- L , 
//     Найти всех из одного отдела - S, 
//     Выход - E");
//          io::stdin()
//         .read_line(&mut choice)
//         .expect("Не ясен ввод");
//         println!("Вы ввели {}",choice);
//             match choice.trim() 
//             {
//                 "Y" => add_person(&mut data),
//                 "L" => list_by_alph(& data),
//                 "S" => by_department(&mut data),
//                 "E" |"e"  => { println!("Вам всего доброго хорошего настроения держитесь здесь...")
//                 ; break },
//                 _ => {println! ("Неверная команда"); }, 
//             }
//         }
//     fn add_person(data: &mut HashMap<String, String>) { 
//         let mut name = String::new();
//         println!("Введите имя сотрудника:");  
//          io::stdin()
//         .read_line(&mut name)
//         .expect("Не ясен ввод");
//         let mut department = String::new();
//         println!("Введите название отдела:");
//         io::stdin()
//         .read_line(&mut department)
//         .expect("Не ясен ввод");
//         data.insert(String::from(name.trim().to_string()),String::from(department.trim().to_string()));
//     }    
//     fn list_by_alph(data: &HashMap<String, String>) {
//         let mut names = Vec::new();
//         for (name, department) in data {
//         names.push(name);
//         names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase())); 
//     }
//     // println!("{:?}", &names);
//     for person in names {
//         println!("{}",person);
//     }
// }
// fn by_department(data: &mut HashMap<String, String>) {
//     let mut department_check = String::new();
//     println!("Введите название отдела:");  
//          io::stdin()
//         .read_line(&mut department_check)
//         .expect("Не ясен ввод");
//         department_check.trim().to_string();

//         for (name, department) in data {
//              if department == &department_check.trim().to_string() 
//             {
//             println!("В этом отделе: {name}");
//             }
//             // else 
//             // {
//             // println!("Не найдено в отделе {}", department_check);
//             // }
//         }
//     }
//     }
// fn main() {
//     use std::fs::File;
//     use std::io::{self, Read};
    
//     fn read_username_from_file() -> Result<String, io::Error> {
//         let username_file_result = File::open("hello.txt");
    
//         let mut username_file = match username_file_result {
//             Ok(file) => file,
//             Err(e) => return Err(e),
//         };
    
//         let mut username = String::new();
    
//         match username_file.read_to_string(&mut username) {
//             Ok(_) => Ok(username),
//             Err(e) => Err(e),
//         }
//     }
//     }
// use std::cmp::PartialOrd;

// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for i in list {
//         if i > largest {
//             largest = i;
//         }
//     }
//     largest
// } 

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65, 3000];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// codewars
// Complete the solution so that it returns true if the first argument(string) passed in ends with the 2nd argument (also a string).
// solution('abc', 'bc') // returns true
// solution('abc', 'd') // returns false

fn main() {
    let phrase1 = "abra ra";
    // let phrase2 = "abra we";
    let vec_from_phrase: Vec<&str> = phrase1.split(' ').collect();
    if vec_from_phrase[0].ends_with(vec_from_phrase[1])
    { println!("condition was true");  } 
    else
    { println!("condition was false"); }
}

// realsolution
// fn main() {
// let phrase1 = "abra ra";
// let vec_from_phrase: Vec<&str> = phrase1.split(' ').collect();
// assert!(vec_from_phrase[0].ends_with(vec_from_phrase[1]));
// }
// codewars solved
// fn solution(word: &str, ending: &str) -> bool {
//     if word.ends_with(ending)
//     { true }
//     else
//     { false }
// }
// }
