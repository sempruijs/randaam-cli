//! # randaam
//!
//! `randaam` generates random imaginary people.
//!
//! Randaam generates random imaginary people.
//! All the content of randaam is currently only availble in Dutch.
//!
//! Randaam can generate the following properties:
//! - names
//! - actions
//! - jobs
//! - ages
//! - emoji
//! - salaries
//! - rarities
//!
//! You can combine all of these properties into a single `randaam`.
//! A "`randaam` is an imaginary person.
//!

use std::fmt::Display;

use rand::seq::SliceRandom; // 0.7.2
use rand::Rng;

/// a randaam is an imaginary person.
pub struct Randaam {
    pub person: String,
    pub age: u8,
    pub location: String,
    pub salary: u32,
    pub rarity: Rarity,
    pub emoji: char,
}

// add rarity in dislpay
impl Display for Randaam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n\n {}\n {}\n {} jaar oud \n €{} per week \n woont in een {}",
            self.rarity, self.emoji, self.person, self.age, self.salary, self.location
        )
    }
}

impl Randaam {
    /// build your own randaam by hand
    pub fn build(
        person: String,
        age: u8,
        location: String,
        salary: u32,
        rarity: Rarity,
        emoji: char,
    ) -> Randaam {
        Randaam {
            person,
            age,
            location,
            salary,
            rarity,
            emoji,
        }
    }
}

/// tells how rare a randaam is.
#[derive(PartialEq, Debug)]
pub enum Rarity {
    Normal,
    Rare,
    Epic,
    Legendary,
    SuperLegendary,
}

impl Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = match self {
            Rarity::SuperLegendary => String::from("SUPER LEGENDARISCH!!!"),
            Rarity::Legendary => String::from("Legendarisch!!"),
            Rarity::Epic => String::from("Episch!"),
            Rarity::Rare => String::from("Zeldzaam"),
            Rarity::Normal => String::from("normaal"),
        };
        write!(f, "{}", r)
    }
}

/// get a random name
///
/// # example
/// ```
/// let random_name = randaam::name();
/// println!("{}", random_name);
/// ```
/// the output could be something like:"Sebartiaan", "Amgla", "Steven", "Clarla"
pub fn name() -> String {
    let names = include_str!("./content/names.txt")
        .lines()
        .collect::<Vec<_>>();
    names.choose(&mut rand::thread_rng()).unwrap().to_string()
}

/// get a random object
///
/// # example
/// ```
/// let random_object = randaam::object();
/// println!("{}", random_object );
/// ```
/// the output could be something like:"goudvis", "pizza", "bomen"
pub fn object() -> String {
    let objects = include_str!("./content/objects.txt")
        .lines()
        .collect::<Vec<_>>();
    objects.choose(&mut rand::thread_rng()).unwrap().to_string()
}

/// get a random action
///
/// # example
/// ```
/// let random_action = randaam::action();
/// println!("{}", random_action);
/// ```
/// the output could be something like:"zitter", "liefhebber", "visser
pub fn action() -> String {
    let actions = include_str!("./content/actions.txt")
        .lines()
        .collect::<Vec<_>>();
    actions.choose(&mut rand::thread_rng()).unwrap().to_string()
}

/// get a random person
///
/// # example
/// ```
/// let random_person = randaam::person();
/// println!("{}", random_person);
/// ```
/// the output could be something like:"Emmanuel de varken wasser", "Sem de brulaap verkoper"
pub fn person() -> String {
    format!("{} de {} {}", name(), object(), action())
}

/// get an random emoji
///
/// # example
/// ```
/// let random_emoji = randaam::emoji();
/// println!("{}", random_emoji);
/// ```
/// the output could be something like:"😀", "😃", "😁"
pub fn emoji() -> char {
    let emojis = include_str!("./content/emojis.txt")
        .chars()
        .filter(|c| c != &'\n')
        .collect::<Vec<_>>();
    //TODO: add more emojis
    emojis.choose(&mut rand::thread_rng()).unwrap().clone()
}

/// get a random location
/// not geographical, just a place where someone could be.
///
/// # example
/// ```
/// let random_location = randaam::location();
/// println!("{}", random_location);
/// ```
/// the output could be something like:"boom", "huis", "snackbar"
pub fn location() -> String {
    let locations = include_str!("./content/locations.txt")
        .lines()
        .collect::<Vec<_>>();

    locations
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
}

/// get a random age between 1 and 100 years
/// the age is in years
///
/// # example
/// ```
/// let random_age = randaam::age();
/// println!("{}", random_age);
/// ```
/// the output could be something like:"42", "2", "89"
pub fn age() -> u8 {
    rand::thread_rng().gen_range(1..101)
}

/// get a rarity
///
/// # Example
/// ```
/// use randaam::Rarity;
///
/// let rarity = randaam::rarity();
/// println!("{}", rarity);
/// ````
/// could be something like: "normaal", "Episch!!"
/// to view the chance list for each rarity, look at the documentation for decide_rarity()
pub fn rarity() -> Rarity {
    let rarity_num = rand::thread_rng().gen_range(1..10001);
    decide_rarity(rarity_num)
}

/// decides how rare a rarity is.
///
/// the input should be a number between 1 and 10000, else if will panic.
///
/// rarity         | chance
/// -------------- | --------------
/// Normal         | 9:10
/// Rare           | 1:10
/// Epic           | 1:100
/// Legendary      |1:1000
/// SuperLegendary | 1:10000
pub fn decide_rarity(x: u32) -> Rarity {
    if x > 10000 {
        panic!("x should be lower or equal to 10000");
    } else if x == 10000 {
        Rarity::SuperLegendary
    } else if x > 9990 {
        Rarity::Legendary
    } else if x > 9900 {
        Rarity::Epic
    } else if x > 9000 {
        Rarity::Rare
    } else if x > 0 {
        Rarity::Normal
    } else {
        panic!("x should be higher or equal to 1");
    }
}

/// get random salary
///
/// salary is in euro's per week
///
/// # Example
/// ```
/// let random_salary = randaam::salary();
/// println!("{}", random_salary);
/// ```
/// Output could be a range between 1 and 2000
pub fn salary() -> u32 {
    rand::thread_rng().gen_range(1..2001)
}

/// generate a random randaam
///
/// # Example
/// ```
/// use randaam::Randaam;
///
/// let r = Randaam::gen();
/// println!("{}", r);
/// ```
/// the output could be something like this:
///
///  normaal
///
///
/// 😆
/// Dennis de poep schepper
/// 62 jaar oud
/// €676 per week
/// woont in een grot
///

impl Randaam {
    pub fn gen() -> Self {
        Randaam::build(person(), age(), location(), salary(), rarity(), emoji())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_name() {
        assert!(name().len() > 0);
    }

    #[test]
    pub fn test_object() {
        assert!(object().len() > 0);
    }

    #[test]
    pub fn test_action() {
        assert!(action().len() > 0);
    }

    #[test]
    pub fn test_location() {
        assert!(location().len() > 0);
    }

    #[test]
    pub fn test_person() {
        assert!(person().contains(" de "));
    }

    #[test]
    pub fn test_age() {
        let age = age();
        assert!(age > 0 && 101 > age);
    }

    #[test]
    #[should_panic]
    pub fn too_high_for_rarity() {
        decide_rarity(10001);
    }

    #[test]
    #[should_panic]
    pub fn too_low_for_rarity() {
        decide_rarity(0);
    }

    #[test]
    pub fn decide_rarity_works() {
        assert_eq!(decide_rarity(9901), Rarity::Epic);
        assert_eq!(decide_rarity(10000), Rarity::SuperLegendary);
        assert_eq!(decide_rarity(9991), Rarity::Legendary);
        assert_eq!(decide_rarity(9001), Rarity::Rare);
        assert_eq!(decide_rarity(1), Rarity::Normal);
    }

    #[test]
    pub fn test_salary() {
        let sample = salary();
        assert!(sample > 0 && 2001 > sample);
    }
}
