fn main() {
    let data = include_str!("./day1/day1p2.txt");
    // let possibleDigits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let lines: usize = data.lines().map(p2_calibration_value).sum();

    println!("{} result", lines);
}
fn p1_calibration_value(line: &str) -> usize {
    let d1 = line
        .chars()
        .find(|c| c.is_numeric())
        .unwrap();
    let d2 = line
        .chars()
        .rev()
        .find(|c| c.is_numeric())
        .unwrap();
    format!("{d1}{d2}").parse().unwrap()
}
fn p2_calibration_value(line: &str) -> usize {
    let line = line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    p1_calibration_value(&line)
}
