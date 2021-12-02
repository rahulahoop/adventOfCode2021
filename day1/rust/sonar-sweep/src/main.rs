use std::fs;

static FILE_PATH: &str = "../../input.txt";

fn main() {
    count_increasing_steps(); // part 1
    count_increasing_window_steps(); // part 2
}

fn count_increasing_steps() {
    let sonar_values = fs::read_to_string(FILE_PATH)
        .expect("File DNE");

    let mut cnt = 0;
    let mut previous: i32 = 0;

    for line in sonar_values.lines() {
        let sweep = line.parse().unwrap();
        if sweep > previous && previous != 0 {
            cnt = cnt + 1;
        }
        previous = sweep;
    }

    println!("PART 1:  {}", cnt);
}

fn count_increasing_window_steps() {
    let sonar_values = fs::read_to_string(FILE_PATH)
        .expect("File DNE");

    let window_size = 3;
    let lines = sonar_values.lines();
    let v : Vec<i32> = lines.into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let mut pos = 0;
    let mut windowed_grouped_sweeps: Vec<i32> = Vec::new();

    for val  in &v {

        let mut peek_pos = 1;
        let mut window_sweep : i32 = *val;

        while peek_pos < window_size {
            let next_val = v.get(pos+peek_pos);
            match next_val {
                None => {
                    print_count(& windowed_grouped_sweeps);
                    return;
                }
                Some(_) => {
                    let nv = next_val.unwrap();
                    window_sweep = window_sweep + nv;
                }
            }
            peek_pos = peek_pos + 1;
        }


        windowed_grouped_sweeps.push(window_sweep);

        pos = pos + 1;
    }
}

fn print_count(v: &Vec<i32>) {
    let mut cnt = 0;
    let mut prevous_window = 0;

    for val in v {
        if *val > prevous_window && prevous_window != 0 {
            cnt = cnt + 1;
        }
        prevous_window = *val;
    }

    println!("PART 2: {}", cnt)
}
