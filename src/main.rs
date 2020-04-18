use regex::Regex;
use structopt::StructOpt;

mod check;

/// Writing Tools
#[derive(StructOpt)]
struct Cli {
    /// The path to the file you want to check
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    /// HTML mode
    #[structopt(short = "H")]
    html: bool,
}

fn main() {
    let args = Cli::from_args();

    let raw = std::fs::read_to_string(&args.path).unwrap();
    let content = prepare_content(raw, args.html);

    println!("{}", check_sentence_length(content.clone()));

    let long_words_errors = check::long_words::long_words(content.clone());
    for error in long_words_errors {
        println!("{}", error);
    }
}

fn prepare_content(raw: String, html: bool) -> String {
    if html {
        // replacing every html tag with a period
        // this is very wrong but is a simple way to get strated
        // ideally this would convert some tags to periods </p></div> (e.g. tags that by default are blocks)
        // and replace other with nothing </strong></a>
        let re = Regex::new(r"(<[^>]*>)").unwrap();
        return re.replace_all(&raw, ".").into_owned(); // I don't understand what into_owned does! But this works
    }

    return raw;
}

#[test]
fn test_prepare_content() {
    let content = String::from(
        "<!doctype html>
<html class=\"no-js\" lang=\"\">

<head><meta charset=\"utf-8\"><title></title></head>

<body>
    <!-- Add your site or application content here -->
    <p>Hello world! This is HTML5 Boilerplate.</p>
    <script src=\"js/main.js\"></script>
</body>
</html>
",
    );

    assert_eq!(
        prepare_content(content.clone(), true),
        String::from(".\n.\n\n.....\n\n.\n    .\n    .Hello world! This is HTML5 Boilerplate..\n    ..\n.\n.\n")
    );

    assert_eq!(prepare_content(content.clone(), false), content.clone());
}

fn check_sentence_length(content: String) -> String {
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
        check_sentence_length(String::from("Vivamus lacinia, libero ac lobortis iaculis, dui dui malesuada diam, eu interdum tellus risus nec leo class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Pellentesque vel elit maximus, suscipit turpis a, elementum turpis. Vivamus sollicitudin arcu sit amet elementum fermentum. Vestibulum sed velit in dolor molestie congue. Vestibulum dui quam, pharetra non egestas id, ullamcorper et mauris. Vestibulum blandit felis quis ligula finibus commodo.")),
        String::from("\nMore than 10% of your sentences have > 18 words\n\tVivamus lacinia, libero ac lobortis iaculis, dui dui malesuada diam, e...")
    );

    assert_eq!(
        check_sentence_length(String::from("Vivamus sollicitudin arcu sit amet elementum fermentum. Vestibulum sed velit in dolor molestie congue. Vestibulum dui quam, pharetra non egestas id, ullamcorper et mauris. Vestibulum blandit felis quis ligula finibus commodo.")),
        String::from("")
    );
}
