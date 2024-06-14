use clap::Args;
use openapi::{apis::{configuration::Configuration, metaspheres_api::{create_metasphere, delete_metasphere, get_metaspheres_from_project}}, models::CreateMetasphereRequest};

#[derive(Debug, Args)]
pub struct SphereListArgs {
    #[arg(short, long)]
    project_id: i32,
    #[arg(short, long)]
    show_deleted: Option<bool>
}

pub async fn list_sphere(configuration: &Configuration, args: &SphereListArgs) {
    match get_metaspheres_from_project(&configuration, args.project_id, args.show_deleted).await {
        Ok(result) => {
            println!("List of metaspheres for project {} :\n {}", args.project_id, serde_json::to_string_pretty(&result).unwrap());
        },
        Err(err) => {
            println!("Error getting metasphere list : {}", err);
        },
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
    instance_size: Option<String>,
    
}

pub async fn create_sphere(configuration: &Configuration, args: SphereCreateArgs) {
    let mut request = CreateMetasphereRequest::new(args.name);
    request.template = Some(Some(args.template));
    request.cloud_provider = Some(args.cloud_provider);
    request.cloud_region = Some(args.cloud_region);
    request.instance_count = Some(args.instance_count);
    //request.instance_size = Some(args.instance_size.into());
    match create_metasphere(configuration, args.project_id, request).await {
        Ok(result) => {
            println!("Metasphere creation success :\n {}", result.uuid);
        },
        Err(err) => {
            println!("Error creating metasphere : {}", err);
        },
    }
}

#[derive(Debug, Args)]
pub struct SphereDeleteArgs{
    #[arg(short, long)]
    metasphere_id: i32
}

pub async fn delete_sphere(configuration: &Configuration, args: SphereDeleteArgs ) {

    match delete_metasphere(configuration, args.metasphere_id).await {
        Ok(_) => {
            println!("Metasphere deletion request success :\n");
        },
        Err(err) => {
            println!("Error creating metasphere : {}", err);
        },
    }
}