use rand::seq::SliceRandom; // 0.7.2
use rand::Rng;

pub struct Randaam {
    pub person: String,
    pub age: u8,
    pub location: String,
    pub salary: u32,
    pub rarity: Rarity,
    pub emoji: char,
}

impl Randaam {
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

    pub fn print(r: Self) {
        println!("{} \n\n", Rarity::format(r.rarity));
        println!("{}", r.emoji);
        println!("{}", r.person);
        println!(" {} jaar oud", r.age);
        println!("€{} per week", r.salary);
        println!("woont in een {}", r.location);
    }
}

#[derive(PartialEq, Debug)]
pub enum Rarity {
    Normal,
    Rare,
    Epic,
    Legendary,
    SuperLegendary,
}

impl Rarity {
    pub fn format(r: Self) -> String {
        match r {
            Rarity::SuperLegendary => String::from("SUPER LEGENDARISCH!!!"),
            Rarity::Legendary => String::from("Legendarisch!!"),
            Rarity::Epic => String::from("Episch!"),
            Rarity::Rare => String::from("Zeldzaam"),
            Rarity::Normal => String::from("normaal"),
        }
    }
}

pub fn name() -> String {
    let names = include_str!("./content/names.txt")
        .lines()
        .collect::<Vec<_>>();
    names.choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn object() -> String {
    let objects = vec![
        "stoep",
        "dinosaurus",
        "klokken",
        "lampen",
        "mieren",
        "appelmoes",
        "poep",
        "bank",
        "goudvis",
        "ramen",
        "pizza",
        "banannen",
        "blopvis",
        "pennen",
        "bomen",
        "schoenen",
        "fietspompen",
        "fietsen",
        "deur",
        "bezem",
        "eekhoorn",
        "muis",
        "tafel",
        "cavia",
        "koek",
        "luiaart",
        "paarden",
        "melkpak",
        "chocola",
        "bananentros",
        "cavia",
        "brulaap",
        "duiven",
        "ezel",
        "geiten",
        "neushoorn",
        "zwijnen",
        "struisvogel",
        "varken",
        "parkiet",
        "stokbrood",
        "boeken",
    ];
    objects.choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn action() -> String {
    let actions = vec![
        "inspecteur",
        "zitter",
        "liefhebber",
        "visser",
        "schepper",
        "specialist",
        "schoonmaker",
        "verkoper",
        "wasser",
        "bakker",
        "temmer",
        "progammeur",
        "vlogger",
    ];
    actions.choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn person() -> String {
    format!("{} de {} {}", name(), object(), action())
}

pub fn emoji() -> char {
    //TODO: add more emojis
    let emojis = vec!['😀', '😃', '😄', '😁', '😆', '😅'];
    emojis.choose(&mut rand::thread_rng()).unwrap().clone()
}

pub fn location() -> String {
    let locations = [
        "huis",
        "grot",
        "hutje",
        "auto",
        "boomhut",
        "helicopter",
        "snoepautomaat",
        "kast",
        "flat",
        "snackbar",
        "school",
        "doos",
        "fabriek",
        "boot",
    ];

    locations
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
}

pub fn age() -> u8 {
    rand::thread_rng().gen_range(1..101)
}

pub fn rarity() -> Rarity {
    let rarity_num = rand::thread_rng().gen_range(1..10001);
    decide_rarity(rarity_num)
}

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

// salary per week
pub fn salary() -> u32 {
    rand::thread_rng().gen_range(1..2001)
}

pub fn gen_randaam() -> Randaam {
    Randaam::build(person(), age(), location(), salary(), rarity(), emoji())
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
