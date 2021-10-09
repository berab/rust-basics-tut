fn main(){
    let mut my_vector = vec![1, 2, 3, 4];

    my_vector.push(59); // [1, 2, 3, 4, 59]
    my_vector.remove(2); // [1, 2, 4, 59] (removes item at index 2)

    for number in my_vector.iter(){
        println!("{}", number);
    }
}