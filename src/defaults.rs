use crate::ifiend_structs::*;
use colored;
use colored::*;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use toml;
//
//
//Handles creating and loading default configuration
//
//
pub fn get_yes_or_no_input() -> bool {
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        match input.as_str().trim() {
            "y" | "Y" | "YES" | "yes" => return true,
            _ => return false,
        }
    }
}

pub fn debug_channels_generator() -> Vec<IfiendChannel> {
    //Generates some channels for default config
    println!("");
    let mut channels: Vec<IfiendChannel> = Vec::new();
    let oneyplays = IfiendChannel {
        handle: "@OneyPlays".to_string(),
    };
    let jacobgeller = IfiendChannel {
        handle: "@JacobGeller".to_string(),
    };
    let emplemon = IfiendChannel {
        handle: "@EmperorLemon".to_string(),
    };
    let solarsands = IfiendChannel {
        handle: "@SolarSands".to_string(),
    };
    let hbomberguy = IfiendChannel {
        handle: "@hbomberguy".to_string(),
    };
    channels.push(oneyplays);
    channels.push(jacobgeller);
    channels.push(emplemon);
    channels.push(solarsands);
    channels.push(hbomberguy);
    channels
}

pub fn set_defaults() {
    let default_videos_per_channel = 3;
    let default_generate_html = true;
    let default_cleanup_html = true;
    let default_cache_path = format!(
        "{}/{}",
        dirs::cache_dir()
            .expect(format!("[{}] Couldn't locate the cache directory.", "ERROR".red()).as_str())
            .to_string_lossy(),
        "ifiend"
    );
    let default_config_path = format!(
        "{}/{}",
        dirs::config_dir()
            .expect(format!("[{}] Couldn't locate the config directory.", "ERROR".red()).as_str())
            .to_string_lossy(),
        "ifiend"
    );
    let default_html_path = format!("{}/{}", default_cache_path, "output.html");
    let default_channels = debug_channels_generator();

    let config = IfiendConfig {
        youtube_downloader: "yt-dlp".to_string(),
        generate_html: default_generate_html,
        cleanup_html: default_cleanup_html,
        cache_path: default_cache_path,
        config_path: default_config_path.clone(),
        html_path: default_html_path,
        channels: default_channels,
        videos_per_channel: default_videos_per_channel,
    };
    println!(
        "[{}] set to [{}].",
        "generate_html".yellow(),
        config.generate_html.to_string().green()
    );
    println!(
        "[{}] set to [{}].",
        "cleanup_html".yellow(),
        config.cleanup_html.to_string().green()
    );
    println!(
        "[{}] set to [{}].",
        "cache_path".yellow(),
        config.cache_path.to_string().green()
    );
    println!(
        "[{}] set to [{}].",
        "config_path".yellow(),
        config.config_path.to_string().green()
    );
    println!(
        "[{}] set to [{}].",
        "html_path".yellow(),
        config.html_path.to_string().green()
    );
    println!(
        "[{}] set to [{}].",
        "youtube_downloader".yellow(),
        config.youtube_downloader.to_string().green()
    );
    let toml =
        toml::to_string(&config).expect("[ERROR] Couldn't parse config struct contents as TOML.");
    fs::create_dir_all(format!(
        "{}/ifiend",
        dirs::config_dir()
            .expect(format!("[{}] Couldn't locate the config directory.", "ERROR".red()).as_str())
            .to_string_lossy()
    ))
    .expect(
        format!(
            "[{}] Couldn't create '{}' directory in the config directory.",
            "ERROR".red(),
            "ifiend".cyan()
        )
        .as_str(),
    );
    let toml_file = File::create(format!("{}/{}", default_config_path, "config.toml"))
        .expect("[ERROR] Couldn't create config.toml");
    let mut writer = BufWriter::new(toml_file);
    writer
        .write_all(toml.as_bytes())
        .expect("[ERROR] Couldn't write to config.toml");
    writer
        .flush()
        .expect("[ERROR] Couldn't write to config.toml");
}

pub fn load_defaults() -> IfiendConfig {
    let config_path = format!(
        "{}/{}/{}",
        dirs::config_dir()
            .expect(format!("[{}] Couldn't locate the config directory.", "ERROR".red()).as_str())
            .to_string_lossy(),
        "ifiend",
        "config.toml"
    );
    let config_file_contents = match fs::read_to_string(config_path.clone()) {
        Ok(config) => config,
        Err(_) => {
            print!(
                "\n{} [{}] {}",
                "Config file".yellow(),
                config_path.cyan(),
                "does not seem to exist.\n".yellow()
            );
            println!("{}", "Would you like to generate it? (y/n): ");
            if get_yes_or_no_input() {
                set_defaults();
                return load_defaults();
            } else {
                println!("{}", "Quitting...");
                std::process::exit(0);
            }
        }
    };
    let config: IfiendConfig = match toml::from_str(&config_file_contents) {
        Ok(config) => config,
        Err(_) => {
            print!("\n{} [{}] {}","Config file".yellow(),config_path.cyan(), "exists, but ifiend couldn't understand it. You may have made a typo while editing it.\n".yellow());
            println!("{}", "Would you like to generate a new one? (y/n): ");
            if get_yes_or_no_input() {
                set_defaults();
                return load_defaults();
            } else {
                println!("{}", "Quitting...");
                std::process::exit(0);
            }
        }
    };
    return config;
}
