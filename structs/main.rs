struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    let mut bg = Color {red: 255, green: 70, blue: 15};

    println!("{}, {}, {}", bg.red, bg.green, bg.blue);

    bg.blue = 31;

    println!("{}, {}, {}", bg.red, bg.green, bg.blue);
}