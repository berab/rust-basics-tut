#[allow(unused_variables)]
fn main() {
    let example_str: &str = "Howdy"; // immutable, a heap ref.
    let example_string: String = String::from("Partner"); // -> mutable and heap(memory storage place)
    
    //translating btw. two
    let string_from_str: String = example_str.to_string();
    let string_from_str2: String = "Some hardcoded string".to_string();

    let string_from_hardcoded = String::from("Some hardcoded");
    let string_from_str_var = String::from("example_str");

    let str_from_string: &str = &example_string; //just a pointer to the string tho.

    // adding two strings
    let combine_string_literals = ["first", "second"].concat();
    let combine_with_format_macro = format!("{} {}", "first", "second");

    let string_plus_str = example_string + example_str; // doesnt work if string is NOT first. get an error

    let mut mut_string = String::new();
    mut_string.push_str(example_str);
    mut_string.push_str("Smt to push");
    mut_string.push('m'); // pushing chars. doesnt work for strings

    let a = String::from("a");
    let b = String::from("b");
    let s = String::new();

    println!("{}", &&a);

    let number_list = vec![34, 50, 25, 100, 65];

    let combined = a + &b + &mut_string;

    let str_from_substring: &str = &example_str[0..2];
    let char_by_index = &example_str.chars().nth(1);

    match char_by_index {
        Some(c) => println!("Found a char {}", c),
        None => {}
    }

    if let Some(c) = example_str.chars().nth(2) {
        println!("Found a char {}", c);
    }
}