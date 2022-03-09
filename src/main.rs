#![feature(iter_intersperse)]

use clap::{Arg, Command};
use colored::*;
use scraper::{ElementRef, Html, Selector};
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("whatthetag")
        .version("1.0")
        .author("BlasmethiaN")
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
        .arg(Arg::new("image").short('i').long("img").required(false))
        .get_matches();

    let number = matches.value_of("number").unwrap();
    let image = matches.is_present("image");
    let white_list = matches
        .values_of("white_list")
        .map(|vals| vals.collect::<HashSet<_>>())
        .unwrap_or(HashSet::new());
    let black_list = matches
        .values_of("black_list")
        .map(|vals| vals.collect::<HashSet<_>>())
        .unwrap_or(HashSet::new());
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
    println!("\n{}\n", name.yellow());

    let span_selector = Selector::parse("span").unwrap();

    let mut tags_text = tags
        .iter()
        .map(|element| element.select(&span_selector).collect::<Vec<ElementRef>>()[0].inner_html())
        .collect::<Vec<_>>();
    tags_text.sort();
    tags_text
        .iter()
        .map(|tag| {
            if white_list.contains(&tag.as_str()) {
                return tag.green();
            } else if black_list.contains(&tag.as_str()) {
                return tag.red();
            }
            tag.normal()
        })
        .intersperse(", ".normal())
        .for_each(|s| print!("{}", s));
    println!("");
    Ok(())
}