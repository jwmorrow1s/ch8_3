pub trait PigLatin {
    fn to_pig_latin(&self) -> Self;
}

#[allow(dead_code)]
#[inline]
fn first_letter_off_add_ay(s: &String) -> String {
    if s.len() == 0 {
        return String::from("");
    }

    if is_vowel(get_first_char(s).unwrap()) {
        format!("{}-hay", s)
    } else {
        format!(
            "{}-{}ay",
            remove_first_char(s),
            get_first_char(s).unwrap().to_lowercase()
        )
    }
}

#[allow(dead_code)]
#[inline]
fn get_first_char(s: &String) -> Option<char> {
    let first_word: Option<&str> = s.split_whitespace().next();

    let first_word = match first_word {
        None => return None,
        Some(word) => word,
    };

    first_word.chars().next()
}

#[allow(dead_code)]
#[inline]
fn remove_first_char(s: &String) -> String {
    let mut first_char_removed = String::new();
    for (iter, c) in s.chars().enumerate() {
        if iter > 0 {
            first_char_removed.push(c)
        }
    }

    first_char_removed
}

#[allow(dead_code)]
#[inline]
fn is_vowel(c: char) -> bool {
    ['A', 'E', 'I', 'O', 'U'].contains(&c.to_ascii_uppercase())
}

impl PigLatin for String {
    fn to_pig_latin(&self) -> Self {
        String::from(
            self.split_whitespace()
                .map(|word| format!("{} ", first_letter_off_add_ay(&String::from(word))))
                .collect::<String>()
                .trim(),
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_vowel_it_works() {
        assert_eq!(is_vowel('A'), true);
        assert_eq!(is_vowel('e'), true);
        assert_eq!(is_vowel('B'), false);
        assert_eq!(is_vowel('c'), false);
    }

    #[test]
    fn remove_first_char_it_works() {
        assert_eq!(remove_first_char(&String::from("Apple")), "pple");
        assert_eq!(remove_first_char(&String::from("")), "");
    }

    #[test]
    fn get_first_char_it_works() {
        assert_eq!(get_first_char(&String::from("Apple")), Some('A'));
        assert_eq!(get_first_char(&String::from("")), None);
    }

    #[test]
    fn first_letter_off_add_ay_it_works() {
        assert_eq!(first_letter_off_add_ay(&String::from("Apple")), "Apple-hay");
        assert_eq!(
            first_letter_off_add_ay(&String::from("Pancakes")),
            "ancakes-pay"
        );
        assert_eq!(first_letter_off_add_ay(&String::from("")), "");
    }

    #[test]
    fn to_pig_latin_it_works() {
        assert_eq!(
            String::from("Pigs in the hay").to_pig_latin(),
            "igs-pay in-hay he-tay ay-hay"
        );
    }
}
