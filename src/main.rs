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
use ifiend_structs::*;
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
        .version("v0.2.0-alpha\nBy https://github.com/monumentality")
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
            format!("[{}] Clap error. Couldn't get vpc arg into a string","ERROR".red()).as_str(),).cyan(),
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
    let fetched_videos = fetch(&settings);
    if settings.generate_html {
        construct_html(&settings, &fetched_videos);
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
    let mut selected_urls = select_download_candidates(&fetched_videos);
    selected_urls = selected_urls.trim().to_string();
    if !settings.never_play && !selected_urls.is_empty() {
        println!(
            "\nStream videos with [{}]? (y/n):",
            settings.video_player.cyan()
        );
        if settings.always_yes_to_play || get_yes_or_no_input() {
            //std::thread::spawn(move || {
            let vec_of_urls: Vec<&str> = selected_urls.split_whitespace().collect();
            std::process::Command::new(&settings.video_player)
            .args(vec_of_urls)
            .spawn()
            .expect(format!("[{}] Couldn't pass the links to {} to play the videos.
                    Maybe {} is not installed? You can either install it, or change it to your video player of choice in {}.
                    The program will pass it URLs as arguments as a string with URLs separated by spaces.",
                    "ERROR".red(), settings.video_player.cyan(), settings.video_player.cyan(), settings.config_path.cyan()).as_str()).wait().expect(format!("[{}] {} errored out.", "ERROR".red(), settings.video_player.cyan()).as_str());
            //});
        }
    }
    if !settings.never_download && !selected_urls.is_empty() {
        println!("\nDownload these? (y/n):");
        if settings.always_yes_to_download || get_yes_or_no_input() {
            println!(
                "\nPassing URLs to [{}]...\n",
                settings.youtube_downloader.cyan()
            );
            let vec_of_urls: Vec<&str> = selected_urls.split_whitespace().collect();
            std::process::Command::new(&settings.youtube_downloader)
            .args(vec_of_urls)
            .spawn().expect(format!("[{}] Couldn't pass the links to {} to download the videos.
                    Maybe {} is not installed? You can either install it, or change it to your downloader of choice in {}.
                    The program will pass it URLs as arguments as a string with URLs separated by spaces.",
                    "ERROR".red(), settings.youtube_downloader.cyan(), settings.youtube_downloader.cyan(), settings.config_path.cyan()).as_str()).wait().expect(format!("[{}] {} errored out.", "ERROR".red(), settings.youtube_downloader.cyan()).as_str());
        }
    }

    println!("\nCleaning up...");
    for file in std::fs::read_dir(settings.cache_path).unwrap() {
        std::fs::remove_file(file.expect("Nothing to clean up.").path()).expect(
            format!(
                "[{}] Couldn't delete {}",
                "ERROR".red(),
                settings.html_path.cyan()
            )
            .as_str(),
        );
    }
    //}
}
//
//
//Asks for input and selects videos to download
//
//
fn select_download_candidates(videos: &Vec<IfiendVideo>) -> String {
    let mut output_urls: String = String::new();
    let mut called_it_quits: bool = false;

    //Starts the loop that allows user to try again if their arg wasn't valid.
    loop {
        //Clears output URLs in case some of the arguments pushed strings into it, but others were
        //incorrect, so that it doesn't persist into the next cycle
        output_urls = String::new();

        //Keeps count and then checks if all the provided args were valid
        let mut args_provided = 0;
        let mut args_validated = 0;

        //Asks for input
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        //Removes trailing spaces and newline
        let input_arguments = input.trim().split_whitespace();

        //Counts provided arguments and stores the result for later
        args_provided = input_arguments.clone().count();

        println!("");

        for argument in input_arguments {
            //Checks if an arg can be parsed as u32.
            match argument.parse::<u32>() {
                //If it can be, that means that it's probably a video index
                Ok(argument_as_u32) => {
                    //It is not yet known if a video with that index exist, so no by default.
                    let mut video_found = false;
                    //Run through video IDs to check if provided ID matches with anything
                    for video in videos {
                        //If ID matches
                        if argument_as_u32 == video.id {
                            //Add corresponding video url and a space at the end
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
                        break;
                    }
                }
                //If an arg cannot be turned into u32, then it probably means that user made a
                //typo, or wants to exit, or is attempting to pass a video title bit
                Err(_) => {
                    //Default to typo
                    let mut argument_is_valid: bool = false;
                    for video in videos {
                        //"a;" means that user doesn't want to select any of the videos
                        if argument.contains("a;") {
                            called_it_quits = true;
                            argument_is_valid = true;
                            //break the for loop and continue to exit
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
        //If not asked to exit by user
        if !called_it_quits {
            //Breaks the loop and returns output_urls if all args were valid
            if args_validated == args_provided {
                break;
            }
            //Starts new cycle if some args were invalid
            continue;
        }
        break;
    }
    return output_urls;
}
