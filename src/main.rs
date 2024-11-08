use colored::Colorize;
use dialoguer::{Input, Select};
use search::menu;
mod clean;
mod fetch;
mod search;
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
#[tokio::main]
async fn main() {

    // Main menu
    let menu_items = vec![
        "Default registry (docker.io)".to_string(),
        "Specify custom registry url".to_string(),
    ];

    // Continously prompt for docker image and registry
    loop {
        println!("{}", ASCII_ART.blue().bold().italic());
        let docker_image: String = Input::new()
        .with_prompt("Enter docker image tag (e to exit)")
        .interact_text()
        .unwrap();

        // Exit if user types 'e'
        if docker_image == "e" {
            break;
        }

        // Select registry
        let selection = Select::new()
            .with_prompt("Select container registry")
            .items(&menu_items)
            .default(0)
            .interact()
            .unwrap();

        let registry=match selection {
            0 => {
                "docker.io".to_string()
            }
            1 => {
                let registry: String = Input::new()
                    .with_prompt("Enter custom registry url")
                    .interact_text()
                    .unwrap();
                registry
            }
            _ => unreachable!(),
        };

        // Fetch the docker image and extract it
        match fetch::fetch_image(&docker_image, &registry).await {
            Ok(temp_dir) => {
                println!("Image fetched and extracted to {}", temp_dir);
                // Start interactive search
                if let Err(e) = menu::interactive_search(temp_dir.into()).await {
                    println!("Error during interactive search: {}", e);
                }
            }
            Err(e) => println!("Error fetching image: {}", e),
        }

    }

}
