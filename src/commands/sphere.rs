use std::collections::HashMap;

use clap::{ArgAction, Args, ValueEnum};
use openapi::{
    apis::{configuration::Configuration, metaspheres_api},
    models,
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
    match metaspheres_api::get_metaspheres_from_project(
        configuration,
        args.project_id,
        args.show_deleted,
    )
    .await
    {
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
    let mut request = models::CreateMetasphereRequest::new(args.name);
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
    match metaspheres_api::create_metasphere(configuration, args.project_id, request).await {
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
    match metaspheres_api::delete_metasphere(configuration, args.metasphere_id).await {
        Ok(_) => {
            println!("Metasphere deletion request success :\n");
        }
        Err(err) => {
            println!("Error creating metasphere : {}", err);
        }
    }
}

#[derive(Debug, Args)]
pub struct SphereOutputArgs {
    #[arg(short, long)]
    metasphere_id: i32,
    #[arg(long, action=ArgAction::SetTrue)]
    public_ip: Option<bool>,
}

pub async fn get_metasphere_outputs(configuration: &Configuration, args: &SphereOutputArgs) {
    let output =
        match metaspheres_api::get_metasphere_outputs(configuration, args.metasphere_id).await {
            Ok(result) => result,
            Err(err) => {
                println!("Error getting metasphere output : {}", err);
                std::process::exit(1);
            }
        };

    if let Some(true) = args.public_ip {
        if let Some(output) = output.output {
            let public_ips = output
                .get("public_ips")
                .and_then(|value| value.get("value"));
            if let Some(public_ips) = public_ips {
                println!(
                    "Metasphere public ips :\n {}",
                    serde_json::to_string_pretty(&public_ips).unwrap()
                );
            }
        }
    } else {
        println!(
            "Metasphere outputs :\n {}",
            serde_json::to_string_pretty(&output).unwrap()
        );
    }
}
