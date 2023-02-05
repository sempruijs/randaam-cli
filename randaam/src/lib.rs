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
}

#[derive(PartialEq, Debug)]
pub enum Rarity {
    Normal,
    Rare,
    Epic,
    Legendary,
    SuperLegendary,
}

pub fn name() -> String {
    let names = vec![
        "Harry",
        "Bert",
        "Henk",
        "Sebastiaan",
        "Papa",
        "Rutger",
        "Mama",
        "Lars",
        "Steven",
        "Jannes",
        "Iwan",
        "Milan",
        "Bertwim",
        "Amgela",
        "Jorgiam",
        "Hansje",
        "Pim",
        "Sandra",
        "Hennie",
        "Achmet",
        "Dirkjan",
        "Barbaba",
        "Jeroen",
        "Riejan",
        "Willem",
        "Leo",
        "Meneer",
        "Bas",
        "Dennis",
        "Jan",
        "Kees",
        "Lies",
        "Sem",
        "Jan",
        "Cor",
        "Lucas",
        "Finn",
        "Daan",
        "Levi",
        "Milan",
        "Bram",
        "Luuk",
        "Emma",
        "Tess",
        "Sophie",
        "Julia",
        "Anna",
        "Mila",
        "Sara",
        "Luuk",
        "Jesse",
        "Mees",
        "Thomas",
        "Sam",
        "Thijs",
        "Adam",
        "Max",
        "Jullian",
        "Hugo",
        "Gijs",
        "Benjamin",
        "Tim",
        "Ruben",
        "Teun",
        "Olivier",
        "Sven",
        "David",
        "Stijn",
        "Tom",
        "Isa",
        "Noa",
        "Jasmijn",
        "Esmee",
        "Sanne",
        "Joël",
        "Emmanuel",
        "Nathan",
        "Tobias",
        "Alex",
        "Eline",
        "Daniële",
        "Elias",
        "Carlijn",
        "Caroline",
        "Lisanne",
        "Evie",
        "Marije",
        "Tirza",
        "Naomi",
        "Joyce",
        "Indy",
        "Marith",
        "Micha",
        "Anne",
        "Hayanne",
        "Isa",
        "Nadine",
        "Aron",
        "Janneke",
    ];

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
