use tests::subtract;

// Note: does not need `#[cfg(test)]` to be included

/*
 * If our project is a binary crate that only contains a src/main.rs 
 * file and doesn’t have a src/lib.rs file, we can’t create integration 
 * tests in the tests directory and bring functions defined in the 
 * src/main.rs file into scope with a use statement. Only library 
 * crates expose functions that other crates can use; binary crates 
 * are meant to be run on their own.
 */

#[test]
fn test_subtract() {
	let result = subtract(10, 10);
	assert!(result == 0);
}
