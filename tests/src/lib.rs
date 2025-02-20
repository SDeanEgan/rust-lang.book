#[derive(PartialEq, Debug)]
struct Rectangle{
	width: u32,
	height: u32,
}

impl Rectangle {
	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}

pub struct Guess {
	val: i32,
}

impl Guess {
	pub fn new(arg: i32) -> Guess {
		if arg < 1 || arg > 100 {
			panic!("Argument for Guess must be between 1 and 100!");
		}
		Guess { val: arg }
	}
}

pub fn subtract(l: usize, r: usize) -> usize {
	l - r
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_add() {
		assert_eq!(subtract(5, 4), 1);
	}
	
	#[test]
	fn failure() {
		panic!("You just can't win.");
	}
	
	#[test]
	fn test_can_hold() {
		let r1 = Rectangle {
			width: 10,
			height: 10,
		};
		let r2 = Rectangle {
			width: 1,
			height: 2,
		};
		
		assert!(r1.can_hold(&r2));
		assert!(!r2.can_hold(&r1));
	}
	
	#[test]
	fn test_eq() {
		let left = Rectangle {
			width: 1,
			height: 1,
		};
		let right = Rectangle {
			width: 1,
			height: 1,
		};
		assert_eq!(left, right);
	}
	
	#[test]
	fn test_msg() {
		let result = subtract(2, 1);
		assert_ne!(
			result,
			1,
			"This failed because `{result}` was equal to 1."
			);
		}
		
	#[test]
	#[should_panic]
	fn test_guess_panic() {
		Guess::new(123456789);
	}
	
	#[test]
	fn it_works() -> Result<(), String> {
        let result = subtract(2, 2);

        if result == 0 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
	
