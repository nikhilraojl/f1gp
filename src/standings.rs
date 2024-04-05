use scraper::{selectable::Selectable, ElementRef};
use serde::{Deserialize, Serialize};
use std::fs;

// internet resources
pub const DRIVER_STANDINGS: &str = "https://www.formula1.com/en/results.html/2024/drivers.html";
pub const TEAM_STANDINGS: &str = "https://www.formula1.com/en/results.html/2024/team.html";

use crate::error::{Error, Result};
use crate::utils::get_or_create_tmp_dir;

#[derive(Debug, Deserialize, Serialize)]
pub struct PositionInfo {
    pub position: usize,
    pub name: String,
    pub points: usize,
}

fn parse_standings_html_table(html: &str, results: &GpResults) -> Result<Vec<PositionInfo>> {
    // As of this commit same table css selector can be used
    // for DRIVER_STANDINGS & TEAM_STANDINGS html pages
    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse("table.resultsarchive-table > tbody > tr")
        .map_err(|_| Error::Scraper)?;

    let table_body = document.select(&selector);

    let mut curr_standings: Vec<PositionInfo> = Vec::new();
    for element in table_body {
        let driver_details = results.parse_table_row(element)?;
        curr_standings.push(driver_details);
    }
    Ok(curr_standings)
}

fn parse_driver_table_row(element: ElementRef) -> Result<PositionInfo> {
    // Parsing based on current website layout, may need to modify parsing
    // if layout changes
    let td_selector = scraper::Selector::parse("td").map_err(|_| Error::Scraper)?;
    let mut iter = element.select(&td_selector);

    // skipping an empty <td>
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

    //skipping nationaliy & team
    iter.next();
    iter.next();

    // points
    let points = iter
        .next()
        .ok_or(Error::ParseDriverInfo)?
        .text()
        .collect::<Vec<_>>()[0]
        .parse::<usize>()?;

    Ok(PositionInfo {
        position,
        name,
        points,
    })
}

fn parse_team_table_row(element: ElementRef) -> Result<PositionInfo> {
    // Parsing based on current website layout, may need to modify parsing
    // if layout changes
    let td_selector = scraper::Selector::parse("td").map_err(|_| Error::Scraper)?;
    let mut iter = element.select(&td_selector);

    // skipping an empty <td>
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

    Ok(PositionInfo {
        position,
        name,
        points,
    })
}

pub enum GpResults {
    Driver,
    Team,
}

impl GpResults {
    fn parse_table_row(&self, element: ElementRef) -> Result<PositionInfo> {
        match self {
            Self::Driver => parse_driver_table_row(element),
            Self::Team => parse_team_table_row(element),
        }
    }

    fn fetch_internet_resource(&self) -> Result<String> {
        let url = match self {
            Self::Driver => {
                println!("Fetching Driver standings");
                DRIVER_STANDINGS
            }
            Self::Team => {
                println!("Fetching Team standings");
                TEAM_STANDINGS
            }
        };
        let data: String = ureq::get(url).call()?.into_string()?;
        Ok(data)
    }

    fn get_local_resource_loc(&self) -> &str {
        match self {
            GpResults::Driver => "2024_driver_standings.json",
            GpResults::Team => "2024_team_standings.json",
        }
    }

    pub fn get_standings(&self, force_save: bool) -> Result<Vec<PositionInfo>> {
        let tmp_dir = get_or_create_tmp_dir()?;
        let standings_file = tmp_dir.join(self.get_local_resource_loc());
        if !standings_file.exists() || force_save {
            let raw_data = self.fetch_internet_resource()?;
            let parsed_data = parse_standings_html_table(&raw_data, self)?;
            let json_data_to_cache = serde_json::to_string(&parsed_data)?;
            fs::write(standings_file, json_data_to_cache)?;
            Ok(parsed_data)
        } else {
            let data = fs::read_to_string(standings_file)?;
            let standings: Vec<PositionInfo> = serde_json::from_str(&data)?;
            Ok(standings)
        }
    }
}
