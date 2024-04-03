#![allow(dead_code)]

use scraper::{selectable::Selectable, ElementRef};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::error::{Error, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct Driver {
    pub position: usize,
    pub name: String,
    pub points: usize,
}

fn fetch_f1_standings() -> Result<String> {
    println!("Fetching standings from internet");
    let data: String = ureq::get("https://www.formula1.com/en/results.html/2024/drivers.html")
        .call()?
        .into_string()?;
    Ok(data)
}

fn parse_html_driver_standings(html: &str) -> Result<Vec<Driver>> {
    // let document = scraper::Html::parse_document(html);
    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse("table.resultsarchive-table > tbody > tr")
        .map_err(|_| Error::Scraper)?;

    let table_body = document.select(&selector);

    let mut standings: Vec<Driver> = Vec::new();
    for element in table_body {
        // for element in table_body.select(&row_selector) {
        let driver_details = parse_driver_table_row(element)?;
        standings.push(driver_details);
    }
    Ok(standings)
}

fn parse_driver_table_row(element: ElementRef) -> Result<Driver> {
    let td_selector = scraper::Selector::parse("td").map_err(|_| Error::Scraper)?;
    let mut iter = element.select(&td_selector);

    // skip
    iter.next();

    // driver position
    let position = iter
        .next()
        .ok_or(Error::ParserDriverInfo)?
        .text()
        .collect::<Vec<_>>()[0]
        .parse::<usize>()?;

    // driver name
    let driver_name = iter.next().ok_or(Error::ParserDriverInfo)?;
    let span_selector = scraper::Selector::parse("span").map_err(|_| Error::Scraper)?;
    let mut span_iter = driver_name.select(&span_selector);
    let first = span_iter
        .next()
        .ok_or(Error::ParserDriverInfo)?
        .text()
        .collect::<Vec<_>>()[0];
    let second = span_iter
        .next()
        .ok_or(Error::ParserDriverInfo)?
        .text()
        .collect::<Vec<_>>()[0];
    let name = format!("{} {}", first, second);

    //skip
    iter.next();
    iter.next();

    // points
    let points = iter
        .next()
        .ok_or(Error::ParserDriverInfo)?
        .text()
        .collect::<Vec<_>>()[0]
        .parse::<usize>()?;

    Ok(Driver {
        position,
        name,
        points,
    })
}

pub fn driver_standings() -> Result<Vec<Driver>> {
    let schedule_dir = std::env::temp_dir().join("f1_2024_schedule");
    if !schedule_dir.exists() {
        std::fs::create_dir(&schedule_dir)?;
    }
    let standings_file = schedule_dir.join("2024_driver_standings.json");
    if standings_file.exists() {
        let data = fs::read_to_string(standings_file)?;
        let standings: Vec<Driver> = serde_json::from_str(&data)?;
        Ok(standings)
    } else {
        let raw_data = fetch_f1_standings()?;
        let parsed_data = parse_html_driver_standings(&raw_data)?;
        let json_data_to_cache = serde_json::to_string(&parsed_data)?;
        fs::write(standings_file, json_data_to_cache)?;
        Ok(parsed_data)
    }
}
