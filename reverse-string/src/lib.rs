use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut output = "".to_string();

    for c in input.graphemes(true).rev() {
        output += c;
    }

    output.trim_matches(char::from(0)).to_string()
}
