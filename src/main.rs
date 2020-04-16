use structopt::StructOpt;

/// Writing Tools
#[derive(StructOpt)]
struct Cli {
    /// The path to the file you want to check
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.path).unwrap();

    let sentences: Vec<&str> = content.split_terminator('.').collect();

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

    let percent_long_sentences: f64 =  long_sentences_count / (short_sentences_count + long_sentences_count) * 100.0;

    if percent_long_sentences > 10.0 {
        println!("\nMore than 10% of your sentences have > 18 words.");
        for sentence in long_sentences {
            println!("\t{}\n", sentence);
        }
    }

    println!("\n{} short sentences and {} long ones. {:.1}%",
        short_sentences_count,
        long_sentences_count,
        percent_long_sentences
    );
}
