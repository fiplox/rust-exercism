// check if m has at least one letter
fn has_letter(m: &str) -> bool {
    for c in m.chars() {
        if (c > 'a' && c < 'z') || (c > 'A' && c < 'Z') {
            return true;
        }
    }
    false
}

pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    if message.chars().last().unwrap() == '?' {
        if message == message.to_uppercase() {
            if !has_letter(message) {
                return "Sure.";
            }
            return "Calm down, I know what I'm doing!";
        }
        return "Sure.";
    }

    if message == message.to_uppercase() {
        if !has_letter(message) {
            return "Whatever.";
        }
        return "Whoa, chill out!";
    }

    "Whatever."
}
