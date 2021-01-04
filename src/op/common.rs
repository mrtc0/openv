extern crate base64_url;
extern crate serde;

use std::collections::BTreeMap as Map;
use std::str;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordHistory {
    pub time: u64,
    pub value: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    pub name: String,
    pub title: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Overview {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<Map<String, String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ainfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ps: Option<u32>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub designation: String,
    pub name: String,
    pub r#type: String,
    pub value: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Detail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Field>>,
    pub notes_plain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    pub password_history: Vec<PasswordHistory>,
    pub sections: Vec<Section>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub uuid: String,
    pub template_uuid: String,
    pub trashed: String,
    pub created_at: String,
    pub updated_at: String,
    pub changer_uuid: String,
    pub item_version: i32,
    pub vault_uuid: String,
    pub overview: Overview,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Detail>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Password {
    notes_plain: String,
    password: String,
    password_history: Vec<PasswordHistory>,
    sections: Vec<Section>,
}

pub fn encode_password(password: &str) -> Result<String, String> {
    let s = Password {
        notes_plain: "".to_string(),
        password: password.to_string(),
        password_history: vec![],
        sections: vec![],
    };

    match serde_json::to_string(&s) {
        Ok(j) => Ok(base64_url::encode(j.as_bytes())),
        Err(err) => return Err(format!("{}", err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_password() {
        let s = "password";
        let expect: Result<String, String> = Ok("eyJub3Rlc1BsYWluIjoiIiwicGFzc3dvcmQiOiJwYXNzd29yZCIsInBhc3N3b3JkSGlzdG9yeSI6W10sInNlY3Rpb25zIjpbXX0".to_string());
        assert_eq!(encode_password(&s), expect,);
    }
}
