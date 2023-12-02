fn main() {
    const RED: i32 = 12;
    const GREEN: i32 = 13;
    const BLUE: i32 = 14;
    let data = include_str!("./day2/day2p1.txt");
    let lines = data.lines().collect::<Vec<&str>>();
    let split_for_games = lines.iter().map(|x| x.split(":").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    let mut sum = 0;
    for game in split_for_games {
        let gamenumber= game[0].chars().filter(|x| x.is_numeric()).collect::<String>().parse::<i32>().unwrap();
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;
        for round in game[1].split(";").collect::<Vec<&str>>() {
            // println!("{:?}", round);
                let split_balls = round.split(",").collect::<Vec<&str>>();
                for ball in split_balls {
                    // println!("{}", ball);
                    if ball.contains("blue") {
                        let x =  ball.chars().filter(|x| x.is_numeric()).collect::<String>();
                        if x.parse::<i32>().unwrap() > BLUE {
                            blue = x.parse::<i32>().unwrap();
                        }
                    } else if ball.contains("red") {
                       let x =  ball.chars().filter(|x| x.is_numeric()).collect::<String>();
                         if x.parse::<i32>().unwrap() > RED {
                             red = x.parse::<i32>().unwrap();
                         }
                    } else if ball.contains("green") {
                        let x =  ball.chars().filter(|x| x.is_numeric()).collect::<String>();
                        if x.parse::<i32>().unwrap() > GREEN {
                            green = x.parse::<i32>().unwrap();
                        }
                    } else {
                        println!("Error {}" ,gamenumber);
                    }
                }
            
                
            }
        if blue <= BLUE && red <= RED && green <= GREEN {
            sum += gamenumber;
            println!("{} {} {} {}", blue, red, green, gamenumber);
        } else {
            println!("{} {} {} {} not Valid", blue, red, green, gamenumber);
        }
         
    }
    println!("{}", sum);
}
