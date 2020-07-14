use std::collections::HashMap;

fn main() {
    let mut name = String::new();
    let mut names:Vec<String> = Vec::new();


    let mut scores : HashMap <String,u8>= HashMap::new();

    println!("enter your name");
    std::io::stdin().read_line(&mut name).expect("error");
    let clone_name = name.trim().to_string().clone();
    names.push(name.trim().to_string());
    println!("Names vec is {:?}",names);

    


    let number:u8 = 34;
    let number_string = number.to_string();

    let name_plus_nmber:String = format!("{}{}",clone_name,number_string);

    scores.insert(name_plus_nmber,34);

    println!("Score HashMap is : {:?}",scores);



}
