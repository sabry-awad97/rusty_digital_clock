# Rusty Digital Clock

This is a simple digital clock written in Rust that displays the current time in a console window using ASCII art. The clock is updated every second and the time is displayed using the 24-hour format.

## Requirements

To run this program, you need to have Rust installed on your machine. You can download and install Rust from the official website: https://www.rust-lang.org/tools/install

## Usage

1. Clone this repository to your local machine:

   ```
   git clone https://github.com/sabry-awad97/rusty_digital_clock.git
   ```

2. Change into the project directory:

   ```
   cd rusty_digital_clock
   ```

3. Compile and run the program:

   ```
   cargo run
   ```

4. The clock will start running in the console window. To stop the program, simply press `Ctrl-C`.

## Customization

If you want to customize the clock display, you can modify the `DIGITS` array in the `main.rs` file. Each element of the array represents a row of ASCII art for a digit. There are 7 rows in total, and each row contains 11 characters.

## License

This program is licensed under the MIT License.
