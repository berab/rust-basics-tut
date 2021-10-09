fn main() {
    let mut x = 3;
    let xr = &mut x;

    println!("{}", x);
    
    *xr += 1; // shadowing was neede b4. no more.

    println!("changed xd {}", x)
    
}