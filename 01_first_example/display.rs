use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

fn main() {
	println!("The value is {}", Structure(2));
}

