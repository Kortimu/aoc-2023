use std::fs::read_to_string;

pub fn run() -> () {
    let results: Vec<String> = read_to_string("./src/days/1/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut calibration_values: Vec<u32> = Vec::new();

    for line in results {
        let numbers: Vec<String> = line.matches(char::is_numeric)
            .map(String::from)
            .collect();

        // let mut calibration_value: String = String::new;
        let first_number = numbers.first().unwrap();
        let last_number = numbers.last().unwrap();
        
        let calibration_value = format!("{first_number}{last_number}");
        calibration_values.push(calibration_value.parse::<u32>().unwrap());
    }

    let sum: u32 = calibration_values.iter().sum();

    return println!("The sum of all calibration values: {:?}", sum);
}