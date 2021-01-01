extern crate base64;
extern crate serde;
extern crate serde_json;

mod op;

use clap::Clap;
use std::io::{self, Write};
use std::str;

use crate::op::OnePassword;

#[derive(Clap)]
#[clap(version = "0.1", author = "mrtc0")]
struct Opts {
    vault: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "0.1", author = "mrtc0")]
    Create(Create),
    List(List),
    Get(Get),
}

#[derive(Clap)]
struct Create {
    title: String,
}

#[derive(Clap)]
struct List {}

#[derive(Clap)]
struct Get {
    #[clap(short, multiple = true)]
    name: Option<Vec<String>>,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Create(create) => {
            let op = OnePassword::new(opts.vault);
            print!("{}.{} > ", op.vault, create.title);
            io::stdout().flush().unwrap();
            let password = get_value_with_interactive();
            match op.create(&create.title, &password) {
                Ok(_) => println!("created"),
                Err(err) => println!("{}", err),
            }
        }
        SubCommand::List(_) => {
            let op = OnePassword::new(opts.vault);
            match op.list() {
                Ok(items) => {
                    for i in items {
                        println!("{}", i.overview.title)
                    }
                }
                Err(err) => println!("* {}", err),
            }
        }
        SubCommand::Get(get) => {
            let op = OnePassword::new(opts.vault);
            match get.name {
                Some(names) => {
                    for n in names {
                        let (name, env_name) = name_pair(&n);
                        match op.get(&name) {
                            Ok(item) => {
                                if let Some(details) = item.details {
                                    println!("{}={}", env_name, details.password);
                                }
                            }
                            Err(e) => {
                                println!("{}", e);
                            }
                        }
                    }
                }
                None => match op.list() {
                    Ok(items) => {
                        for i in items {
                            let name = i.overview.title;
                            match op.get(&name) {
                                Ok(item) => {
                                    if let Some(details) = item.details {
                                        println!("{}={}", name, details.password);
                                    }
                                }
                                Err(e) => {
                                    println!("{}", e);
                                }
                            }
                        }
                    }
                    Err(err) => println!("{}", err),
                },
            }
        }
    }
}

fn get_value_with_interactive() -> String {
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("failed to read from pipe");
    value = value.trim().to_string();

    return value;
}

// name_pair do split with `:` and
// return the pair of item name in 1password and environment variable name
fn name_pair(s: &str) -> (String, String) {
    let v: Vec<&str> = s.split(":").collect();
    let len = v.len();
    if len > 1 {
        return (v[..len - 1].join(":").to_string(), v[len - 1].to_string());
    }
    return (s.to_string(), s.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_pair() {
        let s = "title:MY_ENV";
        let include_colon_in_title = "this-is:title:MY_ENV";
        let not_include_colon = "MY_ENV";

        assert_eq!(name_pair(&s), ("title".to_string(), "MY_ENV".to_string()),);
        assert_eq!(
            name_pair(&include_colon_in_title),
            ("this-is:title".to_string(), "MY_ENV".to_string()),
        );
        assert_eq!(
            name_pair(&not_include_colon),
            ("MY_ENV".to_string(), "MY_ENV".to_string()),
        );
    }
}
