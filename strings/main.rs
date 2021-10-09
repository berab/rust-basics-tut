fn main() {
    let mut my_str = String::from("Mrb ben beran xd");

    //length
    println!("Length: {}", my_str.len());

    //isempty
    println!("Is the string empty?: {}", my_str.is_empty());

    for token in my_str.split_whitespace(){
        println!("{}", token);

    }

    println!("Does the string contain 'Mrb'?: {}", my_str.contains("Mrb"));

    my_str.push_str(" Ama canum cok sukuluyoo yua.");

    println!("{}", my_str);
}