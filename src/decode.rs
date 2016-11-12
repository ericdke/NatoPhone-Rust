use std::collections::HashMap;

pub fn translated(words: Vec<String>) -> String {
    translate(words).join("")
}

fn translate(words: Vec<String>) -> Vec<String> {
    let dic: HashMap<&str, &str> = [("alpha", "a"),
                                    ("bravo", "b"),
                                    ("charlie", "c"),
                                    ("delta", "d"),
                                    ("echo", "e"),
                                    ("foxtrot", "f"),
                                    ("golf", "g"),
                                    ("hotel", "h"),
                                    ("india", "i"),
                                    ("juliet", "j"),
                                    ("kilo", "k"),
                                    ("lima", "l"),
                                    ("mike", "m"),
                                    ("november", "n"),
                                    ("oscar", "o"),
                                    ("papa", "p"),
                                    ("quebec", "q"),
                                    ("romeo", "r"),
                                    ("sierra", "s"),
                                    ("tango", "t"),
                                    ("uniform", "u"),
                                    ("victor", "v"),
                                    ("whiskey", "w"),
                                    ("x-ray", "x"),
                                    ("yankee", "y"),
                                    ("zulu", "z"),
                                    ("zero", "0"),
                                    ("one", "1"),
                                    ("two", "2"),
                                    ("three", "3"),
                                    ("four", "4"),
                                    ("five", "5"),
                                    ("six", "6"),
                                    ("seven", "7"),
                                    ("eight", "8"),
                                    ("niner", "9"),
                                    ("stop", ".")]
        .iter()
        .cloned()
        .collect();

    let mut bucket: Vec<String> = Vec::new();

    for word in words {
        let low = word.to_lowercase();
        let lref: &str = low.as_ref();
        let letter = match dic.get(lref) {
            Some(l) => l.to_string(),
            None => " ".to_string(),
        };
        bucket.push(letter);
    }

    bucket
}