use std::collections::HashMap;

fn main() {
    ////////////////////////////////////////////////////////////////////
    // Vectors: Vec<T>
    ////////////////////////////////////////////////////////////////////
    println!("VECTORS:\n");
    
    let mut v: Vec<i32> = vec![1, 2, 3];
    // use mutable vec and push elements to double the sequence
    let len = v.len();
    for i in 0..len {
        v.push(v[i]);
    }
    println!("{} {} {}", v[3], v[4], v[5]);
    
    
    // 1. push - add element to the end, in place
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4);
    println!("After push: {:?}", v); // [1, 2, 3, 4]

    // 2. pop - remove last element, returns an Option
    let last: Option<i32> = v.pop();
    println!("After pop: {:?}, removed: {:?}", v, last); // [1, 2, 3], Some(4)

    // 3. len - get the length, returns a usize
    println!("Length: {}", v.len()); // 3

    // 4. is_empty - check if empty, returns bool
    println!("Is empty? {}", v.is_empty()); // false

    // 5. get - access element safely, returns an Option of a reference
    let third: &i32 = &v[2];
    println!("The third value in the vector is {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("We now used get to find the third value {third}"),
        None => println!("No third value was found!!"),
    }
    println!("Element at index 1: {:?}", v.get(1)); // Some(2)
    println!("Out of bounds: {:?}", v.get(5)); // None

    // 6. remove - remove element at index, returns that element
    let removed: i32 = v.remove(1);
    println!("After remove: {:?}, removed: {}", v, removed); // [1, 3], 2

    // 7. insert - insert element at index, with elements shifted right
    v.insert(1, 2);
    println!("After insert: {:?}", v); // [1, 2, 3]

    // 8. iter - iterate over elements
    print!("Iterating: ");
    for x in v.iter() {
        print!("{} ", x);
    }
    println!();

    // 9. drain - remove elements in range, return iter of removed
    let mut v2: Vec<i32> = vec![1, 2, 3, 4, 5];
    let drained: Vec<_> = v2.drain(1..4).collect();
    println!("After drain: {:?}, drained: {:?}", v2, drained); // [1, 5], [2, 3, 4]

    // 10. retain - keep only elements that satisfy predicate, in place
    let mut v3: Vec<i32> = vec![1, 2, 3, 4, 5];
    v3.retain(|&x| x % 2 == 0);
    println!("After retain: {:?}", v3); // [2, 4]
    println!();
    
    ////////////////////////////////////////////////////////////////////
    // Strings: String
    ////////////////////////////////////////////////////////////////////
    println!("\nSTRINGS:\n");
    
    let mut s1: String = String::from("foo");
    let s2: &str = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    
    let s1: String = String::from("tic");
    let s2: String = String::from("tac");
    let s3: String = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);
    
    // 1. push_str - append a string slice, in place
    let mut s: String = String::from("Hello");
    s.push_str(", world!");
    println!("After push_str: {}", s); // "Hello, world!"

    // 2. push - single character append, in place
    s.push('!');
    println!("After push: {}", s); // "Hello, world!!"

    // 3. len - get the length in bytes, returns usize
    println!("Length: {}", s.len()); // 14

    // 4. is_empty - check if the string is empty, returns bool
    let empty: String = String::new();
    println!("Is empty? {}", empty.is_empty()); // true

    // 5. replace - replace occurrences of a substring, returns new String
    let replaced: String = s.replace("world", "Rust");
    println!("After replace: {}", replaced); // "Hello, Rust!!"

    // 6. contains - heck if string contains a substring, returns bool
    println!("Contains 'Rust'? {}", replaced.contains("Rust")); // true

    // 7. split_whitespace - split string by whitespace
    let words: Vec<&str> = replaced.split_whitespace().collect();
    println!("Words: {:?}", words); // ["Hello,", "Rust!!"]

    // 8. trim - strip outer whitespace, returns &str 
    let spacey: &str = "  Hello Rust  ";
    println!("Trimmed: '{}'", spacey.trim()); // "Hello Rust"

    // 9. chars - make iterator over characters of &str
    print!("Characters: ");
    for c in "Rust".chars() {
        print!("\n{}", c);
    }
    println!();

    // 10. to_uppercase - convert to uppercase, returns String
    let upper: String = "rust".to_uppercase();
    println!("Uppercase: {}", upper); // "RUST"
    
    ////////////////////////////////////////////////////////////////////
    // Hash Maps: HashMap<K, V>
    ////////////////////////////////////////////////////////////////////
    println!("\nHASHMAPS:\n");
    
    let mut map: HashMap<&str, i32> = HashMap::from([("red", 3), ("blue", 5)]);

    // 1. insert - Add or update a key-value pair
    map.insert("gold", 7);
    println!("After insert: {:?}", map); // {"red": 3, "blue": 5, "gold": 7}

    // 2. get - Retrieve a value by key
    if let Some(&count): Option<&i32> = map.get("red") {
        println!("Red count: {}", count); // 3
    }

    // 3. contains_key - Check if a key exists
    println!("Contains 'blue'? {}", map.contains_key("blue")); // true

    // 4. remove - Remove a key-value pair
    map.remove("blue");
    println!("After remove: {:?}", map); // {"red": 3, "gold": 7}

    // 5. entry - Insert only if key is missing
    map.entry("green").or_insert(10);
    println!("After entry: {:?}", map); // {"red": 3, "gold": 7, "green": 10}

    // 6. iter - Iterate over key-value pairs
    print!("Iterating: ");
    for (key, value) in &map {
        print!("{}:{} ", key, value);
    }
    println!(); // Newline

    // 7. keys - Iterate over keys
    print!("Keys: ");
    for key in map.keys() {
        print!("{} ", key);
    }
    println!();

    // 8. values - Iterate over values
    print!("Values: ");
    for value in map.values() {
        print!("{} ", value);
    }
    println!();

    // 9. retain - Remove elements based on condition
    map.retain(|_key, &mut value| value > 5);
    println!("After retain: {:?}", map); // Keeps values > 5

    // 10. len - Get the number of key-value pairs
    println!("Length: {}", map.len());
    
}
