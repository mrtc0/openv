extern crate serde_json;

mod common;

use std::process::Command;
use std::str;

pub use self::common::Item;

pub struct OnePassword {
    pub vault: String,
}

impl OnePassword {
    pub fn new(vault: String) -> Self {
        Self { vault: vault }
    }

    pub fn create(&self, title: &str, password: &str) -> Result<String, String> {
        match self::common::encode_password(password) {
            Ok(encoded_value) => {
                let args = vec![
                    "create",
                    "item",
                    "Password",
                    "--title",
                    title,
                    "--vault",
                    &self.vault,
                    &encoded_value,
                ];
                OnePassword::exec(args)
            }
            Err(err) => return Err(format!("encode failed: {}", err)),
        }
    }

    pub fn list(&self) -> Result<Vec<Item>, String> {
        let args = vec!["list", "items", "--vault", &self.vault];
        let result = match OnePassword::exec(args) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

        match serde_json::from_str::<Vec<Item>>(&result) {
            Ok(items) => return Ok(items),
            Err(e) => return Err(format!("failed to parse json: {}", e)),
        }
    }

    pub fn get(&self, title: &str) -> Result<Item, String> {
        let args = vec!["get", "item", "--vault", &self.vault, title];
        let result = match OnePassword::exec(args) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

        match serde_json::from_str::<Item>(&result) {
            Ok(item) => return Ok(item),
            Err(e) => return Err(format!("failed to parse json: {}", e)),
        }
    }

    pub fn exec(args: Vec<&str>) -> Result<String, String> {
        let mut cmd = Command::new("op");
        for arg in args {
            cmd.arg(arg);
        }
        match cmd.output() {
            Ok(result) => {
                if result.status.success() {
                    return Ok(str::from_utf8(&result.stdout).unwrap().to_string());
                } else {
                    return Err(str::from_utf8(&result.stderr).unwrap().to_string());
                }
            }
            Err(err) => return Err(err.to_string()),
        }
    }
}
