use colored::*;

pub fn introduction() {
    let title = r#"
  ____  _                 _                     
 |  _ \| |               | |                    
 | |_) | |_   _ _ __ ___ | |__   __ _ ___  ___ 
 |  _ <| | | | | '_ ` _ \| '_ \ / _` / __|/ _ \
 | |_) | | |_| | | | | | | |_) | (_| \__ \  __/
 |____/|_|\__, |_| |_| |_|_.__/ \__,_|___/\___|
          __/ |                                
         |___/                                 
    "#;

    println!("{}", title.cyan());
    println!("{}", "Welcome to Dicey Terminal Trails".bold().yellow());
}
