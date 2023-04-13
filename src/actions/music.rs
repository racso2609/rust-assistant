pub mod google;
use google::youtube::YoutubeSearchResponse;
use std::process::Command;

fn open_browser(url: &str) {
    let mut openurl = Command::new("xdg-open");
    openurl.arg(url);
    openurl.status().expect("fail opening browser");
}

fn format_url(command: &str) -> String {
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

#[tokio::main]
pub async fn reproduce(command: &str) {
    _reproduce(command, Some("youtube"));
}

async fn _reproduce(command: &str, name: Option<&str>) {
    let url = format_url(command);

    let res = YoutubeSearchResponse::get(&url).await.expect("Fail");
    let open_link = YoutubeSearchResponse::get_watch_url(&res.items[0].id.videoId);

    open_browser(&open_link);
}
