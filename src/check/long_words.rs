use std::collections::HashMap;

pub fn long_words(content: String) -> Vec<String> {
    let mut errors = Vec::new();

    let words: HashMap<&str, &str> = [
        ("approximately", "about"),
        ("demonstrate", "show"),
        ("establish", "set up"),
        ("expenditure", "spending"),
        ("facility", "plant, club, warehouse, etc"),
        ("following", "after"),
        ("however", "but"),
        ("manufacture", "make"),
        ("participate", "take part"),
        ("permit", "let"),
        ("prior to", "before"),
        ("sufficient", "enough"),
        ("utilise", "use"),
    ]
    .iter()
    .cloned()
    .collect();

    for (long_word, short_word) in &words {
        if content.find(long_word).is_some() {
            errors.push(format!("\t{} > {}", long_word, short_word));
        }
    }

    if !errors.is_empty() {
        errors.insert(0, String::from("\nUse simple words"));
    }

    return errors;
}

#[test]
fn test_long_words() {
    let mut sorted_result = long_words(String::from(
        "I'll be home in approximately 20 minutes. I manufacture no excuses.",
    ));

    sorted_result.sort();
    assert_eq!(
        sorted_result,
        vec![
            "\tapproximately > about",
            "\tmanufacture > make",
            "\nUse simple words",
        ]
    );

    let empty_vec: Vec<String> = Vec::new();

    assert_eq!(
        long_words(String::from("I'll be home in about 20 minutes.")),
        empty_vec
    );
}
