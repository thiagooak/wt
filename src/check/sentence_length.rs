pub fn sentence_length(content: String) -> String {
    let sentences: Vec<&str> = content.split_terminator('.').collect();

    let mut result = String::new();
    let mut short_sentences_count: f64 = 0.0;
    let mut long_sentences = Vec::new();

    for sentence in sentences {
        let word_count = sentence.split_whitespace().count();

        if word_count <= 18 {
            short_sentences_count += 1.0;
        } else {
            long_sentences.push(sentence);
        }
    }

    let long_sentences_count: f64 = long_sentences.len() as f64;

    let percent_long_sentences: f64 =
        long_sentences_count / (short_sentences_count + long_sentences_count) * 100.0;

    if percent_long_sentences > 10.0 {
        result.push_str("\nMore than 10% of your sentences have > 18 words");
        for sentence in long_sentences {
            result.push_str("\n\t");
            result.push_str(&sentence[..70]);
            result.push_str("...");
        }
    }

    return result;
}

#[test]
fn test_check_sentence_length() {
    assert_eq!(
        sentence_length(String::from("Vivamus lacinia, libero ac lobortis iaculis, dui dui malesuada diam, eu interdum tellus risus nec leo class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Pellentesque vel elit maximus, suscipit turpis a, elementum turpis. Vivamus sollicitudin arcu sit amet elementum fermentum. Vestibulum sed velit in dolor molestie congue. Vestibulum dui quam, pharetra non egestas id, ullamcorper et mauris. Vestibulum blandit felis quis ligula finibus commodo.")),
        String::from("\nMore than 10% of your sentences have > 18 words\n\tVivamus lacinia, libero ac lobortis iaculis, dui dui malesuada diam, e...")
    );

    assert_eq!(
        sentence_length(String::from("Vivamus sollicitudin arcu sit amet elementum fermentum. Vestibulum sed velit in dolor molestie congue. Vestibulum dui quam, pharetra non egestas id, ullamcorper et mauris. Vestibulum blandit felis quis ligula finibus commodo.")),
        String::from("")
    );
}
