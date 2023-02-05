use randaam::Randaam;

fn main() {
    let randaam = Randaam::build(
        randaam::person(),
        randaam::age(),
        randaam::location(),
        randaam::salary(),
        randaam::rarity(),
        randaam::emoji(),
    );

    println!("{}", randaam.person);
    println!("{}", randaam.age);
    println!("{}", randaam.emoji);
    println!("{:?}", randaam.rarity);
    println!("{}", randaam.salary);
    println!("woont in een {}", randaam.location);
}
