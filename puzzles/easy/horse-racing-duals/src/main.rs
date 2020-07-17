use std::io;


macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut horses = Vec::new();
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let pi = parse_input!(input_line, i32);
        horses.push(pi);
    }

    horses.sort();
    let mut min_distance = std::i32::MAX;
    for n in 0..horses.len()-1 {
        if (horses[n+1] - horses[n]) < min_distance {
            min_distance = horses[n+1] - horses[n]
        }
    }

    println!("{}",min_distance);
}
