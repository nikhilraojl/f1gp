use scraper::Selector;
use serde::{Deserialize, Serialize};
use std::fs;
use ureq;

use crate::error::{Error, Result};
use crate::utils::{get_or_create_tmp_dir, PositionInfo};

const BASE_URL: &str = "https://www.formula1.com";
const RACE_RESULTS: &str = "en/results.html/2024/races.html";

#[derive(Debug, Deserialize, Serialize)]
pub struct CompletedRace {
    pub round: usize,
    pub gp_name: String,
    pub results: Vec<PositionInfo>,
}

fn fetch_data(url: &str) -> Result<String> {
    println!("Fetching data");
    let body: String = ureq::get(&url).call()?.into_string()?;
    Ok(body)
}

struct CssSelectors {
    table_selector: Selector,
    td_selector: Selector,
    span_selector: Selector,
}

fn parse_all_results_page(html: String) -> Result<Vec<CompletedRace>> {
    let document = scraper::Html::parse_document(&html);
    // constructing all selectors
    let table_selector = scraper::Selector::parse("table.resultsarchive-table > tbody > tr")
        .map_err(|_| Error::Scraper)?;
    let anchor_selector = scraper::Selector::parse("a").map_err(|_| Error::Scraper)?;
    let td_selector = scraper::Selector::parse("td").map_err(|_| Error::Scraper)?;
    let span_selector = scraper::Selector::parse("span").map_err(|_| Error::Scraper)?;

    let sltrs = CssSelectors {
        table_selector,
        td_selector,
        span_selector,
    };

    let table_body = document.select(&sltrs.table_selector);

    let mut all_results: Vec<CompletedRace> = Vec::new();
    for (idx, element) in table_body.enumerate() {
        let mut iter = element.select(&sltrs.td_selector);
        // useless
        iter.next();

        let td_link = iter.next().ok_or_else(|| Error::ParseTeamInfo)?;
        let link = td_link
            .select(&anchor_selector)
            .next()
            .ok_or_else(|| Error::ParseTeamInfo)?
            .value()
            .attr("href")
            .ok_or_else(|| Error::ParseTeamInfo)?;
        let gp_name = td_link.text().collect::<Vec<_>>()[1].trim().to_owned();
        let results = fetch_parse_individual_race(link, &sltrs)?;
        all_results.push(CompletedRace {
            round: idx + 1,
            gp_name,
            results,
        })
    }

    Ok(all_results)
}

fn fetch_parse_individual_race(href: &str, selectors: &CssSelectors) -> Result<Vec<PositionInfo>> {
    println!("Fetching data for race {}", href);
    let body = fetch_data(&format!("{}/{}", BASE_URL, href))?;
    let document = scraper::Html::parse_document(&body);
    let table_body = document.select(&selectors.table_selector);

    let mut race_result: Vec<PositionInfo> = Vec::new();
    for element in table_body {
        let mut iter = element.select(&selectors.td_selector);
        // useless
        iter.next();

        let position = iter
            .next()
            .ok_or_else(|| Error::ParseTeamInfo)?
            .text()
            .collect::<Vec<_>>()[0]
            .parse::<usize>()
            .or_else(|_| Ok::<usize, Error>(0))?;

        // useless
        iter.next();

        let driver_name = iter.next().ok_or_else(|| Error::ParseTeamInfo)?;
        let mut span_iter = driver_name.select(&selectors.span_selector);
        let first = span_iter
            .next()
            .ok_or_else(|| Error::ParseTeamInfo)?
            .text()
            .collect::<Vec<_>>()[0];
        let second = span_iter
            .next()
            .ok_or_else(|| Error::ParseTeamInfo)?
            .text()
            .collect::<Vec<_>>()[0];
        let name = format!("{} {}", first, second);

        // useless
        iter.next();
        iter.next();
        iter.next();

        // points
        let points = iter
            .next()
            .ok_or(Error::ParseDriverInfo)?
            .text()
            .collect::<Vec<_>>()[0]
            .parse::<usize>()?;

        let res = PositionInfo {
            position,
            name,
            points,
        };
        race_result.push(res);
    }

    Ok(race_result)
}

pub fn get_completed_results(force_save: bool) -> Result<Vec<CompletedRace>> {
    let tmp_dir = get_or_create_tmp_dir()?;
    let results_file = tmp_dir.join("2024_race_results.json");
    if !results_file.exists() || force_save {
        let url = format!("{}/{}", BASE_URL, RACE_RESULTS);
        let raw_data = fetch_data(&url)?;
        let parsed_data = parse_all_results_page(raw_data)?;
        let json_data_to_cache = serde_json::to_string(&parsed_data)?;
        fs::write(results_file, json_data_to_cache)?;
        Ok(parsed_data)
    } else {
        let data = fs::read_to_string(results_file)?;
        let standings: Vec<CompletedRace> = serde_json::from_str(&data)?;
        Ok(standings)
    }
}
