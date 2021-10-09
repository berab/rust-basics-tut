fn main() {
    let numbers: [i32; 4] = [1, 2, 3, 4]; // using : [i32; 4] i32 type and length of 4 but NOT A MUST!
    // let numbers = [2; 400]; 400 times 2 in an array.
    for n in numbers.iter(){
        println!("{}", n);
    }

    for i in 0..numbers.len() {
        println!("{}", numbers[i]);
    }
}