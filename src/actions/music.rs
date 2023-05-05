use super::google::youtube::YoutubeSearchResponse;
use super::utils::{open_browser, format_url};

#[tokio::main]
pub async fn reproduce(command: &str) {
    _reproduce(command, Some("youtube")).await;
}

async fn _reproduce(command: &str, _name: Option<&str>) {
    let url = format_url(command);

    let res = YoutubeSearchResponse::get(&url).await.expect("Fail");
    let open_link = YoutubeSearchResponse::get_watch_url(&res.items[0].id.videoId);

    open_browser(&open_link);
}
