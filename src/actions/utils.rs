use std::process::Command;

pub fn open_browser(url: &str) {
    let mut openurl = Command::new("xdg-open");
    openurl.arg(url);
    openurl.status().expect("fail opening browser");
}

pub fn format_url(command: &str) -> String {
    let mut url = String::from(""); // youtube search url
    let command_by_word: Vec<&str> = command.split_whitespace().collect();

    for (index, word) in command_by_word.clone().into_iter().enumerate() {
        if word != "" {
            url.push_str(word);
            if index < command_by_word.len() - 1 {
                url.push_str("+");
            }
        }
    }
    return url;
}
