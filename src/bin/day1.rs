fn main() {
    let data = include_str!("./day1/day1p1.txt");
    // create  regex to filter all but numbers

    let x = data
        .lines()
        .map(|x|
            x
                .chars()
                .filter(|x| x.is_numeric())
                .collect::<String>()
        )
        .collect::<Vec<String>>();
    let mut sum = 0;
    for i in x {
        let xsplit = i
            .trim()
            .split("")
            .collect::<Vec<&str>>()
            .iter()
            .filter(|x| x.to_string() != "")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let mut newVec = vec![];
        newVec.push(xsplit.first().unwrap().clone());
        newVec.push(xsplit.last().unwrap().clone());
        let value = newVec.join("");
        let value = value.parse::<i32>().unwrap();
        sum += value;
    }
    println!("{}", sum)
}
