use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use crate::error::{Error, Result};
use crate::utils::{DataFetcher, PositionInfo, F1_TABLE_SELECTOR};
use crate::CURR_YEAR;

const BASE_URL: &str = "https://www.formula1.com";

fn fetch_data(url: &str) -> Result<String> {
    let body: String = ureq::get(url).call()?.into_string()?;
    Ok(body)
}

fn parse_all_results_page(
    html: String,
    existing_round_results: Vec<usize>,
) -> Result<Vec<CompletedRace>> {
    let document = scraper::Html::parse_document(&html);

    // constructing all selectors
    let table_selector = scraper::Selector::parse(F1_TABLE_SELECTOR).map_err(|_| Error::Scraper)?;
    let anchor_selector = scraper::Selector::parse("a").map_err(|_| Error::Scraper)?;
    let td_selector = scraper::Selector::parse("td").map_err(|_| Error::Scraper)?;

    let table_body = document.select(&table_selector);

    let mut join_handles: Vec<JoinHandle<Result<()>>> = Vec::new();
    let output_data = Arc::new(Mutex::new(Vec::new()));

    for (idx, element) in table_body.enumerate() {
        let output_arc_clone = output_data.clone();
        if existing_round_results.contains(&(idx + 1)) {
            // why? We don't want to refetch results for Grand Prix already in cache
            // If cached data is corrupted do f1gp clean and f1gp pull or
            // remove cached file in `tmp` location.
            continue;
        }

        let mut iter = element.select(&td_selector);

        let td_link = iter.next().ok_or_else(|| Error::ParseRaceResults)?;
        let a_td_link = td_link
            .select(&anchor_selector)
            .next()
            .ok_or_else(|| Error::ParseRaceResults)?;

        let gp_name = a_td_link.text().collect::<Vec<_>>()[1].to_owned();

        let link = a_td_link
            .value()
            .attr("href")
            .ok_or_else(|| Error::ParseRaceResults)?
            .to_owned();


        let race_url = format!("{}/{}", BASE_URL, link);

        let handle = std::thread::spawn(move || {
            println!("Fetching Grand Prix data from {}", &race_url);
            let body = fetch_data(&race_url)?;
            let gp_result = fetch_parse_individual_race(body)?;
            let mut guraded_data = output_arc_clone
                .lock()
                .map_err(|_| Error::ParseRaceResults)?;
            guraded_data.push(CompletedRace {
                round: idx + 1,
                gp_name,
                results: gp_result,
            });
            Ok(())
        });
        join_handles.push(handle);
    }

    for h in join_handles.into_iter() {
        let _ = h.join().map_err(|_| Error::ParseRaceResults)?;
    }

    let lock = Arc::into_inner(output_data).ok_or(Error::ParseRaceResults)?;
    let mut output_data = lock.into_inner().map_err(|_| Error::ParseRaceResults)?;
    output_data.sort_by(|a, b| a.round.cmp(&b.round));

    Ok(output_data)
}

fn fetch_parse_individual_race(body: String) -> Result<Vec<PositionInfo>> {
    // constructing all selectors
    let td_selector = scraper::Selector::parse("td").map_err(|_| Error::Scraper)?;
    let span_selector = scraper::Selector::parse("span").map_err(|_| Error::Scraper)?;
    let p_selector = scraper::Selector::parse("p").map_err(|_| Error::Scraper)?;
    let driver_span_selector = scraper::Selector::parse("span.test").map_err(|_| Error::Scraper)?;
    let table_selector = scraper::Selector::parse(F1_TABLE_SELECTOR).map_err(|_| Error::Scraper)?;

    let document = scraper::Html::parse_document(&body);
    let table_body = document.select(&table_selector);

    let mut race_result: Vec<PositionInfo> = Vec::new();
    for element in table_body {
        let mut iter = element.select(&td_selector);

        let position = iter
            .next()
            .ok_or_else(|| Error::ParseRaceResults)?
            .text()
            .collect::<Vec<_>>()[0]
            .parse::<usize>()
            .or_else(|_| Ok::<usize, Error>(0))?;

        // driver number, useless
        iter.next();

        // name of the driver
        let driver_name = iter.next().ok_or_else(|| Error::ParseRaceResults)?;

        let p_driver_name = driver_name.select(&p_selector).next().unwrap();
        let span_driver_name = p_driver_name.select(&driver_span_selector).next().unwrap();

        let mut span_iter = span_driver_name.select(&span_selector);
        let first = span_iter
            .next()
            .ok_or_else(|| Error::ParseRaceResults)?
            .text()
            .collect::<Vec<_>>()[0];
        let second = span_iter
            .next()
            .ok_or_else(|| Error::ParseRaceResults)?
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
            .ok_or(Error::ParseRaceResults)?
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

#[derive(Debug, Deserialize, Serialize)]
pub struct CompletedRace {
    round: usize,
    gp_name: String,
    results: Vec<PositionInfo>,
}

impl DataFetcher for CompletedRace {
    type A = Vec<CompletedRace>;

    fn cache_file_name() -> String {
        format!("{}_race_results.json", *CURR_YEAR)
    }

    fn resource_url() -> String {
        println!("Fetching data for all completed Grand Prix");
        let calendar_race_results = format!("en/results/{}/races", *CURR_YEAR);
        format!("{}/{}", BASE_URL, calendar_race_results) 
    }

    fn process_data(raw_data: String, file_path: &Path) -> Result<Self::A> {
        let mut all_results: Vec<CompletedRace> = if file_path.exists() {
            Self::read_from_cache(file_path).unwrap_or(Vec::new())
        } else {
            Vec::new()
        };
        let rounds_cached = all_results.iter().map(|r| r.round).collect::<Vec<usize>>();
        all_results.extend(parse_all_results_page(raw_data, rounds_cached)?);
        Ok(all_results)
    }
}

impl CompletedRace {
    pub fn pp_completed_race_results(&self, output: &mut String) -> Result<()> {
        writeln!(output, "{}", "-".repeat(self.gp_name.len()))?;
        writeln!(output, "{}", self.gp_name)?;
        writeln!(output, "{}", "-".repeat(self.gp_name.len()))?;
        for driver in &self.results {
            writeln!(
                output,
                "{:<3} {:<20} {}",
                driver.position, driver.name, driver.points
            )?;
        }
        Ok(())
    }
}
