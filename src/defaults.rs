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
    let cache_path = format!(
        "{}/{}",
        dirs::cache_dir()
            .expect(format!("[{}] Couldn't locate the cache directory.", "ERROR".red()).as_str())
            .to_string_lossy(),
        "ifiend"
    );
    let config_path = format!(
        "{}/{}",
        dirs::config_dir()
            .expect(format!("[{}] Couldn't locate the config directory.", "ERROR".red()).as_str())
            .to_string_lossy(),
        "ifiend"
    );
    let html_path = format!("{}/{}", cache_path, "output.html");

    let config = IfiendConfig {
        youtube_downloader: "yt-dlp".to_string(),
        video_player: "mpv".to_string(),
        generate_html: false,
        never_download: false,
        always_yes_to_download: false,
        never_play: false,
        always_yes_to_play: false,
        cleanup_html: true,
        cache_path: cache_path.clone(),
        config_path: config_path.clone(),
        html_path,
        videos_per_channel: 3,
        force_sixel_image_support: false,
        channels: debug_channels_generator(),
    };
    println!(
        "[{}] set to [{}].",
        "generate_html".yellow(),
        config.generate_html.to_string().green()
    );
    println!(
        "[{}] set to [{}].",
        "never_download".yellow(),
        config.never_download.to_string().green()
    );
    println!(
        "[{}] set to [{}].",
        "always_yes_to_download".yellow(),
        config.always_yes_to_download.to_string().green()
    );

    println!(
        "[{}] set to [{}].",
        "never_play".yellow(),
        config.never_play.to_string().green()
    );
    println!(
        "[{}] set to [{}].",
        "always_yes_to_play".yellow(),
        config.always_yes_to_play.to_string().green()
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
        "video_player".yellow(),
        config.video_player.to_string().green()
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
    let toml_file = File::create(format!("{}/{}", config.config_path, "config.toml"))
        .expect("[ERROR] Couldn't create config.toml");
    let mut writer = BufWriter::new(toml_file);
    writer
        .write_all(toml.as_bytes())
        .expect("[ERROR] Couldn't write to config.toml");
    writer
        .flush()
        .expect("[ERROR] Couldn't write to config.toml");
    std::fs::create_dir_all(config.cache_path).expect(
        format!(
            "[{}] Couldn't create '{}' directory in the cache directory",
            "ERROR".red(),
            "ifiend".cyan()
        )
        .as_str(),
    );
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
pub fn terminal_properties_init(settings: &IfiendConfig) -> IfiendTerminal {
    let ifiend_parent_pids = get_parent_pid(std::process::id());
    let terminal_process_name = get_pid_name(ifiend_parent_pids[1]);
    let mut terminal = IfiendTerminal {
        supports_images: false,
        supports_sixel: false,
        is_kitty: false,
        is_wezterm: false,
    };
    if terminal_process_name.contains("kitty") {
        terminal.supports_images = true;
        terminal.supports_sixel = false;
        terminal.is_kitty = true;
    }
    if terminal_process_name.contains("wezterm") {
        terminal.supports_images = true;
        terminal.supports_sixel = true;
        terminal.is_wezterm = true;
    }
    let terminals_that_support_sixel: Vec<&str> = [
        "konsole",
        "bobcat",
        "contour",
        "ctx",
        "darktile",
        "domterm",
        "eat",
        "foot",
        "iterm2",
        "laterminal",
        "macterm",
        "mintty",
        "mlterm",
        "rlogin",
        "sixel-tmux",
        "st",
        "swiftterm",
        "syncterm",
        "tmux",
        "toyterm",
        "u++",
        "xfce-terminal",
        "xterm",
        "xterm.js",
        "yaft",
        "yakuake",
        "zallij",
    ]
    .to_vec();

    for name in terminals_that_support_sixel {
        if terminal_process_name.contains(name) {
            terminal.supports_images = true;
            terminal.supports_sixel = true;
        }
    }

    if settings.force_sixel_image_support {
        terminal.supports_images = true;
        terminal.supports_sixel = true;
    }
    terminal
}
pub fn get_parent_pid(pid: u32) -> Vec<u32> {
    let mut pids: Vec<u32> = Vec::new();
    // ps -o ppid=66393
    let ret = std::process::Command::new("ps")
        .arg("-o")
        .arg(format!("ppid={}", pid))
        .output();

    if ret.is_err() {
        return pids;
    }
    let output = String::from_utf8_lossy(&ret.unwrap().stdout).to_string();
    for pid in output.split_whitespace() {
        match pid.parse::<u32>() {
            Ok(p) => {
                pids.push(p);
                //println!("{}", p)
            }
            Err(_) => break,
        }
    }
    pids
}

pub fn get_pid_name(pid: u32) -> String {
    // ps -p 66393 -o comm=
    let ret = std::process::Command::new("ps")
        .arg("-p")
        .arg(format!("{}", pid))
        .arg("-o")
        .arg("comm=")
        .output();

    let name = String::from_utf8_lossy(&ret.unwrap().stdout).to_string();
    //println!("terminal: {:?}", name);
    name
}

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
