//! Simple example of using BudouX Rust Wrapper

fn main() {
    // Load the default Japanese parser
    let parser = budoux_rust_wrapper::load_default_japanese_parser();

    // Example sentences
    let sentences = [
        "今日は天気です。",
        "本日は晴天です。",
        "私は遅刻魔で、待ち合わせにいつも遅刻してしまいます。",
        "メールで待ち合わせ相手に一言、「ごめんね」と謝ればどうにかなると思っていました。",
        "海外ではケータイを持っていない。",
    ];

    // Process each sentence
    for sentence in sentences {
        println!("Original: {}", sentence);

        // Parse the sentence
        let chunks = parser.parse(sentence);

        // Print the chunks
        println!("Chunks: {:?}", chunks);

        // Print the chunks one per line
        println!("Formatted:");
        for chunk in chunks {
            println!("  {}", chunk);
        }

        println!();
    }
}
