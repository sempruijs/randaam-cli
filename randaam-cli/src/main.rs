use randaam::{Randaam, Rarity};

fn main() {
    let randaam = Randaam::build(
        randaam::person(),
        17,
        "woont in een boom".to_string(),
        300,
        Rarity::Normal,
    );

    println!("{}", randaam.person);
}
