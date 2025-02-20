use rand::{
    distr::{Distribution, Uniform},
    Rng,
};

fn main() {
    let mut rng = rand::rng();
    println!("Integer: {}", rng.random_range(0..10));
    println!("Float: {}", rng.random_range(0.0..10.0));

    let die = Uniform::try_from(1..7).unwrap();
    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the Die: {}", throw);
        if throw == 6 {
            println!("You won!");
            break;
        }
    }
}
