mod hey;

fn main() {
	//let x: u8;
	let x = 50;
	//println!("The value of x is: {}", x);
		print_number(x, "x");

	if x == 5 {
		println!("x is five!");
	} else {
		println!("x is not five :(");
	}

	let y = if x == 5 {
		5
	} else {
		10
	};
	// println!("So the value of y is: {}", y);
	print_number(y, "y");

	hey::run();
}

fn print_number(x: u8, value: &str) {
	println!("The value of {} is: {}", value, x);
}

// cargo run
