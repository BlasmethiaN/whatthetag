use clap::{Arg, Command};
use colored::*;
use itertools::intersperse;
use serde_derive::Deserialize;
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Deserialize, Debug)]
struct Config {
  white_list: Option<HashSet<String>>,
  black_list: Option<HashSet<String>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let matches = Command::new("whatthetag")
    .version("1.0")
    .author("BlasmethiaN feat. sproott")
    .about("Search for doujins safely.")
    .arg(Arg::new("number").index(1))
    .arg(
      Arg::new("white_list")
        .short('w')
        .multiple_values(true)
        .takes_value(true)
        .required(false),
    )
    .arg(
      Arg::new("black_list")
        .short('b')
        .multiple_values(true)
        .takes_value(true)
        .required(false),
    )
    // .arg(Arg::new("image").short('i').long("img").required(false))
    .get_matches();

  let config_file_name = env::var("XDG_CONFIG_HOME").unwrap_or(env::var("HOME")? + "/.config")
    + "/whatthetag/config.toml";
  let config_str = fs::read_to_string(config_file_name).unwrap_or("".to_string());
  let config: Config = toml::from_str(&config_str)?;

  let number = matches.value_of("number").unwrap();
  // let image = matches.is_present("image");
  let white_list = matches
    .values_of("white_list")
    .map(|vals| vals.map(String::from).collect::<HashSet<_>>())
    .unwrap_or(config.white_list.unwrap_or(HashSet::new()));
  let black_list = matches
    .values_of("black_list")
    .map(|vals| vals.map(String::from).collect::<HashSet<_>>())
    .unwrap_or(config.black_list.unwrap_or(HashSet::new()));
  let url = "https://nhentai.net/api/gallery/";
  let resp = reqwest::get(String::from(url) + number)
    .await?
    .text()
    .await?;

  let parsed_respose: serde_json::Value = serde_json::from_str(&resp)?;

  let tags = parsed_respose["tags"]
    .as_array()
    .unwrap()
    .iter()
    .filter(|tag| tag["type"] == "tag")
    .map(|tag| tag["name"].as_str().unwrap().to_string())
    .collect::<Vec<_>>();
  let name = parsed_respose["title"]["pretty"].as_str().unwrap();
  let artists = parsed_respose["tags"]
    .as_array()
    .unwrap()
    .iter()
    .filter(|tag| tag["type"] == "artist")
    .map(|artist| artist["name"].as_str().unwrap().to_string())
    .collect::<Vec<_>>();

  println!("\n{} {}", "Title:".bold().yellow(), name.normal());
  println!(
    "{} {}\n",
    "Artists:".bold().yellow(),
    artists.join(", ").normal()
  );

  print!("{}", "Tags: ".bold().cyan());

  let mut tags = tags.clone();
  tags.sort();
  intersperse(
    tags.iter().map(|tag| {
      if white_list.contains(tag) {
        return tag.green();
      } else if black_list.contains(tag) {
        return tag.red();
      }
      tag.normal()
    }),
    ", ".normal(),
  )
  .for_each(|s| print!("{}", s));

  println!("");
  Ok(())
}

// TODO image printing
// TODO nonexistent doujin error
// TODO show help with wrong usage (panic!)
