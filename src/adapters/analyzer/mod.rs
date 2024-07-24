use any_ascii::any_ascii;
use regex::Regex;
pub fn is_quebec(text: &String) -> bool {
    let keywords = [
        "quebec",
        "montreal",
        "polqc",
        "cegep",
        "uqam",
        "uqac",
        "uqtr",
        "radio-canada",
        "followqc",
        "suivreqc",
        "followquebec",
        "suivrequebec",
        "tiohtia:ke",
        "#mtl",
        "qc",
        "#littqc",
        "#bdqc",
        "🇲🇶",
        "lacstJean",
        "rimouski",
        "chicoutimi",
    ];

    for keyword in keywords {
        let re = Regex::new(keyword).unwrap();

        let text = any_ascii(text).to_ascii_lowercase();

        if re.is_match(&text) {
            return true;
        }
    }

    return false;
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_is_quebec() {
        let examples = [
            "Quebec",
            "MOntreal",
            "montreal",
            "Montréal",
            "UQTR",
            "followqc",
            "Chicoutimi",
        ];

        examples.iter().for_each(|example| {
            assert_eq!(is_quebec(&String::from(*example)), true);
        });
    }
}
