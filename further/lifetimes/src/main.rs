const SOME_CONST_A: &str = "I'm a constant!";
const SOME_CONST_B: &str = "I'm a constant too!";

fn main() {
    //let a:Vec<i32> = vec![1,2,3,4,5];
    //let b:Vec<i32> = vec![2,3,4,5];
    //let result = get_vec_slice(&a, &b);
    //println!("{:?}", result);
    let a = String::from("a");
    let greater = some_func(&SOME_CONST_A, &SOME_CONST_B);
    println!("{}", greater);
}

fn get_vec_slice<'a>(param_1: &'a [i32], param_2: &'a [i32]) -> &'a [i32] {
    if param_1.len() > param_2.len() {
        &param_1[0..2]
    } else {
        &param_2[0..2]
    }
} // strings are just vectors with u8 so the same concept applies

// static: lifetime that lasts the entire program
// constants are static by nature
// can also have static variables

fn some_func(param_1: &'static str, param_2: &'static str) -> &'static str {
    if param_1.len() > param_2.len() {
        param_1
    } else {
        param_2
    }
}