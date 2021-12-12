use crate::file_handler;

pub fn execute_dive_task(){
    println!("Start Dive Task.");
    let file_content = file_handler::read_all::<String>("./src/dive/data.txt");
    let horizontal_movements: Vec<i32> = file_content
                                            .iter()
                                            .filter(|x| x.as_ref().unwrap().split(' ').collect::<Vec<&str>>().first() == Some(&"forward"))
                                            .map(|x| x.as_ref().unwrap().split(' ').collect::<Vec<&str>>().last().unwrap().parse().unwrap())
                                            .collect();

    let depth_movements: Vec<(&str, i32)> = file_content
                                                .iter()
                                                .filter(|x| x.as_ref().unwrap().split(' ').collect::<Vec<&str>>().first() != Some(&"forward"))
                                                .map(|x| (*x.as_ref().unwrap().split(' ').collect::<Vec<&str>>().first().unwrap(), x.as_ref().unwrap().split(' ').collect::<Vec<&str>>().last().unwrap().parse().unwrap()))
                                                .collect();

    let aim_movements: Vec<(&str, i32)> = file_content
                                            .iter()
                                            .map(|x| (*x.as_ref().unwrap().split(' ').collect::<Vec<&str>>().first().unwrap(), x.as_ref().unwrap().split(' ').collect::<Vec<&str>>().last().unwrap().parse().unwrap()))
                                            .collect();

    let horizontal_result = handle_horizontal_position(horizontal_movements);
    let depth_result = handle_depth(depth_movements);
    let aim_result = handle_depth(aim_movements);

    println!("Depth: {0}\nHorizontal: {1}", depth_result, horizontal_result);
    println!("Result Part 01 of the dive Task: {0}", horizontal_result * depth_result);
    
    println!("Aim: {0}\nHorizontal: {1}", aim_result, horizontal_result);
    println!("Result Part 02 of the dive Task: {0}", horizontal_result * aim_result);
    println!("Stop Dive Task.");
}

fn handle_horizontal_position(movements: Vec<i32>) -> i32{
    movements.iter().sum()
}

fn handle_depth(movements: Vec<(&str, i32)>) -> i32{
    let mut aim = 0;
    let mut depth = 0;

    for (key, value) in movements.into_iter(){
        match key {
            "down" => aim = aim + value,
            "up" => aim = aim - value,
            "forward" => depth = (aim * value) + depth,
            _ => print!("Key not found")
        }
    }

    if depth == 0 {
        aim
    }else{
        depth
    }
}