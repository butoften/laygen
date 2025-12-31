use anyhow::Result;
use std::fs;
use std::path::Path;
use crate::i18n::I18n;
use crate::file_ops::update_mod_file;

pub fn generate_controller(
    dir_path: &Path,
    file_name: &str,
    method_name: &str,
    service_file_name: &str,
    i18n: &I18n,
) -> Result<()> {
    // Create directory if it doesn't exist
    fs::create_dir_all(dir_path)?;

    let file_path = dir_path.join(format!("{}.rs", file_name));
    
    // Generate controller content
    let content = generate_controller_template(file_name, method_name, service_file_name);
    
    fs::write(&file_path, content)?;
    println!("{}", i18n.file_created(&file_path.display().to_string()));

    // Update mod.rs or lib.rs
    let updated_file = update_mod_file(dir_path, file_name)?;
    println!("{}", i18n.mod_updated(&updated_file.display().to_string()));

    Ok(())
}

pub fn generate_service(
    dir_path: &Path,
    file_name: &str,
    method_name: &str,
    i18n: &I18n,
) -> Result<()> {
    // Create directory if it doesn't exist
    fs::create_dir_all(dir_path)?;

    let file_path = dir_path.join(format!("{}.rs", file_name));
    
    // Generate service content
    let content = generate_service_template(method_name);
    
    fs::write(&file_path, content)?;
    println!("{}", i18n.file_created(&file_path.display().to_string()));

    // Update mod.rs or lib.rs
    let updated_file = update_mod_file(dir_path, file_name)?;
    println!("{}", i18n.mod_updated(&updated_file.display().to_string()));

    Ok(())
}

fn generate_controller_template(file_name: &str, method_name: &str, service_file_name: &str) -> String {
    // Convert file_name to PascalCase for struct names
    let struct_name = to_pascal_case(file_name);
    
    format!(r#"use axum::extract::Query;
use axum::extract::State;
use axum::response::Json;
use axum::Extension;
use error_crate::api_error::{{ApiError, ApiResult}};
use public::safe_json::SafeJson;
use public::public::{{
    origin_display, public_list_response, ApiResponse, AppState, PublicListResponse, QueryParams,
    ReturnResult,
}};
use service_admin_chinese_hsk::{};

use serde::{{Deserialize, Serialize}};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct {}Request {{
    // Add your request fields here
}}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct {}Response {{
    // Add your response fields here
}}

pub async fn {}(
    Extension(domain): Extension<String>,
    Query(query): Query<QueryParams>,
    State(state): State<AppState>,
    SafeJson(body): SafeJson<{}Request>,
) -> ApiResult<Json<ApiResponse<serde_json::Value>>> {{
    let res = {}::{}(&state.conn, &domain).await?;
    
    let result = origin_display(query, res, &domain);
    
    Ok(ApiResponse::success(result).to_json())
}}
"#, service_file_name, struct_name, struct_name, method_name, struct_name, service_file_name, method_name)
}

fn generate_service_template(method_name: &str) -> String {
    format!(r#"use sea_orm::DatabaseConnection;
use error_crate::api_error::ApiResult;

pub async fn {}(
    conn: &DatabaseConnection,
    domain: &str,
) -> ApiResult<()> {{
    // Add your service logic here
    
    Ok(())
}}
"#, method_name)
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("user_profile"), "UserProfile");
        assert_eq!(to_pascal_case("test"), "Test");
        assert_eq!(to_pascal_case("my_api_controller"), "MyApiController");
    }
}

