use std::collections::HashMap;
use std::fs;

static FILE_PATH: &str = "../../input.txt";

fn main() {
    calculate_power_consumption(); // part 1
    calculate_life_suppport_rating();
}

fn calculate_life_suppport_rating() {
    let values : Vec<String> = fs::read_to_string(FILE_PATH)
        .expect("file not found!")
        .lines()
        .map(|x| x.to_string())
        .collect();

    let oxygen_value_bin = calculate_oxygen(values.to_vec());
    let co2_value_bin = calculate_carbon_dioxide(values.to_vec());

    let oxygen_value = isize::from_str_radix(oxygen_value_bin.as_ref(), 2, ).unwrap() as i32;
    let co2_value = isize::from_str_radix(co2_value_bin.as_ref(), 2, ).unwrap() as i32;
    println!("{}, {}", oxygen_value_bin, co2_value_bin);
    println!("PART 2: o2 {} co2 {} v: {}",oxygen_value, co2_value, oxygen_value * co2_value);
}

fn calculate_carbon_dioxide(mut values: Vec<String>) -> String {
    let bit_length = values.get(0).unwrap().len() as i32;

    return find_apex_string(0, bit_length, &mut values, '0');
}

fn calculate_oxygen(mut values: Vec<String>) -> String {

    let bit_length = values.get(0).unwrap().len() as i32;

    return find_apex_string(0, bit_length, &mut values, '1');

}

fn find_apex_string(i: usize, bit_length: i32, apex_strings: &mut Vec<String>, tie_breaker : char) -> String {
    let mut values : Vec<String> = Vec::new();

    let mut z_index_count = 0;
    let mut o_index_count = 0;
    for bin_string in apex_strings.iter() {
        let val_at_index = bin_string.chars().nth(i as usize).unwrap();

        if val_at_index == '1' {
            o_index_count = o_index_count + 1;
        } else {
            z_index_count = z_index_count + 1;
        }
    }

    for bin_string in apex_strings.iter() {
        let val_at_index = bin_string.chars().nth(i as usize).unwrap();

        if tie_breaker == '1' {
            if z_index_count > o_index_count && val_at_index == '0' {
                values.push(bin_string.to_string())
            } else if o_index_count > z_index_count && val_at_index == '1' {
                values.push(bin_string.to_string());
            } else if z_index_count == o_index_count && val_at_index == tie_breaker {
                values.push(bin_string.to_string());
            }
        } else {
            if z_index_count < o_index_count && val_at_index == '0' {
                values.push(bin_string.to_string())
            } else if o_index_count < z_index_count && val_at_index == '1' {
                values.push(bin_string.to_string());
            } else if z_index_count == o_index_count && val_at_index == tie_breaker {
                values.push(bin_string.to_string());
            }
        }

    }

    if values.len() != 1 && i < bit_length as usize {
        find_apex_string(i + 1, bit_length, &mut values, tie_breaker)
    } else {
        return values.get(0).unwrap().to_string();
    }

}


fn calculate_power_consumption() {
    let diagnostics = fs::read_to_string(FILE_PATH).expect("File DNE");

    let mut bit_length: i32 = 0;
    let mut bit_tuple_cnt = HashMap::new();

    let lines = diagnostics.lines();

    for line in lines {
        if bit_length == 0 {
            bit_length = line.len() as i32;
        }

        for (idx, bit) in line.chars().enumerate() {
            match bit_tuple_cnt.get(&idx) {
                None => {
                    let tuple = if bit == '1' { (0, 1) } else { (1, 0) };
                    bit_tuple_cnt.insert(idx, tuple);
                },
                Some(_) => {
                    let (mut z, mut o) = bit_tuple_cnt.get(&idx).unwrap();

                    if bit == '1' {
                        z = z + 1;
                    } else {
                        o = o + 1;
                    }

                    bit_tuple_cnt.insert(idx, (z, o));
                }
            }
        }
    }

    let mut epsilon_string = String::from("");

    for x in 0..bit_length {
        let (z, o) = &bit_tuple_cnt.get(&(x as usize)).unwrap();

        if z > o {
            epsilon_string.push('0');
        } else {
            epsilon_string.push('1');
        }
    }


    let mut gamma_string = str::replace(&epsilon_string, "1", "x");
    gamma_string = str::replace(&gamma_string, "0", "1");
    gamma_string = str::replace(&gamma_string, "x", "0");

    let epsilon_rate = isize::from_str_radix(epsilon_string.as_ref(), 2, ).unwrap() as i32;
    let gamma_rate = isize::from_str_radix(gamma_string.as_ref(), 2, ).unwrap() as i32;


    println!("b {} i {}", epsilon_string, epsilon_rate);
    println!("b {} i {}", gamma_string, gamma_rate);

    println!("PART 1: {}", epsilon_rate * gamma_rate)
}
