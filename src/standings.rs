#![allow(dead_code)]

use scraper::{selectable::Selectable, ElementRef};
use serde::{Deserialize, Serialize};
use std::fs;

// internet resources
pub const DRIVER_STANDINGS: &str = "https://www.formula1.com/en/results.html/2024/drivers.html";
pub const TEAM_STANDINGS: &str = "https://www.formula1.com/en/results.html/2024/team.html";

use crate::error::{Error, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrStandings {
    pub position: usize,
    pub name: String,
    pub points: usize,
}

fn fetch_resource(url: &str) -> Result<String> {
    let data: String = ureq::get(url).call()?.into_string()?;
    Ok(data)
}

fn parse_standings_html_table(html: &str, results: &GpResults) -> Result<Vec<CurrStandings>> {
    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse("table.resultsarchive-table > tbody > tr")
        .map_err(|_| Error::Scraper)?;

    let table_body = document.select(&selector);

    let mut curr_standings: Vec<CurrStandings> = Vec::new();
    for element in table_body {
        let driver_details = results.parse_table_row(element)?;
        curr_standings.push(driver_details);
    }
    Ok(curr_standings)
}

pub enum GpResults {
    Driver,
    Team,
}

impl GpResults {
    fn parse_table_row(&self, element: ElementRef) -> Result<CurrStandings> {
        match self {
            Self::Driver => parse_driver_table_row(element),
            Self::Team => parse_team_table_row(element),
        }
    }

    fn get_inter_resource(&self) -> Result<String> {
        match self {
            Self::Driver => {
                println!("Fetching Driver standings from internet");
                fetch_resource(DRIVER_STANDINGS)
            }
            Self::Team => {
                println!("Fetching Team standings from internet");
                fetch_resource(TEAM_STANDINGS)
            }
        }
    }

    fn get_local_resource_loc(&self) -> &str {
        match self {
            GpResults::Driver => "2024_driver_standings.json",
            GpResults::Team => "2024_team_standings.json",
        }
    }

    pub fn get_standings(&self, force_save: bool) -> Result<Vec<CurrStandings>> {
        let schedule_dir = std::env::temp_dir().join("f1_2024_schedule");
        if !schedule_dir.exists() {
            std::fs::create_dir(&schedule_dir)?;
        }
        let standings_file = schedule_dir.join(self.get_local_resource_loc());
        if !standings_file.exists() || force_save {
            let raw_data = self.get_inter_resource()?;
            let parsed_data = parse_standings_html_table(&raw_data, self)?;
            let json_data_to_cache = serde_json::to_string(&parsed_data)?;
            fs::write(standings_file, json_data_to_cache)?;
            Ok(parsed_data)
        } else {
            let data = fs::read_to_string(standings_file)?;
            let standings: Vec<CurrStandings> = serde_json::from_str(&data)?;
            Ok(standings)
        }
    }
}

fn parse_driver_table_row(element: ElementRef) -> Result<CurrStandings> {
    let td_selector = scraper::Selector::parse("td").map_err(|_| Error::Scraper)?;
    let mut iter = element.select(&td_selector);

    // skip
    iter.next();

    // driver position
    let position = iter
        .next()
        .ok_or(Error::ParseDriverInfo)?
        .text()
        .collect::<Vec<_>>()[0]
        .parse::<usize>()?;

    // driver name
    let driver_name = iter.next().ok_or(Error::ParseDriverInfo)?;
    let span_selector = scraper::Selector::parse("span").map_err(|_| Error::Scraper)?;
    let mut span_iter = driver_name.select(&span_selector);
    let first = span_iter
        .next()
        .ok_or(Error::ParseDriverInfo)?
        .text()
        .collect::<Vec<_>>()[0];
    let second = span_iter
        .next()
        .ok_or(Error::ParseDriverInfo)?
        .text()
        .collect::<Vec<_>>()[0];
    let name = format!("{} {}", first, second);

    //skip
    iter.next();
    iter.next();

    // points
    let points = iter
        .next()
        .ok_or(Error::ParseDriverInfo)?
        .text()
        .collect::<Vec<_>>()[0]
        .parse::<usize>()?;

    Ok(CurrStandings {
        position,
        name,
        points,
    })
}

fn parse_team_table_row(element: ElementRef) -> Result<CurrStandings> {
    let td_selector = scraper::Selector::parse("td").map_err(|_| Error::Scraper)?;
    let mut iter = element.select(&td_selector);

    // skip
    iter.next();

    // team position
    let position = iter
        .next()
        .ok_or(Error::ParseTeamInfo)?
        .text()
        .collect::<Vec<_>>()[0]
        .parse::<usize>()?;

    // team name
    let team_name = iter.next().ok_or(Error::ParseTeamInfo)?;
    let name = team_name.text().collect::<Vec<&str>>();
    let name = name.get(1).ok_or_else(|| Error::ParseTeamInfo)?;
    let name = name.to_owned().to_owned();

    // points
    let points = iter
        .next()
        .ok_or(Error::ParseTeamInfo)?
        .text()
        .collect::<Vec<_>>()[0]
        .parse::<usize>()?;

    Ok(CurrStandings {
        position,
        name,
        points,
    })
}
