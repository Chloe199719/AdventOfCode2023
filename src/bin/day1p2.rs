use std::collections::HashMap;
fn main() {
    let data = include_str!("./day1/day1p2.txt");
    // let possibleDigits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let map = word_to_digit();
    let lines = data.lines().collect::<Vec<&str>>();
    let mut sum = 0;
    for line in lines.iter() {
        let digits = replace_spelled_numbers(&line, &map);
        println!("{}", digits);
        let mut stringResult = String::new();
        stringResult.push(digits.chars().nth(0).unwrap());
        stringResult.push(
            digits
                .chars()
                .nth(digits.len() - 1)
                .unwrap()
        );
        // println!("{}", stringResult);
        sum += stringResult.parse::<i32>().unwrap();
        // println!("{}", sum);
    }

    println!("{} result", sum);
}

fn word_to_digit() -> HashMap<&'static str, char> {
    let mut map = HashMap::new();
    map.insert("one", '1');
    map.insert("two", '2');
    map.insert("three", '3');
    map.insert("four", '4');
    map.insert("five", '5');
    map.insert("six", '6');
    map.insert("seven", '7');
    map.insert("eight", '8');
    map.insert("nine", '9');
    map.insert("zero", '0');
    map
}
fn replace_spelled_numbers(input: &str, map: &HashMap<&str, char>) -> String {
    let mut result = String::new();
    let mut word = String::new();

    for ch in input.chars() {
        if ch.is_alphabetic() {
            word.push(ch);
            if let Some(&digit) = map.get(word.as_str()) {
                result.push(digit);
                word.clear();
            }
        } else {
            // Check if the word is a valid number before clearing it
            if !word.is_empty() && map.contains_key(word.as_str()) {
                result.push(map[word.as_str()]);
            }
            word.clear();
            result.push(ch);
        }
    }

    // Check for a remaining valid number word at the end of the input
    if !word.is_empty() && map.contains_key(word.as_str()) {
        result.push(map[word.as_str()]);
    }

    result
        .chars()
        .filter(|c| c.is_digit(10))
        .collect()
}
