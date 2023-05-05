use serde::{Deserialize, Serialize};
use exitfailure::ExitFailure;

#[derive(Serialize, Deserialize, Debug)]
pub struct YoutubeSearchResponse {
    pub kind: String,
    pub etag: String,
    pub nextPageToken: String,
    pub pageInfo: YoutubePageInfo,
    pub items: Vec<YoutubeSearchItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YoutubePageInfo {
    pub totalResults: i32,
    pub resultsPerPage: i16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YoutubeSearchItem {
    pub kind: String,
    pub etag: String,
    pub id: YoutubeIdentifier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YoutubeIdentifier {
    pub kind: String,
    pub videoId: String
}

impl YoutubeSearchResponse {
    pub fn get_watch_url(video_id: &str) -> String {
        let mut url = String::from("https://www.youtube.com/watch?v=");
        url.push_str(video_id);

        return url;
    }
    pub async fn get(search_key: &str) -> Result<Self, ExitFailure> {
        let key = std::env::var("YT_API_KEY").expect("YT_API_KEY env-var not found");
        let mut url = String::from("https://www.googleapis.com/youtube/v3/search?q=");
        url.push_str(search_key);
        url.push_str("&key=");
        url.push_str(&key);

        let res = reqwest::get(url)
            .await?
            .json::<YoutubeSearchResponse>()
            .await?;

        Ok(res)
    }
}
