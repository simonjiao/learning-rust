pub fn run() {
    format_json().unwrap();
    toml_deserialize().unwrap();
    byteorder_integer().unwrap();
}

use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug, PartialEq, Default)]
struct Payload {
    kind: u8,
    value: u16,
}

fn byteorder_integer() -> Result<()> {
    let payload = Payload::default();
    let encoded_bytes = encode(&payload)?;
    let decoded = decode(&encoded_bytes)?;
    assert_eq!(decoded, payload);
    Ok(())
}

fn encode(payload: &Payload) -> Result<Vec<u8>> {
    let mut bytes = vec![];
    bytes.write_u8(payload.kind)?;
    bytes.write_u16::<LittleEndian>(payload.value)?;
    Ok(bytes)
}

fn decode(mut bytes: &[u8]) -> Result<Payload> {
    Ok(Payload {
        kind: bytes.read_u8()?,
        value: bytes.read_u16::<LittleEndian>()?
    })
}

use serde_json::json;
use serde_json::Value;

fn format_json() -> Result<()> {
    let j = r#"{
                 "userid": 103609,
                 "verified": true,
                 "access_privileges": [
                   "user",
                   "admin"
                 ]
               }"#;

    let parsed:Value = serde_json::from_str(j)?;

    let expected = json!({
       "userid": 103609,
        "verified": true,
        "access_privileges": ["user", "admin"],
    });

    assert_eq!(expected, parsed);
    Ok(())
}

use toml::Value as TomlValue;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Config {
    package: Package,
    dependencies: HashMap<String, String>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

fn toml_deserialize() -> Result<()> {
    let toml_content = r#"
          [package]
          name = "your_package"
          version = "0.1.0"
          authors = ["You! <you@example.org>"]

          [dependencies]
          serde = "1.0"
    "#;

    let package_info:TomlValue = toml::from_str(toml_content)?;

    assert_eq!(package_info["dependencies"]["serde"].as_str(), Some("1.0"));
    assert_eq!(package_info["package"]["name"].as_str(), Some("your_package"));

    let package_info:Config = toml::from_str(toml_content)?;
    assert_eq!(package_info.package.name, "your_package");
    assert_eq!(package_info.dependencies["serde"], "1.0");
    assert_eq!(package_info.package.authors, vec!["You! <you@example.org>"]);
    assert_eq!(package_info.package.version, "0.1.0");

    Ok(())
}