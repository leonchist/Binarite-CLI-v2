use commands::{parse_command, CommandContext};
use config::Config;
use cred_store::{CredStore, Credentials};
use dotenvy_macro::dotenv;
//use openapi::apis::{configuration::Configuration, metaspheres_api::get_metaspheres_from_project};

mod auth;
mod commands;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = Config::new();
    let mut credentials = Credentials::new()
        .set_file_name(".mg/auth".to_string())
        .build()
        .load()
        .expect("Failed to load credentials");

    let mut context = CommandContext {
        config: &config,
        cred_store: &mut credentials,
    };
    
    //invoke_command(&mut context);
    parse_command(&mut context).await;
    
    Ok(())

    // let mut configuration = Configuration::new();
    // // configuration.api_key = Some(ApiKey{
    // //     key: "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImNuN1ppQ3VjUzVPSHlNMm9GT0RrYiJ9.eyJlbWFpbCI6InRoaWJhdWx0K2FkbWluQG1ldGFncmF2aXR5LmNvbSIsImJpbmFyaXRlLXJvbGVzIjpbIk1HX0FETUlOIl0sImJpbmFyaXRlLXBlcm1pc3Npb25zIjpbIndzLW1nLWVvYy1hcGkiLCJFZGdlT2ZDaGFvcy1wcm9qZWN0LWNyZWF0ZSIsInRoaWJhdWx0LXRlc3QzLW1ldGFncmF2aXR5LWVkZ2VvZmNoYW9zLWRlbGV0ZSIsInRoaWJhdWx0LXRlc3QzLW1ldGFncmF2aXR5LWVkZ2VvZmNoYW9zLXVwZGF0ZSIsInRoaWJhdWx0LXRlc3QzLW1ldGFncmF2aXR5LWVkZ2VvZmNoYW9zLW5hbWVzcGFjZS11cGRhdGUiXSwiaXNzIjoiaHR0cHM6Ly9kZXYtdmY1bW9vend6YzhuZWc2dC5ldS5hdXRoMC5jb20vIiwic3ViIjoiYXV0aDB8NjU3MDc2ZjY4OWJhYzU0NGE3YjM0ZWMyIiwiYXVkIjpbImh0dHA6Ly9sb2NhbGhvc3Q6ODA4MC8iLCJodHRwczovL2Rldi12ZjVtb296d3pjOG5lZzZ0LmV1LmF1dGgwLmNvbS91c2VyaW5mbyJdLCJpYXQiOjE3MDU1Njc2MTUsImV4cCI6MTcwNTY1NDAxNSwiYXpwIjoiOVBxOW9ibGpabzFENWlPdDJBUUhYcVZGdHFwb1pFaXciLCJzY29wZSI6Im9wZW5pZCBwcm9maWxlIGVtYWlsIGNoYW5nZV9wYXNzd29yZCJ9.lcKKVj54Eug7QHHHTiOMw3QkOw3IpcdBIwGh7hBcKUMR_fBil5wXY-q65fdHhgzEjZI8On3miVZT5h0QPZimBI1ArH8nuTE1RCgLvODv28rNlPP7LkTijpBs2zXvAi1CKDZohzsac-tu-HmduqzuI_QSMdBuFa8DazrAZh0fnb-MCG8euxIGjCEZ98spkwKx58Yn7t1yuo0dcPKOHzjtDPN0Kc0v9eZaDDqgd_pYTJzzcud7inoWXj3U8Aa902uMwV2IMVphNCRM6fcS4-O7DhYKPZwjRPn1uyOtjlJuoaWfkrlfkYO3tFlAUbvKLIf5W_mx_PeCi-q4yycf5ImDzA".into(),
    // //     prefix: None
    // // });

    // // configuration.oauth_access_token = Some("eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImNuN1ppQ3VjUzVPSHlNMm9GT0RrYiJ9.eyJlbWFpbCI6InRoaWJhdWx0K2FkbWluQG1ldGFncmF2aXR5LmNvbSIsImJpbmFyaXRlLXJvbGVzIjpbIk1HX0FETUlOIl0sImJpbmFyaXRlLXBlcm1pc3Npb25zIjpbIndzLW1nLWVvYy1hcGkiLCJFZGdlT2ZDaGFvcy1wcm9qZWN0LWNyZWF0ZSIsInRoaWJhdWx0LXRlc3QzLW1ldGFncmF2aXR5LWVkZ2VvZmNoYW9zLWRlbGV0ZSIsInRoaWJhdWx0LXRlc3QzLW1ldGFncmF2aXR5LWVkZ2VvZmNoYW9zLXVwZGF0ZSIsInRoaWJhdWx0LXRlc3QzLW1ldGFncmF2aXR5LWVkZ2VvZmNoYW9zLW5hbWVzcGFjZS11cGRhdGUiXSwiaXNzIjoiaHR0cHM6Ly9kZXYtdmY1bW9vend6YzhuZWc2dC5ldS5hdXRoMC5jb20vIiwic3ViIjoiYXV0aDB8NjU3MDc2ZjY4OWJhYzU0NGE3YjM0ZWMyIiwiYXVkIjpbImh0dHA6Ly9sb2NhbGhvc3Q6ODA4MC8iLCJodHRwczovL2Rldi12ZjVtb296d3pjOG5lZzZ0LmV1LmF1dGgwLmNvbS91c2VyaW5mbyJdLCJpYXQiOjE3MDU1Njc2MTUsImV4cCI6MTcwNTY1NDAxNSwiYXpwIjoiOVBxOW9ibGpabzFENWlPdDJBUUhYcVZGdHFwb1pFaXciLCJzY29wZSI6Im9wZW5pZCBwcm9maWxlIGVtYWlsIGNoYW5nZV9wYXNzd29yZCJ9.lcKKVj54Eug7QHHHTiOMw3QkOw3IpcdBIwGh7hBcKUMR_fBil5wXY-q65fdHhgzEjZI8On3miVZT5h0QPZimBI1ArH8nuTE1RCgLvODv28rNlPP7LkTijpBs2zXvAi1CKDZohzsac-tu-HmduqzuI_QSMdBuFa8DazrAZh0fnb-MCG8euxIGjCEZ98spkwKx58Yn7t1yuo0dcPKOHzjtDPN0Kc0v9eZaDDqgd_pYTJzzcud7inoWXj3U8Aa902uMwV2IMVphNCRM6fcS4-O7DhYKPZwjRPn1uyOtjlJuoaWfkrlfkYO3tFlAUbvKLIf5W_mx_PeCi-q4yycf5ImDzA".into());

    // configuration.bearer_access_token = Some("eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImNuN1ppQ3VjUzVPSHlNMm9GT0RrYiJ9.eyJlbWFpbCI6InRoaWJhdWx0K2FkbWluQG1ldGFncmF2aXR5LmNvbSIsImJpbmFyaXRlLXJvbGVzIjpbIk1HX0FETUlOIl0sImJpbmFyaXRlLXBlcm1pc3Npb25zIjpbIndzLW1nLWVvYy1hcGkiLCJFZGdlT2ZDaGFvcy1wcm9qZWN0LWNyZWF0ZSIsInRoaWJhdWx0LXRlc3QzLW1ldGFncmF2aXR5LWVkZ2VvZmNoYW9zLWRlbGV0ZSIsInRoaWJhdWx0LXRlc3QzLW1ldGFncmF2aXR5LWVkZ2VvZmNoYW9zLXVwZGF0ZSIsInRoaWJhdWx0LXRlc3QzLW1ldGFncmF2aXR5LWVkZ2VvZmNoYW9zLW5hbWVzcGFjZS11cGRhdGUiXSwiaXNzIjoiaHR0cHM6Ly9kZXYtdmY1bW9vend6YzhuZWc2dC5ldS5hdXRoMC5jb20vIiwic3ViIjoiYXV0aDB8NjU3MDc2ZjY4OWJhYzU0NGE3YjM0ZWMyIiwiYXVkIjpbImh0dHA6Ly9sb2NhbGhvc3Q6ODA4MC8iLCJodHRwczovL2Rldi12ZjVtb296d3pjOG5lZzZ0LmV1LmF1dGgwLmNvbS91c2VyaW5mbyJdLCJpYXQiOjE3MDU1Njc2MTUsImV4cCI6MTcwNTY1NDAxNSwiYXpwIjoiOVBxOW9ibGpabzFENWlPdDJBUUhYcVZGdHFwb1pFaXciLCJzY29wZSI6Im9wZW5pZCBwcm9maWxlIGVtYWlsIGNoYW5nZV9wYXNzd29yZCJ9.lcKKVj54Eug7QHHHTiOMw3QkOw3IpcdBIwGh7hBcKUMR_fBil5wXY-q65fdHhgzEjZI8On3miVZT5h0QPZimBI1ArH8nuTE1RCgLvODv28rNlPP7LkTijpBs2zXvAi1CKDZohzsac-tu-HmduqzuI_QSMdBuFa8DazrAZh0fnb-MCG8euxIGjCEZ98spkwKx58Yn7t1yuo0dcPKOHzjtDPN0Kc0v9eZaDDqgd_pYTJzzcud7inoWXj3U8Aa902uMwV2IMVphNCRM6fcS4-O7DhYKPZwjRPn1uyOtjlJuoaWfkrlfkYO3tFlAUbvKLIf5W_mx_PeCi-q4yycf5ImDzA".into());

    // configuration.base_path = "http://localhost:8080".into();

    // let project_id = 1;

    // match get_metaspheres_from_project(&configuration, project_id, Some(true)).await {
    //     Ok(spheres) => {
    //         dbg!(spheres);
    //     }
    //     Err(err) => {
    //         println!("{}", err.to_string());
    //     }
    // }
}
