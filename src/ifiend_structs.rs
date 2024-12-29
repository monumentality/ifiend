use serde::{Deserialize, Serialize};
//
//
//Handles all the structs and methods
//
//
#[derive(Serialize, Deserialize, Clone)]
pub struct IfiendVideo {
    pub id: u32,
    pub url: String,
    pub thumbnail: String,
    pub title: String,
    pub duration: String,
    pub freshness: String,
    pub views: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct IfiendChannel {
    //name: String,
    pub handle: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct IfiendTerminal {
    pub supports_images: bool,
    pub supports_sixel: bool,
    pub is_kitty: bool,
    pub is_wezterm: bool,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct IfiendConfig {
    pub generate_html: bool,
    pub never_download: bool,
    pub always_yes_to_download: bool,
    pub never_play: bool,
    pub always_yes_to_play: bool,
    pub cleanup_html: bool,
    pub cache_path: String,
    pub config_path: String,
    pub html_path: String,
    pub youtube_downloader: String,
    pub video_player: String,
    pub force_sixel_image_support: bool,
    pub channels: Vec<IfiendChannel>,
    pub videos_per_channel: u32,
}

pub struct IfiendHtmlConstructorOutput {
    pub videos: Vec<IfiendVideo>,
    pub generated_filepath: String,
}
pub trait ToIfiendChannel {
    fn to_ifiend_channel(&self) -> IfiendChannel;
}
impl ToIfiendChannel for &str {
    fn to_ifiend_channel(&self) -> IfiendChannel {
        let string_as_channel = IfiendChannel {
            handle: self.to_string(),
        };
        string_as_channel
    }
}
impl ToIfiendChannel for String {
    fn to_ifiend_channel(&self) -> IfiendChannel {
        let string_as_channel = IfiendChannel {
            handle: self.to_string(),
        };
        string_as_channel
    }
}
