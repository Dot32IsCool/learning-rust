use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

//mod hey;

fn main() {
	//let x: u8;
	// let x = 50;
	// //println!("The value of x is: {}", x);
	// 	print_number(x, "x");

	// if x == 5 {
	// 	println!("x is five!");
	// } else {
	// 	println!("x is not five :(");
	// }

	// let y = if x == 5 {
	// 	5
	// } else {
	// 	10
	// };
	// println!("So the value of y is: {}", y);
	//print_number(y, "y");

	//hey::run();

	println!("\n\n\n{}{}", "Guess the number!".bright_yellow().bold(), "\n(To quit, press ctrl+c)".bright_red());
	let secret_number = rand::thread_rng().gen_range(1, 101);
	let mut attempts = 1;
	//println!("The secret number is {}", secret_number);
	println!("Please input your guess:");

	loop {
		let mut guess = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Please input a valid number!");
				continue;
			}
		}; //.expect("Program crash, please input a number");
		//println!("You guessed: {}", guess);


		match guess.cmp(&secret_number) {
			Ordering::Less => println!("{}", "Too small!".bright_cyan()),
			Ordering::Greater => println!("{}", "Too big!".bright_cyan()),
			Ordering::Equal => {
				println!("You got it! It took you {} attempts.", attempts);
				break;
			}
		}

		attempts += 1;
	}
}

// fn print_number(x: u8, value: &str) {
// 	println!("The value of {} is: {}", value, x);
// }

// cargo run
