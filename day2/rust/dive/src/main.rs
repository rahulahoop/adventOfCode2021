use std::fs;
use std::str::FromStr;

static FILE_PATH: &str = "../../input.txt";

fn main() {
    find_horizontal_position(); // part 1
    find_horizontal_position_using_aim();
}
fn find_horizontal_position() {
    let mut hz_pos = 0;
    let mut dp_pos = 0;

    let directions = fs::read_to_string(FILE_PATH)
        .expect("File DNE");

    for line in directions.lines() {
        let mut whitespace = line.split_whitespace();
        let dir = whitespace.next().unwrap();
        let amt = i32::from_str(whitespace.next().unwrap()).unwrap();

        match dir.to_lowercase().as_ref() {
            "up" => dp_pos = dp_pos - amt,
            "down" => dp_pos = dp_pos + amt,
            "forward" => hz_pos = hz_pos + amt,
            _ => {}
        }
    }

    println!("PART 1: hz: {}, dp: {}, final {}", hz_pos, dp_pos, hz_pos * dp_pos);

}

fn find_horizontal_position_using_aim() {
    let mut hz_pos = 0;
    let mut dp_pos = 0;
    let mut aim = 0;

    let directions = fs::read_to_string(FILE_PATH)
        .expect("File DNE");

    for line in directions.lines() {
        let mut whitespace = line.split_whitespace();
        let dir = whitespace.next().unwrap();
        let amt = i32::from_str(whitespace.next().unwrap()).unwrap();

        match dir.to_lowercase().as_ref() {
            "up" => aim = aim - amt
            ,
            "down" => aim = aim + amt
            ,
            "forward" => {
                hz_pos = hz_pos + amt;
                dp_pos = dp_pos + (aim * amt);
            },
            _ => {}
        }
    }

    println!("PART 2: hz: {}, dp: {}, aim {}, final {}", hz_pos, dp_pos, aim, hz_pos * dp_pos);

}


