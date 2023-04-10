mod youtube;
use std::process::Command;
use youtube::YoutubeSearchResponse;

pub const MUSIC: &str = "reproduce";

pub struct Action {
    pub action: String,
    pub command: String,
}

fn open_browser(url: &str) {
    let mut openurl = Command::new("xdg-open");
    openurl.arg(url);
    openurl.status().expect("fail opening browser");
}

#[tokio::main]
pub async fn reproduce(command: &str) {
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
    let res = YoutubeSearchResponse::get(&url).await.expect("Fail");
    let open_link = YoutubeSearchResponse::get_watch_url(&res.items[0].id.videoId);

    open_browser(&open_link);
}

pub fn execute(action: Action) {
    match action.action.as_str() {
        MUSIC => reproduce(action.command.as_str()),
        _ => (),
    }
}
