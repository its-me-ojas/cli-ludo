use colored::*;

pub fn introduction() {
    let title = r#"
 _   _      _ _         ____  _                           
| | | | ___| | | ___   |  _ \| | __ _ _   _  ___ _ __ ___ 
| |_| |/ _ \ | |/ _ \  | |_) | |/ _` | | | |/ _ \ '__/ __|
|  _  |  __/ | | (_) | |  __/| | (_| | |_| |  __/ |  \__ \
|_| |_|\___|_|_|\___/  |_|   |_|\__,_|\__, |\___|_|  |___/
                                      |___/                   "#;

    println!("{}", title.bright_green().bold());
    println!("{}", "Welcome to Dicey Terminal Trails".bold().yellow());
}
