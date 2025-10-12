use clap::{Arg, Command, command};

fn main() {
    let matches = command!()
        .subcommands([
            Command::new("add").about("Adds a new git profile.").args([
                Arg::new("profilename").help("What you want to call this profile e.g. work."),
                Arg::new("username").help("Your github username."),
                Arg::new("email").help("Your github email."),
                Arg::new("personal access key").help("your github personal access key"),
            ]),
            Command::new("list").about("Lists all git profiles."),
            Command::new("update").about("Overwrites a git profile"),
            Command::new("delete").about("Delete a git profile"),
            Command::new("switch")
                .about("Switch to another git profile")
                .arg(Arg::new("targetprofile").help("The name of the profile to switch to.")),
        ])
        .get_matches();

    if let Some(name) = matches.subcommand_matches("add") {
        println!("matches: {:?}", name);

        if let Some(profile_name) = name.get_one::<String>("profilename") {
            println!("profile name: {:?}", profile_name);
        }

        if let Some(username) = name.get_one::<String>("username") {
            println!("profile name: {:?}", username);
        }

        if let Some(email) = name.get_one::<String>("email") {
            println!("profile name: {:?}", email);
        }

        if let Some(access_key) = name.get_one::<String>("personal access key") {
            println!("profile name: {:?}", access_key);
        }
    }
}
