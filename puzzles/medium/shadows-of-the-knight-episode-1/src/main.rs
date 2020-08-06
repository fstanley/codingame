use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

struct Point {
    x: i32,
    y: i32,
}

struct Target_Zone {
    top_left: Point,
    bottom_right: Point,
}

impl Target_Zone {
    fn center(&self) -> Point {
        let x = (self.top_left.x + self.bottom_right.x) / 2;
        let y = (self.top_left.y + self.bottom_right.y) / 2;
        Point { x, y }
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32); // width of the building.
    let h = parse_input!(inputs[1], i32); // height of the building.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // maximum number of turns before game over.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let x0 = parse_input!(inputs[0], i32);
    let y0 = parse_input!(inputs[1], i32);

    let mut target = Target_Zone {
        top_left: Point { x: 0, y: 0 },
        bottom_right: Point { x: w - 1, y: h - 1 },
    };

    let mut current_position = Point { x: x0, y: y0 };
    // game loop

    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let bomb_dir = input_line.trim().to_string(); // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)
        match bomb_dir.as_str() {
            "U" => {
                target.top_left = Point {
                    x: current_position.x,
                    y: target.top_left.y,
                };
                target.bottom_right = Point {
                    x: current_position.x,
                    y: current_position.y - 1,
                };
            }
            "UR" => {
                target.top_left = Point {
                    x: current_position.x + 1,
                    y: target.top_left.y,
                };
                target.bottom_right = Point {
                    x: target.bottom_right.x,
                    y: current_position.y - 1,
                };
            }
            "R" => {
                target.top_left = Point {
                    x: current_position.x + 1,
                    y: current_position.y,
                };
                target.bottom_right = Point {
                    x: target.bottom_right.x,
                    y: current_position.y,
                };
            }
            "DR" => {
                target.top_left = Point {
                    x: current_position.x + 1,
                    y: current_position.y + 1,
                };
                target.bottom_right = Point {
                    x: target.bottom_right.x,
                    y: target.bottom_right.y,
                };
            }
            "D" => {
                target.top_left = Point {
                    x: current_position.x,
                    y: current_position.y + 1,
                };
                target.bottom_right = Point {
                    x: current_position.x,
                    y: target.bottom_right.y,
                };
            }
            "DL" => {
                target.top_left = Point {
                    x: target.top_left.x,
                    y: current_position.y + 1,
                };
                target.bottom_right = Point {
                    x: current_position.x - 1,
                    y: target.bottom_right.y,
                };
            }
            "L" => {
                target.top_left = Point {
                    x: target.top_left.x,
                    y: current_position.y,
                };
                target.bottom_right = Point {
                    x: current_position.x - 1,
                    y: current_position.y,
                };
            }
            "UL" => {
                target.top_left = Point {
                    x: target.top_left.x,
                    y: target.top_left.y,
                };
                target.bottom_right = Point {
                    x: current_position.x - 1,
                    y: current_position.y - 1,
                };
            }
            _ => print!(""),
        }

        println!("{} {}", target.center().x, target.center().y);
        current_position.x = target.center().x;
        current_position.y = target.center().y;
    }
}
