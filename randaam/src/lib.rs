use rand::seq::SliceRandom; // 0.7.2

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
        "Daniële",
        "Elias",
        "Carlijn",
        "Caroline",
        "Lisanne",
        "Marije",
        "Tirza",
        "Naomi",
        "Joyce",
        "Indy",
        "Marith",
        "Micha",
        "Hayanne",
        "Isa",
        "Nadine",
        "Aron",
    ];

    names.choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn object() -> String {
    let objects = vec!["stoep", "dinosaurus", "klokken", "lampen", "mieren", "appelmoes", "poep", "bank", "goudvis", "ramen", "pizza", "banannen", "blopvis", "pennen", "bomen", "schoenen", "fietspompen", "fietsen", "deur", "bezem", "eekhoorn", "muis", "tafel", "cavia", "koek", "luiaart", "paarden", "melkpak", "chocola", "bananentros", "cavia", "brulaap", "duiven", "ezel", "geiten", "neushoorn", "zwijnen", "struisvogel", "varken", "parkiet", "stokbrood", "boeken"];
    objects.choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn action() -> String {
    let actions = vec!["inspecteur", "zitter", "liefhebber", "visser", "schepper", "specialist", "schoonmaker", "verkoper", "wasser", "bakker", "temmer", "progammeur", "vlogger"];
    actions.choose(&mut rand::thread_rng()).unwrap().to_string()
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
}
