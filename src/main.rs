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

    println!(
        "{}",
        check::sentence_length::sentence_length(content.clone())
    );

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
