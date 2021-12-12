use crate::file_handler;

/*
    count, how many items n > n-1
*/
pub fn execute_sonar_sweep_task() {
    println!("Start Sonar Sweep Task.");

    let sonar_sweep_data = file_handler::read_all::<i32>("./src/sonar_sweep/data.txt");

    //let result = sonar_sweeper_data
    //    .iter()
    //    .filter(|&x| 
    //        x != &sonar_sweeper_data[0] 
    //        && x.as_ref().unwrap() > &sonar_sweeper_data[sonar_sweeper_data.iter().position(|y| y.as_ref().unwrap() == x.as_ref().unwrap()).unwrap() - 1].as_ref().unwrap())
    //    .map(|x| println!("Der Wert {0:?} ist groeser als der vorherige Wert {1}", x, &sonar_sweeper_data[sonar_sweeper_data.iter().position(|y| y.as_ref().unwrap() == x.as_ref().unwrap()).unwrap() - 1].as_ref().unwrap()))
    //    .count();

    println!("Result of Part 1 of Sonar Sweep Task: {0}", count_increased_numbers(sonar_sweep_data.iter().map(|x| *x.as_ref().unwrap()).collect()));

    let mut sum_collection: Vec<i32> = Vec::new();
    
    for (position, item) in sonar_sweep_data.iter().enumerate() {
        if sonar_sweep_data.iter().count() > position + 2 {
            let sum = item.as_ref().unwrap() + sonar_sweep_data[position + 1].as_ref().unwrap() + sonar_sweep_data[position + 2].as_ref().expect("Out of index");
            sum_collection.push(sum);
        }
    }

    println!("Result of Part 2 of Sonar Sweep Task: {0}", count_increased_numbers(sum_collection));
    println!("Stop Sonar Sweep Task.");
}

fn count_increased_numbers(collection:Vec<i32>) -> i32{
    let mut counter = 0;

    for (position, item) in collection.iter().enumerate() {
        if position != 0 {
            if item > &collection[position - 1] {
                counter = counter + 1;
            }
        }
    }

    counter
}