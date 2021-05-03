use std::io;
use std::cmp::Ordering;
use rand::Rng;

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

	println!("Guess the number!\n(To quit, enter a non-number into the guess and the program will crash)");
	let secret_number = rand::thread_rng().gen_range(1, 101);
	let mut attempts = 1;
	//println!("The secret number is {}", secret_number);
	println!("Please input your guess.");

	loop {
		let mut guess = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");
		let guess: u32 = guess.trim().parse().expect("Program crash, please input a number");
		//println!("You guessed: {}", guess);

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
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
