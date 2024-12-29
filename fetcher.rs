use crate::defaults::terminal_properties_init;
use crate::ifiend_structs;
use crate::ifiend_structs::*;
use colored;
use colored::*;
//
//
//Handles the meat of the program i.e getting data from YouTube and organising
//each video into a struct and collecting the resulting structs into a vector.
//
//
pub fn fetch(settings: &IfiendConfig) -> Vec<ifiend_structs::IfiendVideo> {
    let terminal = terminal_properties_init(&settings);
    let mut videos: Vec<IfiendVideo> = Vec::new();
    println!("{}", "\nStarting browser...");
    let browser = match headless_chrome::Browser::default() {
        Ok(value) => {
            println!("");
            value
        }
        Err(_) => {
            println!(
                "{}",
                "[ERROR] Something went wrong while starting the browser :(".red()
            );
            panic!()
        }
    };
    println!("{}", "Channels selected: \n");
    for channel in &settings.channels {
        print!("{} ", channel.handle.cyan());
        println!("")
    }
    println!("");
    let mut video_id_iterator = 0;
    for channel in settings.channels.clone() {
        let tab = browser
            .new_tab()
            .expect("[ERROR] Couldn't open new browser tab.");
        println!("Grabbing {}'s videos...", channel.handle.cyan());
        tab.navigate_to(format!("https://www.youtube.com/{}/videos", channel.handle).as_str())
            .expect("[ERROR] Something went wrong while trying to open a link in spawned browser.");
        let html_videos = tab
            .wait_for_elements("div#content.style-scope.ytd-rich-item-renderer")
            .expect("[ERROR] Something went wrong while waiting for elements on a YouTube channel page.");
        let mut i = 0;
        //let videos_per_channel = 3;
        for html_video in html_videos {
            i += 1;
            if i >= settings.videos_per_channel + 1 {
                break;
            }
            //Geting video ID and URL.
            //The returned value here looks like this /watch?v=8s7qSxjpLdA
            //Gibberish after the equal sign = is the video ID
            let watch_and_video_id = html_video
                .wait_for_element("a")
                .expect(format!("[{}] Couln't wait for video ID and URL element from YouTube.", "ERROR".red()).as_str())
                .get_attributes()
                .expect(format!("[{}] Couln't get video ID and URL attributes from YouTube.", "ERROR".red()).as_str())
                .expect(format!("[{}] Couln't get video ID and URL attributes from YouTube.", "ERROR".red()).as_str())
                .get(11)
                .expect(format!("[{}] Couln't extract video ID and URL form attributes procured from YouTube.", "ERROR".red()).as_str())
                .to_owned();
            //Concatenate the returned value to get a video url
            let url = format!("https://www.youtube.com{watch_and_video_id}");
            //Splitting the returned value to get just the video ID
            let id_parts = watch_and_video_id.split("=");
            let id_parts_collection = id_parts.collect::<Vec<&str>>();
            //Constructing a link to the thumbnail using the video ID
            let thumbnail = format!(
                "https://img.youtube.com/vi/{}/hqdefault.jpg",
                id_parts_collection[1]
            );
            //
            //Video duration in a verbose format (X hours, Y minutes, Z seconds.)
            //
            /*let duration = html_video
                .wait_for_element("yt-formatted-string")
                .unwrap()
                .get_attributes()
                .unwrap()
                .unwrap()
                .get(5)
                .unwrap()
                .to_owned();
            */
            //
            //Getting video duration
            //
            let duration = html_video
                .wait_for_element("badge-shape")
                .expect(
                    format!(
                        "[{}] Couln't wait for video duration element from YouTube.",
                        "ERROR".red()
                    )
                    .as_str(),
                )
                .get_inner_text()
                .expect(
                    format!(
                        "[{}] Couln't extract video duration from YouTube.",
                        "ERROR".red()
                    )
                    .as_str(),
                )
                .to_owned();
            //
            //Getting video title
            //
            let title = html_video
                .wait_for_element("h3")
                .expect(
                    format!(
                        "[{}] Couln't wait for video title from YouTube.",
                        "ERROR".red()
                    )
                    .as_str(),
                )
                .get_inner_text()
                .expect(
                    format!(
                        "[{}] Couln't extract video title from an element from YouTube.",
                        "ERROR".red()
                    )
                    .as_str(),
                )
                .to_owned();
            //
            //Getting video views and 'X hours ago'
            //
            let views_and_freshness = html_video
                .wait_for_element("ytd-video-meta-block")
                .expect(
                    format!(
                        "[{}] Couln't wait for video views and freshness from YouTube.",
                        "ERROR".red()
                    )
                    .as_str(),
                )
                .get_inner_text()
                .expect(
                    format!(
                        "[{}] Couln't extract video views and freshness from YouTube.",
                        "ERROR".red()
                    )
                    .as_str(),
                )
                .to_owned();
            //
            //Separating views form freshness
            //
            let vf_parts = views_and_freshness.split("\n");
            let vf_parts_collection = vf_parts.collect::<Vec<&str>>();

            let views = vf_parts_collection[0];
            let freshness = vf_parts_collection[1];
            let video = IfiendVideo {
                id: video_id_iterator,
                title: title.clone(),
                duration: duration.clone(),
                freshness: freshness.to_string(),
                views: views.to_string(),
                url: url.clone(),
                thumbnail: thumbnail.clone(),
            };

            //Prints data on a particular video that's just been fetched and determines how to go
            //about displaying the thumbnail

            fetch_info_printer(&channel, &video, &terminal, &settings);

            videos.push(video);
            video_id_iterator += 1;
        }
    }
    videos
}

fn fetch_info_printer(
    channel: &IfiendChannel,
    video: &IfiendVideo,
    terminal: &IfiendTerminal,
    settings: &IfiendConfig,
) {
    println!("");
    println!("[{}]", video.id.to_string().yellow());
    println!("{}", channel.handle.cyan());
    println!("[{}]", video.title.green());
    if terminal.supports_images {
        if terminal.is_kitty {
            let mut image = std::process::Command::new("kitten")
                    .args(["icat", "--align", "left", &video.thumbnail.clone()])
                    .spawn().expect("Couldn't spawn image. The program tries to spawn <kitten icat 'link to an image on the web'>. Check and see if it works on your system. Troubleshoot from there.");
            image.wait().expect("[ERROR] image.wait() failed.");
        }
        if terminal.supports_sixel {
            let thumbnail_img_path = format!(
                "{}/thumbnail_for_video_id_{}.jpg",
                settings.cache_path, video.id
            );
            let _ = std::process::Command::new("curl")
                .args(["--output", thumbnail_img_path.as_str(), &video.thumbnail])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect(format!("[{}] Couldn't launch '{}'. Maybe it's not installed. It is needed to download the thumbnails to display them in terminal window if you're not using kitty terminal.", "ERROR".red(), "curl".cyan()).as_str())
                .wait()
                .expect(format!("[{}] Couldn't launch '{}'. Maybe it's not installed. It is needed to download the thumbnails to display them in terminal window if you're not using kitty terminal.", "ERROR".red(), "curl".cyan()).as_str());
            if terminal.is_wezterm {
                std::process::Command::new("wezterm")
                    .args(["imgcat", &thumbnail_img_path])
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            } else {
                std::process::Command::new("img2sixel")
                    .arg(thumbnail_img_path)
                    .spawn()
                    .expect(format!("[{}] Couldn't launch '{}'. Maybe it's not installed. It is needed to download the thumbnails to display them in terminal window if you're not using kitty terminal or wezterm.", "ERROR".red(), "img2sixel".cyan()).as_str())
                    .wait()
                    .expect(format!("[{}] Couldn't launch '{}'. Maybe it's not installed. It is needed to download the thumbnails to display them in terminal window if you're not using kitty terminal or wezterm.", "ERROR".red(), "img2sixel".cyan()).as_str());
            }
        }
    }
    println!(
        "[{}] [{}] [{}]",
        video.freshness.yellow(),
        video.duration.yellow(),
        video.views.yellow()
    );
    //println!("[{duration}]");
    //println!("[{views}]");
    println!("[{}]", video.url);
    println!("[{}]\n", video.thumbnail);
}
