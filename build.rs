use dotenvy::dotenv;
use std::{env, fs::File, io::Write};

fn main() -> std::io::Result<()>{
    dotenv().ok();
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=.env");
    println!("cargo::rerun-if-env-changed=AUTH0_DOMAIN");
    println!("cargo::rerun-if-env-changed=AUTH0_CLIENT_ID");
    println!("cargo::rerun-if-env-changed=AUTH0_AUDIENCE");
    println!("cargo::rerun-if-env-changed=BINARITE_URL");

    

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let file = File::create(std::path::Path::new(&out_dir).join("oauth.rs"))?;
    writeln!(&file, "static auth0_client_id: &str = \"{}\";", env::var("AUTH0_CLIENT_ID").unwrap())?;
    writeln!(&file, "static auth0_domain: &str = \"{}\";", env::var("AUTH0_DOMAIN").unwrap())?;
    writeln!(&file, "static auth0_audience: &str = \"{}\";", env::var("AUTH0_AUDIENCE").unwrap())?;
    writeln!(&file, "static binarite_url: &str = \"{}\";", env::var("BINARITE_URL").unwrap())?;
    Ok(())
}