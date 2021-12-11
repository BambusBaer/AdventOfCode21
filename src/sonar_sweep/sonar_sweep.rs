use crate::file_handler;

/*
    Increment counter, when n > n-1
*/
pub fn execute_sonar_sweep_task() {
    println!("Start Sonar Sweep Task.");

    let sonar_sweeper_data = file_handler::read_all::<i32>("./src/01/data.txt");

    let result = sonar_sweeper_data
        .iter()
        .filter(|&x| x.as_ref().unwrap() > &sonar_sweeper_data[sonar_sweeper_data.iter().position(|y| y.as_ref().unwrap() == x.as_ref().unwrap()).unwrap() - 1].as_ref().unwrap())
        .count();

    println!("Result of [01] Sonar Sweep Task: {0}", result);
    println!("Stop Sonar Sweep Task.");
}