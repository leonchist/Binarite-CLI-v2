use std::collections::HashMap;

use clap::{Args, ValueEnum};
use openapi::{
    apis::{
        configuration::Configuration,
        metaspheres_api::{create_metasphere, delete_metasphere, get_metaspheres_from_project},
    },
    models::{self, CreateMetasphereRequest},
};
use serde_json::json;

#[derive(Debug, Args)]
pub struct SphereListArgs {
    #[arg(short, long)]
    project_id: i32,
    #[arg(short, long)]
    show_deleted: Option<bool>,
}

pub async fn list_sphere(configuration: &Configuration, args: &SphereListArgs) {
    match get_metaspheres_from_project(configuration, args.project_id, args.show_deleted).await {
        Ok(result) => {
            println!(
                "List of metaspheres for project {} :\n {}",
                args.project_id,
                serde_json::to_string_pretty(&result).unwrap()
            );
        }
        Err(err) => {
            println!("Error getting metasphere list : {}", err);
        }
    };
}

#[derive(Debug, Args)]
pub struct SphereCreateArgs {
    #[arg(short, long)]
    project_id: i32,
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    template: String,
    #[arg(long)]
    cloud_provider: Option<String>,
    #[arg(long)]
    cloud_region: Option<String>,
    #[arg(long)]
    instance_count: Option<i32>,
    #[arg(long)]
    instance_size: Option<AltInstanceSize>,
    #[arg(long)]
    custom_args: Option<String>,
}

#[derive(ValueEnum, Clone, Debug)]
enum AltInstanceSize {
    S,
    M,
    L,
    Xl,
}

pub async fn create_sphere(configuration: &Configuration, args: SphereCreateArgs) {
    let mut request = CreateMetasphereRequest::new(args.name);
    request.template = Some(Some(args.template));
    request.cloud_provider = Some(args.cloud_provider);
    request.cloud_region = Some(args.cloud_region);
    request.instance_count = Some(args.instance_count);
    if let Some(custom_args) = args.custom_args {
        let mut custom_arguments = HashMap::new();
        let args = custom_args.split(',');
        for arg in args {
            if let Some((key, value)) = arg.split_once('=') {
                custom_arguments.insert(key, value);
            }
        }
        request.terraform_var = Some(Some(json!(custom_arguments)));
    }
    if let Some(instance_size) = args.instance_size {
        let instance_size = match instance_size {
            AltInstanceSize::S => models::InstanceSize::S,
            AltInstanceSize::M => models::InstanceSize::M,
            AltInstanceSize::L => models::InstanceSize::L,
            AltInstanceSize::Xl => models::InstanceSize::Xl,
        };
        request.instance_size = Some(Some(instance_size));
    }
    match create_metasphere(configuration, args.project_id, request).await {
        Ok(result) => {
            println!(
                "Metasphere creation success :\n\tuuid : {}\n\tid : {}",
                result.uuid, result.id
            );
        }
        Err(err) => {
            println!("Error creating metasphere : {}", err);
        }
    }
}

#[derive(Debug, Args)]
pub struct SphereDeleteArgs {
    #[arg(short, long)]
    metasphere_id: i32,
}

pub async fn delete_sphere(configuration: &Configuration, args: SphereDeleteArgs) {
    match delete_metasphere(configuration, args.metasphere_id).await {
        Ok(_) => {
            println!("Metasphere deletion request success :\n");
        }
        Err(err) => {
            println!("Error creating metasphere : {}", err);
        }
    }
}
