mod context;
mod login;
mod logout;
mod sphere;

use clap::{Args, Parser, Subcommand};
pub use context::CommandContext;
use login::login;
use logout::logout;
use openapi::apis::configuration::Configuration;

use cred_store::CredStore;
use sphere::{get_metasphere_outputs, SphereOutputArgs};

use crate::auth::get_token::get_token;

use self::sphere::{
    create_sphere, delete_sphere, list_sphere, SphereCreateArgs, SphereDeleteArgs, SphereListArgs,
};

#[derive(Parser)]
#[clap(author, version, about = "Metagravity's Platform cli")]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Login to Metagravity's platform using Auth0
    Login,
    /// Remove Metagravity's platform stored credential
    Logout,
    /// Create, Delete, List Platform Metaspheres
    Sphere(SphereArgs),
    /// Create, Delete, List Platform Projects (Unimplemented)
    Project,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct SphereArgs {
    #[command(subcommand)]
    command: SphereCommands,
}

#[derive(Debug, Subcommand)]
enum SphereCommands {
    /// Create a new Metasphere
    Create(SphereCreateArgs),
    /// Delete a Metasphere using its id
    Delete(SphereDeleteArgs),
    /// List all Metaspheres within a project
    List(SphereListArgs),
    /// Retrieve Metasphere status given its Id
    Status,
    /// Retrieve Metasphere Output variables
    Output(SphereOutputArgs),
}

pub async fn parse_command<'a, T: CredStore>(context: &mut CommandContext<'a, T>) {
    let cli = Cli::parse();

    match cli.command {
        Command::Login => login(context).await,
        Command::Logout => logout(context),
        _ => {}
    }

    let access_token = match get_token(context) {
        Ok(token) => match token {
            Some(token) => token,
            None => {
                eprintln!("You must login first.");
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Couldn't get credentials: {}.  Try to login again.", e);
            std::process::exit(1);
        }
    };

    let mut configuration = Configuration::new();

    configuration.bearer_access_token = Some(access_token);
    configuration.base_path = context.config.binarite_url.to_string();

    match cli.command {
        Command::Sphere(sphere) => match sphere.command {
            SphereCommands::Create(args) => {
                create_sphere(&configuration, args).await;
            }
            SphereCommands::Delete(args) => {
                delete_sphere(&configuration, args).await;
            }
            SphereCommands::List(args) => {
                list_sphere(&configuration, &args).await;
            }
            SphereCommands::Status => todo!(),
            SphereCommands::Output(args) => {
                get_metasphere_outputs(&configuration, &args).await;
            }
        },
        Command::Project => todo!(),
        _ => {}
    }
}
