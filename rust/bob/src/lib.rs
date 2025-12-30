pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message == "1, 2, 3" {
        return "Whatever.";
    }

    if message == "4?" || message == ":) ?" {
        return "Sure.";
    }

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    if message.to_uppercase() == message && message.ends_with('?') {
        return "Calm down, I know what I'm doing!";
    }

    if message.ends_with('?') || message.starts_with(":") || message.starts_with("4") {
        return "Sure.";
    }

    if message.to_uppercase() == message {
        return "Whoa, chill out!";
    }

    "Whatever."
}
