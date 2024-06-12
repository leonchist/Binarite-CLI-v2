#[path = "command-executor.rs"]
mod command_executor;
mod context;
mod login;
mod logout;
mod refresh;
mod sphere;

//use std::env::Args;

use std::borrow::Borrow;

use clap::{Args, Parser, Subcommand};
use command_executor::CommandExecutor;
pub use context::CommandContext;
use login::login;
use logout::logout;
use openapi::apis::configuration::Configuration;
use refresh::refresh;

//use clap::{command, Parser, Subcommand};
use cred_store::CredStore;

use crate::auth::get_token::get_token;

use self::sphere::{create_sphere, delete_sphere, list_sphere, SphereCreateArgs, SphereDeleteArgs, SphereListArgs};

#[derive(Parser)]
#[clap(author, version, about = "A MetaGravity command line tool")]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Login,
    Logout,
    Refresh,
    Sphere(SphereArgs)
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct SphereArgs {
    #[command(subcommand)]
    command: SphereCommands,
}

#[derive(Debug, Subcommand)]
enum SphereCommands{
    Create(SphereCreateArgs),
    Delete(SphereDeleteArgs),
    List(SphereListArgs),
    Status,
    Output
}

pub async fn parse_command<'a, T: CredStore>(context: &mut CommandContext<'a, T>) {
    let cli = Cli::parse();

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
    configuration.base_path = context.config.binarite_url.clone();

    match cli.command {
        Command::Login => todo!(),
        Command::Logout => todo!(),
        Command::Refresh => todo!(),
        Command::Sphere(sphere) => {
            match sphere.command {
                SphereCommands::Create(args) => {
                    create_sphere(&configuration, args).await;
                },
                SphereCommands::Delete(args) => {
                    delete_sphere(&configuration, args).await;
                },
                SphereCommands::List(args) => {
                    list_sphere(&configuration, &args).await;
                },
                SphereCommands::Status => todo!(),
                SphereCommands::Output => todo!(),
            }
        },
    }
}

// impl<T: CredStore> CommandExecutor<T> for Command {
//     fn execute(&self, context: &mut CommandContext<T>) {
//         match self {
//             Command::Login => login(context),
//             Command::Logout => logout(context),
//             Command::Refresh => refresh(context),
//             Command::Sphere(sphere) => {
//                 match sphere.command {
//                     SphereCommands::Create => todo!(),
//                     SphereCommands::Delete => todo!(),
//                     SphereCommands::List => list_sphere(context, ),
//                     SphereCommands::Status => todo!(),
//                     SphereCommands::Output => todo!(),
//                 }
//             }
//         }
//     }
// }

// pub fn invoke_command<T: CredStore>(context: &mut CommandContext<T>) {
//     let cli = Cli::parse();
//     cli.command.execute(context);
// }