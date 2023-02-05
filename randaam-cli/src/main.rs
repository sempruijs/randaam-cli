use randaam::Randaam;

fn main() {
    let randaam = Randaam::build(
        randaam::person(),
        randaam::age(),
        "woont in een boom".to_string(),
        randaam::salary(),
        randaam::rarity(),
        randaam::emoji(),
    );

    println!("{}", randaam.person);
    println!("{}", randaam.age);
    println!("{}", randaam.emoji);
    println!("{:?}", randaam.rarity);
    println!("{}", randaam.salary);
}
