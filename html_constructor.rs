use colored::Colorize;

use crate::ifiend_structs::*;
use std::fs::File;
use std::io::{BufWriter, Write};

//
//
//Handles constructing html page with videos.
//
//
pub fn construct_html(settings: &IfiendConfig, fetched_videos: &Vec<IfiendVideo>) {
    println!(
        "\nGenerating {} file with thumbnails...",
        settings.html_path.cyan()
    );
    let html_filepath = settings.html_path.clone();
    let html_file = File::create(html_filepath.clone())
        .expect(format!("[ERROR] Couldn't create {} file.", html_filepath).as_str());
    let mut writer = BufWriter::new(html_file);
    writer
        .write_all(
            b"
<!DOCTYPE html>
<html lang='en'>
<head>
    <style>
    h1 {text-align: center;}
    h2 {text-align: center;}
    h3 {text-align: center;}
    img {text-align: center;}
    figure {text-align: center;}
    </style>
    <title>
        ifiend
    </title>
    <meta charset='utf-8'>
</head>

<body>
",
        )
        .expect(
            format!(
                "[{}] Couldn't write to '{}'.",
                "ERROR".red(),
                settings.html_path.cyan()
            )
            .as_str(),
        );
    for video in fetched_videos {
        writer
            .write_all(
                format!(
                    "
    <h2>[{}] {}</h2>
    <figure>
        <img src='{}' alt='idk'>
        <figcaption>
            {} | {} | {}
        </figcaption>
    </figure>
    <br>
    <br>
",
                    video.id,
                    video.title,
                    video.thumbnail,
                    video.views,
                    video.duration,
                    video.freshness,
                )
                .as_bytes(),
            )
            .expect(
                format!(
                    "[{}] Couldn't write to '{}'.",
                    "ERROR".red(),
                    settings.html_path.cyan()
                )
                .as_str(),
            );
    }
    writer
        .write_all(b"</body></html>")
        .expect(format!("[ERROR] Couldn't write to {} file.", settings.html_path).as_str());
    writer
        .flush()
        .expect(format!("[ERROR] Couldn't write to {} file.", settings.html_path).as_str());
}
