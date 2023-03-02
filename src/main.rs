use chrono::Local;

const DIGITS : [[&str; 11]; 7] = [
    ["┏━┓ ","  ╻  "," ┏━┓ ", " ┏━┓ "," ╻ ╻ "," ┏━┓ "," ┏   "," ┏━┓ "," ┏━┓ "," ┏━┓ ","   "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ "," ╻ "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┏━┛ ", " ┣━┫ "," ┗━┫ "," ┗━┓ "," ┣━┓ ","   ┃ "," ┣━┫ "," ┗━┫ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ╹ "],
    ["┗━┛ ","  ╹  "," ┗━━ ", " ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   "],
];

fn main() {
    let mut clock = DigitalClock::new();
    loop {
        clock.update();
    }
}

struct DigitalClock {
    digits: [[&'static str; 11]; 7],
}

impl DigitalClock {
    fn new() -> DigitalClock {
        DigitalClock { digits: DIGITS }
    }

    fn print_time(&mut self) {
        let time = Local::now().time();
        let time_str = time.format("%H:%M:%S").to_string();
        let rows = self.digits.len();
        let cols = time_str.len();
        for row in 0..rows {
            for col in 0..cols {
                let c = time_str.chars().nth(col).unwrap();
                let digit_col = match c {
                    '0'..='9' => c as usize - '0' as usize,
                    _ => 10,
                };
                print!("{} ", self.digits[row][digit_col]);
            }
            println!();
        }
    }

    fn update(&mut self) {
        print!("\x1B[2J");
        print!("\x1B[?25l");
        self.print_time();
        std::thread::sleep(std::time::Duration::from_millis(999));
        print!("\x1b[7A");
    }
}
