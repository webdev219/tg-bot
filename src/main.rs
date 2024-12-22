#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

use std::collections::HashMap;

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score}");
   
    let field_name = String::from("Favorite color");
    let field_value = 32;

    scores.insert(field_name, field_value);
    scores.insert(String::from("Blue"), 25);

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    let text = "hello world wonderful world";

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    for word in text.split_whitespace() {
        let count = scores.entry(word.to_string()).or_insert(0);

        println!("{count}");
        *count += 1;
    }
    println!("{scores:?}");

}