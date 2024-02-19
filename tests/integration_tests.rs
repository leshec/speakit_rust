use speakit_rust::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_clean_word() {
        assert_eq!(clean_word("cielo"), Some("cielo".to_string()));
        assert_eq!(clean_word("perciò"), Some("perciò".to_string()));
        assert_eq!(clean_word("Camicia!"), Some("camicia".to_string()));
        assert_eq!(clean_word("123"), None);
        assert_eq!(clean_word("l'ufficio"), None);
        assert_eq!(clean_word("ka-ban"), None);
    }

    #[test]
    fn test_count_frequency() {
        let lines = vec![
            "This is a test file for checking the functionality of the code.",
            "It contains some words like cielo, camicia, cielo, civile, perciò, etc.",
            "We will use this file to test if the code correctly retrieves words containing a given substring."
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let freq = count_frequency(&lines);

        assert_eq!(freq.get("cielo"), Some(&2));
        assert_eq!(freq.get("camicia"), Some(&1));
        assert_eq!(freq.get("perciò"), Some(&1));
        assert_eq!(freq.get("balloon"), None); // Testing a non-existing word
        assert_eq!(freq.get("this"), Some(&2)); // Testing case-insensitivity
    }

    #[test]
    fn test_get_matching_words() {
        let freq: HashMap<String, usize> = [
            ("cielo".to_string(), 3),
            ("camicia".to_string(), 2),
            ("finito".to_string(), 4),
            ("finita".to_string(), 9),
            ("perciò".to_string(), 1),
            ("due".to_string(), 1),
            ("settimana".to_string(), 3),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(
            get_matching_words(&freq, "ci"),
            vec![&"cielo".to_string(), &"camicia".to_string(), &"perciò".to_string()]
        );
        assert_eq!(get_matching_words(&freq, "ue"), vec![&"due".to_string()]);
        assert_eq!(get_matching_words(&freq, "it"), vec![&"finita".to_string(), &"finito".to_string()]);
        assert_eq!(get_matching_words(&freq, "a"), vec![&"finita".to_string(), &"settimana".to_string(), &"camicia".to_string()]);
    }
}

