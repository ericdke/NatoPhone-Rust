use std::collections::HashMap;

pub fn translated(words: Vec<String>) -> String {
    translate(words).join("")
}

fn translate(words: Vec<String>) -> Vec<String> {
    let dic: HashMap<&str, &str> = [("a", "alpha"),
                                    ("b", "bravo"),
                                    ("c", "charlie"),
                                    ("d", "delta"),
                                    ("e", "echo"),
                                    ("f", "foxtrot"),
                                    ("g", "golf"),
                                    ("h", "hotel"),
                                    ("i", "india"),
                                    ("j", "juliet"),
                                    ("k", "kilo"),
                                    ("l", "lima"),
                                    ("m", "mike"),
                                    ("n", "november"),
                                    ("o", "oscar"),
                                    ("p", "papa"),
                                    ("q", "quebec"),
                                    ("r", "romeo"),
                                    ("s", "sierra"),
                                    ("t", "tango"),
                                    ("u", "uniform"),
                                    ("v", "victor"),
                                    ("w", "whiskey"),
                                    ("x", "x-ray"),
                                    ("y", "yankee"),
                                    ("z", "zulu"),
                                    ("0", "zero"),
                                    ("1", "one"),
                                    ("2", "two"),
                                    ("3", "three"),
                                    ("4", "four"),
                                    ("5", "five"),
                                    ("6", "six"),
                                    ("7", "seven"),
                                    ("8", "eight"),
                                    ("9", "niner"),
                                    (".", "stop")]
        .iter()
        .cloned()
        .collect();

    let mut bucket: Vec<String> = Vec::new();
    let mut count = 0;
    let len = words.len();

    for word in &words {
        count += 1;
        let low = word.to_lowercase();
        let ref chs: Vec<&str> = low.split("").collect();
        for c in chs {
            if c == &"" {
                continue;
            }
            let nat = match dic.get(c) {
                Some(w) => w.to_string().to_uppercase(),
                None => "".to_string(),
            };
            if nat != "" {
                bucket.push(nat);
                bucket.push(" ".to_string());
            }
        }
        if len > 1 && len != count {
            bucket.push("- ".to_string());
        }
    }

    bucket
}