use clap::{Arg, Command};
use colored::*;

use crate::{fetch::fetch_image, search::menu::interactive_search};

static ASCII_ART: &str = r"
         .--.           
        /    \ .--.  
        |  O   /    \ 
        .--'    '-----'
        |  .-~~-.     |
        | /      \    |
        ||       |    | Dockerdump.
        | \      /    |
        |  `---'      |
        '-----------'
        A whale in a bucket.
        @https://github.com/saefstroem/dockerdump
        ";

static DEFAULT_REGISTRY: &str = "docker.io";

pub async fn extract_args() {
    let matches = Command::new("dockerdump")
        .long_about(ASCII_ART)
        .about(ASCII_ART)
        .subcommand(
            Command::new("extract")
                .about("extracts files from a docker image. Specify custom registry with -r=<registry_url>")
                .arg(
                    Arg::new("image")
                        .required(true)
                        .help("The docker image to extract from"),
                )
                .arg(
                    Arg::new("registry")
                        .short('r')
                        .long("registry")
                        .help("The registry to pull the image from"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("extract", sub_matches)) => {
            let image = sub_matches.get_one::<String>("image").unwrap();
            let registry = sub_matches
                .get_one::<String>("registry")
                .map_or(DEFAULT_REGISTRY, |v| v);
            println!("{}", ASCII_ART.blue().bold().italic());
            match fetch_image(image, registry).await {
                Ok(temp_dir) => {
                    println!("Image fetched and extracted to {}", temp_dir.blue().bold());
                    // Start interactive search
                    if let Err(e) = interactive_search(temp_dir.into()).await {
                        println!("Error during interactive search: {}", e);
                    }
                }
                Err(e) => println!("Error fetching image: {}", e),
            }
        }
        _ => println!("dockerdump: command not found, use -h for help"),
    }
}
