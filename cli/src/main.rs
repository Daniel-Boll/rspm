// This code aims to create a password manager and generator for the user.

// USAGE:
// ./spm login
// ./spm configure --file <path>
// ./spm configure --migrate <path> [TODO]
// ./spm add <name> --password <password> --email <email> --url <url> --notes <notes>
// ./spm list
// ./spm delete <name>

// On add command email, url and notes are optional.

use argh::FromArgs;
use manager::{
    add::{add_password, generate_password},
    configure::define_file_location,
    get::get_password,
    list::list_passwords,
    login::login,
};

fn default_file() -> String {
    format!(
        "{}/.local/share/spm/passwords.spm",
        std::env::var("HOME").unwrap()
    )
}

#[derive(FromArgs, Debug)]
/// SPM::<Simple Password Manager> - Password manager and generator written in Rust .
struct SPMParser {
    #[argh(subcommand)]
    subcommand: SubCommand,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum SubCommand {
    Add(SubCommandAdd),
    Configure(SubCommandConfigure),
    Login(SubCommandLogin),
    List(SubCommandList),
    Get(SubCommandGet),
    // Delete(SubCommandDelete),
}

#[derive(FromArgs, Debug)]
/// Login into an account.   
#[argh(subcommand, name = "login")]
pub struct SubCommandLogin {
    #[argh(option, short = 'e', long = "email")]
    /// email of the account. 
    email: Option<String>,

    #[argh(option, short = 'p', long = "password")]
    /// password of the account.   
    password: Option<String>,
}

#[derive(FromArgs, Debug)]
/// Configure the password manager.   
#[argh(subcommand, name = "configure")]
struct SubCommandConfigure {
    #[argh(option, short = 'f', long = "file", default = "default_file()")]
    /// file to store the passwords.   (default at $HOME/.local/share/spm/passwords.spm)
    file: String,
}

#[derive(FromArgs, Debug)]
/// Add a new password to the password manager.   
#[argh(subcommand, name = "add")]
struct SubCommandAdd {
    #[argh(positional)]
    /// name of the password.   
    name: String,

    #[argh(option, short = 'p', long = "password")]
    /// password to be added.   
    password: Option<String>,

    #[argh(option, short = 'g', long = "generate-password")]
    /// generate a strong password. 瑱
    generate_password: Option<u32>,

    #[argh(option, short = 'e', long = "email")]
    /// email of the password. 
    email: Option<String>,

    #[argh(option, short = 'u', long = "url")]
    /// url of the password. 
    url: Option<String>,

    #[argh(option, short = 'n', long = "notes")]
    /// notes of the password. פֿ
    notes: Option<String>,
}

#[derive(FromArgs, Debug)]
/// List all passwords names.   
#[argh(subcommand, name = "list")]
pub struct SubCommandList {}

#[derive(FromArgs, Debug)]
/// Get a specific password. 什  
#[argh(subcommand, name = "get")]
pub struct SubCommandGet {
    #[argh(positional)]
    /// name of the password.   
    name: String,
}

fn main() {
    let args: SPMParser = argh::from_env::<SPMParser>();

    match args.subcommand {
        #[allow(unused_must_use)]
        SubCommand::Login(subcommand) => {
            login(subcommand.email, subcommand.password);
        }
        SubCommand::List(_) => {
            list_passwords();
        }
        SubCommand::Configure(subcommand) => {
            // TODO: to call configure it must confirm credentials exists
            // credentials_valid();
            define_file_location(subcommand.file);
        }
        SubCommand::Add(subcommand) => {
            // credentials_valid();
            let password = match subcommand.password {
                Some(password) => password,
                None => generate_password(subcommand.generate_password.unwrap_or(8)),
            };

            add_password(
                subcommand.name,
                Some(password),
                subcommand.email,
                subcommand.url,
                subcommand.notes,
            );
        }
        SubCommand::Get(subcommand) => {
            // credentials_valid();
            get_password(subcommand.name);
        }
    }
}
