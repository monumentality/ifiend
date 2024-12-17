use clap::{Arg, ArgAction, ArgMatches, Command};
use defaults::{get_yes_or_no_input, load_defaults};
use fetcher::fetch;
use html_constructor::construct_html;
pub mod defaults;
pub mod fetcher;
pub mod html_constructor;
pub mod ifiend_structs;
use colored;
use colored::*;
use ifiend_structs::{IfiendConfig, IfiendVideo, ToIfiendChannel};
//
//
//Handles da flow and also arguments
//
//
fn main() {
    //Loads config into non-persistent settings
    //Also creates config if non-existent
    let mut settings = load_defaults();

    //Defines different commands and args
    let matches = Command::new("ifiend")
        .about("\nCheck YouTube without leaving the comfort of your terminal.")
        .version("v0.1.1-alpha.\nBy https://github.com/monumentality")
        .subcommand_required(true)
        .arg_required_else_help(true)
        /*
        .subcommand(
            Command::new("set-default")
                .short_flag('s')
                //.long_flag("set-default")
                .about("set default parameters")
                .arg(
                    Arg::new("channels")
                        .short('c')
                        .long("channels")
                        .conflicts_with("video-count")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .help("sets the channels that you'll get videos from"),
                )
                .arg(
                    Arg::new("vpc")
                        .short('v')
                        .long("videos-per-channel")
                        .conflicts_with("channels")
                        .action(ArgAction::SetTrue)
                        .num_args(1)
                        .help("sets the default number of videos to get per channel"),
                ),
        )*/
        .subcommand(
            Command::new("fetch")
                .about(format!("Fetch and present videos.\n\nUsage:\n{}\n\n[{}]\nVideos-per-channel to fetch. Comes first. Uses default if not specified.\nShould always be specified if using with [channel] argument.\n\n[{}]\nChannels to fetch videos from. Can use multiple. Requires specifying vpc. Comes after [vpc].\nCan search config for matches or use provided channel directly if it's not in config.\nUses default if not specified.\n", "ifiend fetch [vpc] [channel]".cyan(),"vpc".yellow(), "channel".yellow()))
                .arg(
                    Arg::new("vpc")
                        .action(ArgAction::Set)
                        .num_args(1),
                )
                .arg(
                    Arg::new("channel")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .get_matches();

    //Defines behavior when passing different args
    match matches.subcommand() {
        Some(("fetch", fetch_matches)) => {
            println!("");
            if fetch_matches.contains_id("channel") {
                let provided_channels = fetch_matches.get_many::<String>("channel").expect(
                    format!(
                        "[{}] Clap error. Couldn't get channel args into a string",
                        "ERROR".red()
                    )
                    .as_str(),
                );
                let config_channels = settings.channels.clone();
                let mut args_aborted = 0;
                settings.channels = Vec::new();
                for argument in provided_channels {
                    let mut is_in_config = false;
                    for config_channel in &config_channels {
                        if config_channel.handle.contains(argument) {
                            println!(
                                "Parsing '{}' as [{}]",
                                argument.cyan(),
                                config_channel.handle.cyan()
                            );
                            is_in_config = true;
                            settings.channels.push(config_channel.clone());
                        }
                    }
                    if !is_in_config {
                        println!("Provided channel '{}' does not seem to be in config. Use anyway? (y/n): ", argument.cyan());
                        if get_yes_or_no_input() {
                            settings.channels.push(argument.to_ifiend_channel());
                        } else {
                            args_aborted += 1;
                        }
                    }
                }
                println!("");
                //Exits if you have refused to use all the custom channels provided.
                //The 3 is ifiend + fetch + [vpc]
                if args_aborted == std::env::args().count() - 3 {
                    std::process::exit(0);
                }
            }
            if fetch_matches.contains_id("vpc") {
                //Sets settings according to user provided vpc
                //(without writing it to default)
                settings = parse_vpc_and_assign_it(settings, &fetch_matches);
            }
            //Main program
            start_doing_the_deed_finally(settings);
        }
        _ => unreachable!(),
    }
}
//
//
//Parses provided vpc (videos-per-channel) as u32 and reassigns current settings
//
//
fn parse_vpc_and_assign_it(
    mut settings: IfiendConfig,
    provided_non_parsed_vpc: &ArgMatches,
) -> IfiendConfig {
    settings.videos_per_channel = match provided_non_parsed_vpc
        .get_one::<String>("vpc")
        .expect(
            format!(
                "[{}] Clap error. Couldn't get vpc arg into a string",
                "ERROR".red()
            )
            .as_str(),
        )
        .parse::<u32>()
    {
        Ok(vpc_u32) => vpc_u32,
        //Use default if provided argument is gibberish and doesn't parse as u32
        Err(_) => {
            println!
                            (
                                "\n[ERROR] '{}' is not a valid number of videos to fetch.\nVideos-per-channel must be a positive number. Using default: [{}]",
                            provided_non_parsed_vpc.get_one::<String>("vpc").expect(
            format!(
                "[{}] Clap error. Couldn't get vpc arg into a string",
                "ERROR".red()
            )
            .as_str(),
        )
.cyan(),
                            settings.videos_per_channel.to_string().cyan()
                            );
            //Returns default
            settings.videos_per_channel
        }
    };
    settings
}
//
//
//Da meat
//Invokes fetch(), html_constructor(), select_download_candidates() etc.
//
fn start_doing_the_deed_finally(settings: IfiendConfig) {
    println!(
        "[{}] set to [{}].",
        "videos_per_channel".yellow(),
        settings.videos_per_channel.to_string().cyan()
    );
    let fetched_videos;
    if settings.generate_html {
        fetched_videos = construct_html(settings.clone(), fetch(settings.clone()));
        println!("Open the generated html file? (y/n):");

        if get_yes_or_no_input() {
            let html_path = settings.html_path.clone();
            std::thread::spawn(move || {
                open::that(html_path.clone()).expect(
                    format!(
                        "[{}] Couldn't open [{}] with default program.",
                        "ERROR".red(),
                        html_path
                    )
                    .as_str(),
                );
            });
        }
    } else {
        fetched_videos = fetch(settings.clone());
    }

    println!("");
    println!("Please select and enter the videos you'd like to download (case-sensitive).");
    println!(
        "{}\n{}\n{}",
        "You can combine video IDs and words from titles on the same line (one word per title):",
        "[EXAMPLE]".green(),
        "1 5 Guitar 3 cookies GAMEPLAY 7".cyan()
    );
    println!("'{}' to abort.", "a;".cyan());
    let urls_to_download = select_download_candidates(fetched_videos);
    println!("urls gotten: {}", urls_to_download);
    if !urls_to_download.is_empty() {
        std::process::Command::new(&settings.youtube_downloader)
            .arg(urls_to_download)
            .spawn().expect(format!("[{}] Couldn't pass the links to {} to download the videos.
                    Maybe {} is not installed? You can either install it, or change it to your downloader of choice in {}.
                    The program will pass it URLs as arguments as a string with URLs separated by spaces.",
                    "ERROR".red(), settings.youtube_downloader.cyan(), settings.youtube_downloader.cyan(), "config.toml".cyan()).as_str());
    }
    if settings.generate_html && settings.cleanup_html {
        println!("\nCleaning up...");
        std::fs::remove_file(&settings.html_path).expect(
            format!(
                "[{}] Couldn't delete {}",
                "ERROR".red(),
                settings.html_path.cyan()
            )
            .as_str(),
        );
    }
}
//
//
//Asks for input and selects videos to download
//
//
fn select_download_candidates(videos: Vec<IfiendVideo>) -> String {
    let mut output_selected_int: Vec<u32> = Vec::new();
    let mut output_urls: String = String::new();
    let mut called_it_quits: bool = false;
    loop {
        output_urls = String::new();
        let mut args_provided = 0;
        let mut args_validated = 0;
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let input_arguments = input.trim().split_whitespace();
        args_provided = input_arguments.clone().count();
        println!("");
        for argument in input_arguments {
            match argument.parse::<u32>() {
                Ok(argument_as_u32) => {
                    let mut video_found = false;
                    output_selected_int.push(argument_as_u32);
                    for video in &videos {
                        if argument_as_u32 == video.id {
                            output_urls.push_str(format!("{} ", video.url).as_str());
                            println!(
                                "Parsing '{}' as [{}]...",
                                argument.cyan(),
                                video.title.green()
                            );
                            args_validated += 1;
                            video_found = true;
                        }
                    }
                    if !video_found {
                        println!("Video indexed '{}' does not exist.", argument.cyan());
                        select_download_candidates(videos.clone());
                    }
                }
                Err(_) => {
                    let mut argument_is_valid: bool = false;
                    for video in &videos {
                        if argument.contains("a;") {
                            called_it_quits = true;
                            argument_is_valid = true;
                            break;
                        } else if video.title.contains(argument) {
                            output_urls.push_str(format!("{} ", video.url).as_str());
                            println!(
                                "Parsing [{}] as [{}]...",
                                argument.cyan(),
                                video.title.green()
                            );
                            args_validated += 1;
                            argument_is_valid = true;
                        }
                    }
                    if !argument_is_valid {
                        println!(
                            "'{}' {}",
                            argument.cyan(),
                            "is not a valid video ID, nor is it found in video titles. Try again."
                        );
                    }
                }
            }
        }
        if !called_it_quits {
            if args_validated == args_provided {
                break;
            }
            continue;
        }
        break;
    }
    return output_urls;
}
