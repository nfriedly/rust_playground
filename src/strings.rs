use unicode_segmentation::UnicodeSegmentation;
use regex::Regex;


pub fn print_chars(string: &str) {
    for c in string.chars() {
        println!("{}", c);
    }
}

pub fn print_graphmems(string: &str) {
    let gphs = UnicodeSegmentation::graphemes(string, true).collect::<Vec<&str>>();
    for gph in gphs {
        println!("{}", gph);
    }
}

// Convert strings to Pig Latin, where the first consonant of each word is moved to
// the end of the word with an added “ay”, so “first” becomes “irst-fay”. Words that
// start with a vowel get “hay” added to the end instead (“apple” becomes “apple-hay”).
// Remember about UTF-8 encoding!
// todo: clean up the various str/string conversions
pub fn pig_latin(s: String) -> String {
    let re_vowel = Regex::new(r"^(?i)[aeiouy]").unwrap();
    let re_char = Regex::new(r"^(?i)[a-z]").unwrap(); // todo: make this match boundaries instead
    let mut cary: Option<char> = None;
    let mut output = String::new();

    for c in s.chars() {
        let was_in_word = cary.is_some();
        let now_in_word = re_char.is_match(&c.to_string());

        if !was_in_word && now_in_word {
            // start of word: is it a vowel?
            if re_vowel.is_match(&c.to_string()) {
                // - yes: emit it and carry 'h'
                cary = Some('h');
                output.push(c);
            } else {
                // - no: cary it
                cary = Some(c);
            }
        } else if was_in_word && !now_in_word {
            // end of word: emit -cary + ay + boundary char
            output += &format!("-{}ay{}", cary.unwrap(), c)[..];
            cary = None;
        } else {
            // either in a word or in a series of boundary characters
            output.push(c);
        }
    }
    if let Some(c) = cary {
        output += &format!("-{}ay", c)[..];
    }
    output
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_str () {
        let actual = pig_latin(String::from(""));
        let expected = String::from("");
        assert_eq!(actual, expected);
    }

    #[test]
    fn word () {
        let actual = pig_latin(String::from("first"));
        let expected = String::from("irst-fay");
        assert_eq!(actual, expected);
    }

    #[test]
    fn starting_vowel () {
        let actual = pig_latin(String::from("apple"));
        let expected = String::from("apple-hay");
        assert_eq!(actual, expected);
    }

    #[test]
    fn phrase () {
        let actual = pig_latin(String::from("I get to eat the apple first"));
        let expected = String::from("I-hay et-gay o-tay eat-hay he-tay apple-hay irst-fay");
        assert_eq!(actual, expected);
    }

    #[test]
    fn sentence_with_punctuation () {
        let actual = pig_latin(String::from("I, me, get to eat the apple first!"));
        let expected = String::from("I-hay, e-may, et-gay o-tay eat-hay he-tay apple-hay irst-fay!");
        assert_eq!(actual, expected);
    }

    #[test]
    fn parens () {
        let actual = pig_latin(String::from("I (me) get to eat the apple first!"));
        let expected = String::from("I-hay (e-may) et-gay o-tay eat-hay he-tay apple-hay irst-fay!");
        assert_eq!(actual, expected);
    }
}
