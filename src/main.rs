use anyhow::{Context, Result};
use clap::{Arg, Command, command};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command as ProcessCommand;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Profile {
    username: String,
    email: String,
    access_token: String,
}

type Profiles = HashMap<String, Profile>;

const CONFIG_DIR: &str = ".config/git-thing";
const CONFIG_FILE: &str = "config.json";

fn get_config_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    let config_dir = home.join(CONFIG_DIR);
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
    }
    Ok(config_dir.join(CONFIG_FILE))
}

fn load_profiles() -> Result<Profiles> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        return Ok(HashMap::new());
    }
    
    let content = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file: {:?}", config_path))?;
    
    serde_json::from_str(&content).context("Failed to parse config file")
}

fn save_profiles(profiles: &Profiles) -> Result<()> {
    let config_path = get_config_path()?;
    let content = serde_json::to_string_pretty(profiles)
        .context("Failed to serialize profiles")?;
    
    fs::write(&config_path, content)
        .with_context(|| format!("Failed to write to config file: {:?}", config_path))
}

fn add_profile(name: &str, username: &str, email: &str, access_token: &str) -> Result<()> {
    let mut profiles = load_profiles()?;
    
    let profile = Profile {
        username: username.to_string(),
        email: email.to_string(),
        access_token: access_token.to_string(),
    };
    
    profiles.insert(name.to_string(), profile);
    save_profiles(&profiles)?;
    
    println!("Profile '{}' added successfully!", name);
    Ok(())
}

fn list_profiles() -> Result<()> {
    let profiles = load_profiles()?;
    
    if profiles.is_empty() {
        println!("No profiles found.");
        return Ok(());
    }
    
    println!("Available profiles:");
    for (name, profile) in &profiles {
        println!("\nProfile: {}", name);
        println!("  Username: {}", profile.username);
        println!("  Email: {}", profile.email);
    }
    
    Ok(())
}

fn switch_profile(profile_name: &str) -> Result<()> {
    let profiles = load_profiles()?;
    let profile = profiles.get(profile_name)
        .with_context(|| format!("Profile '{}' not found", profile_name))?;
    
    // Update git config
    ProcessCommand::new("git")
        .args(["config", "--global", "user.name", &profile.username])
        .status()
        .context("Failed to set git user name")?;
    
    ProcessCommand::new("git")
        .args(["config", "--global", "user.email", &profile.email])
        .status()
        .context("Failed to set git user email")?;
    
    // Update git credentials
    let credentials_path = dirs::home_dir()
        .context("Could not find home directory")?
        .join(".git-credentials");
    
    let credentials = format!("https://{}:{}@github.com", profile.username, profile.access_token);
    let mut file = File::create(&credentials_path)
        .with_context(|| format!("Failed to create credentials file: {:?}", credentials_path))?;
    
    writeln!(file, "{}", credentials)
        .with_context(|| format!("Failed to write to credentials file: {:?}", credentials_path))?;
    
    println!("Switched to profile '{}' successfully!", profile_name);
    Ok(())
}

fn main() -> Result<()> {
    let matches = command!()
        .subcommands([
            Command::new("add")
                .about("Adds a new git profile.")
                .args([
                    Arg::new("profilename")
                        .long("profilename")
                        .short('p')
                        .help("What you want to call this profile e.g. work.")
                        .required(true),
                    Arg::new("username")
                        .long("username")
                        .short('u')
                        .help("Your github username.")
                        .required(true),
                    Arg::new("email")
                        .long("email")
                        .short('e')
                        .help("Your github email.")
                        .required(true),
                    Arg::new("personal_access_key")
                        .long("personal-access-key")
                        .short('k')
                        .help("your github personal access key")
                        .required(true),
                ]),
            Command::new("list")
                .about("Lists all git profiles."),
            Command::new("switch")
                .about("Switch to another git profile")
                .arg(
                    Arg::new("profile")
                        .long("profile")
                        .short('p')
                        .help("The name of the profile to switch to.")
                        .required(true)
                ),
        ])
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let profile_name = sub_matches.get_one::<String>("profilename").unwrap();
            let username = sub_matches.get_one::<String>("username").unwrap();
            let email = sub_matches.get_one::<String>("email").unwrap();
            let access_token = sub_matches.get_one::<String>("personal_access_key").unwrap();
            
            add_profile(profile_name, username, email, access_token)
        },
        Some(("list", _)) => list_profiles(),
        Some(("switch", sub_matches)) => {
            let profile_name = sub_matches.get_one::<String>("profile").unwrap();
            switch_profile(profile_name)
        },
        _ => {
            println!("No valid command provided. Use --help for usage information.");
            Ok(())
        }
    }
}
