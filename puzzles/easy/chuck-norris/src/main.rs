use std::io;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let message = input_line.trim_end().to_string();

    let binary_string = to_bytes(&message);
    let mut chuck_norris_string = String::new();

    let mut current_value = binary_string.chars().next().unwrap();
    let mut count = 0;
    let mut it = binary_string.chars().peekable();
    while let Some(c) = it.next() {
        if c == current_value {
            count += 1;
        }
        if c != current_value {
            chuck_norris_string.push_str(match current_value {
                '1' => " 0 ",
                _ => " 00 ",
            });
            for _n in 0..count {
                chuck_norris_string.push_str("0");
            }
            current_value = c;
            count = 1;
        }
        if it.peek().is_none() {
            chuck_norris_string.push_str(match current_value {
                '1' => " 0 ",
                _ => " 00 ",
            });
            for _n in 0..count {
                chuck_norris_string.push_str("0");
            }
            current_value = c;
            count = 1;
        }
    }
    let chuck_norris_string = chuck_norris_string.trim();
    // println!("{}", binary_string);
    println!("{}", chuck_norris_string);
}

fn to_bytes(input_string: &str) -> String {
    let mut binary_string = String::new();
    for b in input_string.as_bytes() {
        eprintln!("{:07b}", b);
        let b = format!("{:07b}", b);
        binary_string.push_str(&b)
    }
    binary_string
}
