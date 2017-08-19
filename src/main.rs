extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;

#[macro_use]
extern crate error_chain;

use std::env;
use std::fs::File;
use std::io::{self, Read};

mod error;


fn std_in() -> error::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}


fn arg_in() -> error::Result<Option<String>> {
    Ok(if let Some(arg1) = env::args().nth(1) {
        let mut file = File::open(&arg1)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Some(buffer)
    } else {
        None
    })
}


#[derive(Serialize, Debug)]
enum Value {
    Json(serde_json::Value),
    Yaml(serde_yaml::Value),
    Toml(toml::value::Value),
}

impl Value {
    fn to_json(&self) -> error::Result<String> {
        match *self {
            Value::Json(ref value) => serde_json::to_string_pretty(&value),
            Value::Yaml(ref value) => serde_json::to_string_pretty(&value),
            Value::Toml(ref value) => serde_json::to_string_pretty(&value),
        }
        .map_err(Into::into)
    }
}

fn parse_json(raw: &str) -> error::Result<Value> {
    Ok(Value::Json(serde_json::from_str(raw)?))
}

fn parse_yaml(raw: &str) -> error::Result<Value> {
    Ok(Value::Yaml(serde_yaml::from_str(raw)?))
}

fn parse_toml(raw: &str) -> error::Result<Value> {
    Ok(Value::Toml(toml::from_str(raw)?))
}

fn _main() -> error::Result<()> {
    let raw = match arg_in()? {
        Some(raw) => raw,
        None => std_in()?
    };
    let parsed = parse_json(&raw)
        .or(parse_yaml(&raw))
        .or(parse_toml(&raw))?;

    println!("{}", parsed.to_json()?);
    Ok(())
}

quick_main!(_main);
