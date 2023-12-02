fn main() {

    let data = include_str!("./day2/day2p2.txt");
    let lines = data.lines().collect::<Vec<&str>>();
    let split_for_games = lines.iter().map(|x| x.split(":").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    let mut sum = 0;
    for game in split_for_games {
        let gamenumber= game[0].chars().filter(|x| x.is_numeric()).collect::<String>().parse::<i32>().unwrap();
        let mut blue = 1;
        let mut red = 1;
        let mut green = 1;
        for round in game[1].split(";").collect::<Vec<&str>>() {
            // println!("{:?}", round);
                let split_balls = round.split(",").collect::<Vec<&str>>();
                for ball in split_balls {
                    // println!("{}", ball);
                    if ball.contains("blue") {
                        let x =  ball.chars().filter(|x| x.is_numeric()).collect::<String>();
                        if x.parse::<i32>().unwrap() >  blue {
                            blue = x.parse::<i32>().unwrap();
                        }
                    } else if ball.contains("red") {
                       let x =  ball.chars().filter(|x| x.is_numeric()).collect::<String>();
                         if x.parse::<i32>().unwrap() > red {
                             red = x.parse::<i32>().unwrap();
                         }
                    } else if ball.contains("green") {
                        let x =  ball.chars().filter(|x| x.is_numeric()).collect::<String>();
                        if x.parse::<i32>().unwrap() > green {
                            green = x.parse::<i32>().unwrap();
                        }
                    } else {
                        println!("Error {}" ,gamenumber);
                    }
                }
            
                
            }
       sum += red * green * blue;
         
    }
    println!("{}", sum);
}
