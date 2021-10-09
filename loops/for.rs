fn main(){
    let animals = vec!["bunny", "pisi bunny", "pattes yaso"];

    // for a in animals.iter() { 
    //     println!("{}",a);
    // }

    for (i, a) in animals.iter().enumerate(){
        println!("{}. animal is {}", i+1, a);
    } 

    println!("would not get {} without .iter again xd", animals[0]); // without .iter() , cannot access this animals vector anymore. would get an error!!
}