struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    let blue = Color {red: 0, green: 0, blue: 255};

    // print_color(blue);
    // print_color(blue); doesnt work the second the. error: use of moved value: blue so pass by ref.
    print_color(&blue);
    print_color(&blue); // no error
}

fn print_color(c: &Color) {
    println!("Color -R:{}, G:{}, B:{}", c.red, c.green, c.blue); 

}