use clap::{Arg, Command};
use colored::*;
use itertools::intersperse;
use scraper::{ElementRef, Html, Selector};
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
    let url = "https://nhentai.net/g/";
    let resp = reqwest::get(String::from(url) + number)
        .await?
        .text()
        .await?;
    let document = Html::parse_document(&resp);
    let selector = Selector::parse("section#tags > div > span > a").unwrap();
    let name_selector = Selector::parse(".title > .pretty").unwrap();

    let tags: Vec<_> = document
        .select(&selector)
        .filter(|element| element.value().attr("href").unwrap().contains("/tag/"))
        .collect();
    let name = document.select(&name_selector).collect::<Vec<_>>()[0].inner_html();
    println!("\n{} {}\n", "Title:".bold().yellow(), name.yellow());

    let span_selector = Selector::parse("span").unwrap();

    print!("{}", "Tags: ".bold().cyan());
    let mut tags_text = tags
        .iter()
        .map(|element| element.select(&span_selector).collect::<Vec<ElementRef>>()[0].inner_html())
        .collect::<Vec<_>>();
    tags_text.sort();
    intersperse(
        tags_text.iter().map(|tag| {
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
