use randaam::{Randaam, Rarity};

fn main() {
    let randaam = Randaam::build(
        randaam::person(),
        randaam::age(),
        "woont in een boom".to_string(),
        300,
        Rarity::Normal,
        randaam::emoji(),
    );

    println!("{}", randaam.person);
    println!("{}", randaam.age);
    println!("{}", randaam.emoji);
}
