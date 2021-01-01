extern crate serde;

use std::process::Command;
use std::str;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PasswordHistory {
    pub time: u64,
    pub value: String
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Section {
    pub name: String,
    pub title: String
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Password {
    note_plain: String,
    password: String,
    password_history: Vec<PasswordHistory>,
    sections: Vec<Section>,
}

fn encode_password(password: &str) -> Result<String, String> {
    let s = Password {
        note_plain: "".to_string(),
        password: password.to_string(),
        password_history: vec![],
        sections: vec![],
    };

    match serde_json::to_string(&s) {
        Ok(j) => Ok(base64::encode(j.as_bytes())),
        Err(err) => return Err(format!("{}", err)),
    }
}

pub fn exec(vault: &str, title: &str, password: &str) -> Result<String, String> {
    match encode_password(password) {
        Ok(encoded_value) => {
            match Command::new("op").
                arg("create").
                arg("item").
                arg("Password").
                arg("--title").
                arg(title).
                arg("--vault").
                arg(vault).
                arg(encoded_value).output() {
                    Ok(result) => {
                        if result.status.success() {
                            return Ok(str::from_utf8(&result.stdout).unwrap().to_string());
                        } else {
                            return Err(str::from_utf8(&result.stderr).unwrap().to_string());
                        }
                    },
                    Err(err) => { return Err(err.to_string()); }
                }
            },
        Err(err) => {
            return Err(err.to_string())
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_password() {
        let s = "password";
        let expect: Result<String, String> = Ok("eyJub3RlUGxhaW4iOiIiLCJwYXNzd29yZCI6InBhc3N3b3JkIiwicGFzc3dvcmRIaXN0b3J5IjpbXSwic2VjdGlvbnMiOltdfQ==".to_string());
        assert_eq!(
            encode_password(&s),
            expect,
                  );
    }
}
