use randaam::Randaam;

fn main() {
    let r = randaam::gen_randaam();

    println!("{}", r.person);
    println!("{}", r.age);
    println!("{}", r.emoji);
    println!("{:?}", r.rarity);
    println!("{}", r.salary);
    println!("woont in een {}", r.location);
}
