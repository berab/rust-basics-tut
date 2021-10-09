enum Direction{
    Up,
    Down,
    Left,
    Right
}

fn main(){
    let player_direction:Direction = Direction::Down; //

    match player_direction {
        Direction::Up => println!("We are heading up!"),
        Direction::Down => println!("downin!"),
        Direction::Left => println!("lefting!"),
        Direction::Right => println!("right choice!"),
    }


}