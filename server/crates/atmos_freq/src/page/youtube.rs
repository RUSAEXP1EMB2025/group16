use crate::AtmosFreq;
use atmos_config::Config;
use atmos_dict::Atmosdict;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use url::Url;

#[derive(Debug, Deserialize)]
struct YouTubeResponse {
    items: Vec<VideoItem>,
}

#[derive(Debug, Deserialize)]
struct VideoItem {
    snippet: VideoSnippet,
}

#[derive(Debug, Deserialize)]
struct VideoSnippet {
    title: String,
    tags: Option<Vec<String>>,
    description: String,
    #[serde(rename = "categoryId")]
    category_id: String,
}

// 結果を格納する構造体
#[derive(Debug, Serialize)]
struct VideoInfo {
    title: String,
    tags: Vec<String>,
    description: String,
    category_id: String,
}

struct YouTubeClient {
    api_key: String,
    client: reqwest::Client,
}

impl YouTubeClient {
    fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    /// YouTubeのURLからvideo IDを抽出する
    fn extract_video_id(&self, url: &Url) -> Result<String, Box<dyn Error>> {
        match url.host_str() {
            Some("www.youtube.com") | Some("youtube.com") => {
                // 通常のYouTubeURL: https://www.youtube.com/watch?v=VIDEO_ID
                if let Some(query) = url.query() {
                    for (key, value) in url::form_urlencoded::parse(query.as_bytes()) {
                        if key == "v" {
                            return Ok(value.to_string());
                        }
                    }
                }
                Err("Video ID not found in URL query".into())
            }
            Some("youtu.be") => {
                // 短縮URL: https://youtu.be/VIDEO_ID
                if let Some(mut path) = url.path_segments() {
                    if let Some(video_id) = path.next_back() {
                        return Ok(video_id.to_string());
                    }
                }
                Err("Video ID not found in shortened URL".into())
            }
            _ => Err("Not a valid YouTube URL".into()),
        }
    }

    /// YouTube Data APIを使って動画情報を取得する
    async fn get_video_info(&self, video_id: &str) -> Result<VideoInfo, Box<dyn Error>> {
        // APIキーの検証
        if self.api_key.is_empty() {
            return Err("YouTube API key is not set. Please set YOUTUBE_API_KEY environment variable or provide a valid API key.".into());
        }

        let url = format!(
            "https://www.googleapis.com/youtube/v3/videos?id={}&part=snippet&key={}",
            video_id, self.api_key
        );

        println!(
            "Making API request to: {}",
            url.replace(&self.api_key, "***")
        );

        let response = self.client.get(&url).send().await?;
        let status = response.status();

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Could not read response body".to_string());
            return Err(
                format!("API request failed: {} - Response: {}", status, error_text).into(),
            );
        }

        let response_text = response.text().await?;
        //println!("API Response: {}", response_text);

        let youtube_response: YouTubeResponse = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse JSON response: {}", e))?;

        if youtube_response.items.is_empty() {
            return Err("Video not found or video is private/deleted".into());
        }

        let snippet = &youtube_response.items[0].snippet;

        Ok(VideoInfo {
            title: snippet.title.clone(),
            tags: snippet.tags.clone().unwrap_or_default(),
            description: snippet.description.clone(),
            category_id: snippet.category_id.clone(),
        })
    }

    /// URLから動画情報を取得する（wrapper関数）
    async fn get_video_info_from_url(&self, url: &Url) -> Result<VideoInfo, Box<dyn Error>> {
        let video_id = self.extract_video_id(url)?;
        self.get_video_info(&video_id).await
    }
}

fn coversion_category(id: i32) -> Option<&'static str> {
    match id {
        1 => Some("映画とアニメ"),
        2 => Some("自動車と乗り物"),
        10 => Some("音楽"),
        15 => Some("ペットと動物"),
        17 => Some("スポーツ"),
        19 => Some("旅行とイベント"),
        20 => Some("ゲーム"),
        22 => Some("ブログ"),
        23 => Some("コメディー"),
        24 => Some("エンターテイメント"),
        25 => Some("ニュースと政治"),
        26 => Some("ハウツーとスタイル"),
        27 => Some("教育"),
        28 => Some("科学と技術"),
        29 => Some("非営利団体と社会活動"),
        _ => None, // ☓ になっているIDや未定義のIDは None を返す
    }
}

impl AtmosFreq {
    pub async fn from_youtube(url: &Url, atmosdict: &Atmosdict) -> Self {
        let config = Config::from_env();
        let client = YouTubeClient::new(config.youtube_api_key);
        let video_info = client.get_video_info_from_url(url).await.unwrap();
        let category = coversion_category(video_info.category_id.parse().unwrap()).unwrap();

        let mut infos = Vec::<String>::new();
        infos.push(video_info.title);
        infos.push(video_info.description);
        for tag in video_info.tags {
            infos.push(tag);
        }
        infos.push(category.to_string());

        let (pos_dict, neg_dict) = atmosdict.get_pos_neg().await.unwrap();

        let mut pos_count = 0;
        let mut neg_count = 0;

        for info in infos {
            for pos_word in &pos_dict {
                if info.contains(pos_word) {
                    pos_count += 1;
                    dbg!(pos_word);
                }
            }
            for neg_word in &neg_dict {
                if info.contains(neg_word) {
                    neg_count += 1;
                }
            }
        }
        let total = pos_count + neg_count;

        let score = (pos_count as f64 / total as f64) * 100.0;
        AtmosFreq(score)
    }
}

#[cfg(test)]
mod test {
    use atmos_config::Config;
    use atmos_dict::Atmosdict;
    use url::Url;

    use crate::AtmosFreq;

    #[tokio::test]
    async fn test_generate_atmosfreq_from_youtube() {
        let config = Config::from_env();
        let atmosdict = Atmosdict::new(&config.database_path).await.unwrap();

        //ドラマ「笑ゥせぇるすまん」喪黒福造を演じるのは、秋山竜次(ロバート)！／7月18日(金)よりPrime Videoで独占配信
        let atmosfreq = AtmosFreq::from_youtube(
            &Url::parse("https://youtu.be/E0n8zwIdwFw?si=xImXFFfjmrIn-Kjs").unwrap(),
            &atmosdict,
        )
        .await;
        dbg!(atmosfreq);

        //【ドジャースがリーグ最速で50勝到達！山本無双ピッチで7勝目、マンシー満塁HR含む2安打6打点、コンフォート2戦連発！】ドジャースvsロッキーズ 試合ハイライト MLB2025シーズン 6.26
        let atmosfreq = AtmosFreq::from_youtube(
            &Url::parse("https://youtu.be/eA78BZt_alA?si=cSLWnG2sfXZq6t8x").unwrap(),
            &atmosdict,
        )
        .await;
        dbg!(atmosfreq);

        //『劇場版「無限城編」公開記念！「鬼滅の刃」全七夜特別放送』告知CM
        let atmosfreq = AtmosFreq::from_youtube(
            &Url::parse("https://youtu.be/zn4vWW3MDEw?si=OpJY_MjBMpyxRMD8").unwrap(),
            &atmosdict,
        )
        .await;
        dbg!(atmosfreq);

        //『イカゲーム』シーズン3 最終ゲーム 予告編 - Netflix
        let atmosfreq = AtmosFreq::from_youtube(
            &Url::parse("https://youtu.be/LTeOBlrHhhE?si=Fdumv-Hz588voZd1").unwrap(),
            &atmosdict,
        )
        .await;
        dbg!(atmosfreq);
    }
}
