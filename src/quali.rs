use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use ureq;

use crate::error::{Error, Result};
use crate::utils::{DataFetcher, QualiPositionInfo, F1_TABLE_SELECTOR};

const BASE_URL: &str = "https://www.formula1.com";
const CALENDAR_RACE_RESULTS: &str = "en/results/2024/races";
const SUB_URL_PER_RACE_RESULT: &str = "en/results/2024";

#[derive(Debug, Deserialize, Serialize)]
pub struct CompletedQualifying {
    round: usize,
    gp_name: String,
    results: Vec<QualiPositionInfo>,
}

fn fetch_data(url: &str) -> Result<String> {
    let body: String = ureq::get(url).call()?.into_string()?;
    Ok(body)
}

pub fn parse_qualifying_page(
    html: String,
    existing_round_results: Vec<usize>,
) -> Result<Vec<CompletedQualifying>> {
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
        let link = td_link
            .select(&anchor_selector)
            .next()
            .ok_or_else(|| Error::ParseRaceResults)?
            .value()
            .attr("href")
            .ok_or_else(|| Error::ParseRaceResults)?
            .to_owned();
        let gp_name = td_link.text().collect::<Vec<_>>()[0].trim().to_owned();
        let race_url = format!("{}/{}/{}", BASE_URL, SUB_URL_PER_RACE_RESULT, link);
        let quali_url = race_url.replace("race-result", "qualifying");

        let handle = std::thread::spawn(move || {
            println!("Fetching Qualifying data from {}", &quali_url);
            let body = fetch_data(&quali_url)?;
            let quali_result = fetch_parse_individual_quali_result(body)?;
            let mut guraded_data = output_arc_clone
                .lock()
                .map_err(|_| Error::ParseRaceResults)?;
            guraded_data.push(CompletedQualifying {
                round: idx + 1,
                gp_name,
                results: quali_result,
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

fn fetch_parse_individual_quali_result(document: String) -> Result<Vec<QualiPositionInfo>> {
    let document = scraper::Html::parse_document(&document);
    let f1_table_selector = scraper::Selector::parse(F1_TABLE_SELECTOR).unwrap();

    let td_selector = scraper::Selector::parse("td").unwrap();

    let doc_iter = document.select(&f1_table_selector);

    let mut output = Vec::new();

    for element in doc_iter {
        let mut element_iter = element.select(&td_selector);

        let position = element_iter.next().unwrap().text().collect::<Vec<_>>()[0]
            .parse::<usize>()
            .unwrap_or(0);

        // skip car number
        element_iter.next();

        let full_name = element_iter.next().unwrap().text().collect::<Vec<_>>();
        let mut name = full_name[0].to_owned();
        name.push(' ');
        name.push_str(full_name[2]);

        // skip team name
        element_iter.next();

        let q = element_iter.next().unwrap().text().collect::<Vec<_>>();
        let q1 = q.get(0).map(|s| s.to_owned().to_owned());

        let q = element_iter.next().unwrap().text().collect::<Vec<_>>();
        let q2 = q.get(0).map(|s| s.to_owned().to_owned());

        let q = element_iter.next().unwrap().text().collect::<Vec<_>>();
        let q3 = q.get(0).map(|s| s.to_owned().to_owned());

        let quali_result = QualiPositionInfo {
            position,
            name,
            q1,
            q2,
            q3,
        };
        output.push(quali_result);
    }
    Ok(output)
}

impl DataFetcher for CompletedQualifying {
    type A = Vec<CompletedQualifying>;

    fn cache_file_name() -> String {
        "2024_quali_results.json".to_owned()
    }

    fn resource_url() -> String {
        println!("Fetching data for all completed Qualifying Prix");
        format!("{}/{}", BASE_URL, CALENDAR_RACE_RESULTS)
    }

    fn process_data(raw_data: String, file_path: &Path) -> Result<Self::A> {
        let mut all_results: Vec<CompletedQualifying> = if file_path.exists() {
            Self::read_from_cache(file_path).unwrap_or(Vec::new())
        } else {
            Vec::new()
        };
        let rounds_cached = all_results.iter().map(|r| r.round).collect::<Vec<usize>>();
        all_results.extend(parse_qualifying_page(raw_data, rounds_cached)?);
        Ok(all_results)
    }
}

impl CompletedQualifying {
    pub fn get_completed_quali_results(force_save: bool) -> Result<Vec<CompletedQualifying>> {
        let results = Self::get_data(force_save)?;
        if !results.is_empty() {
            Ok(results)
        } else {
            Err(Error::NoResults)
        }
    }

    pub fn pp_completed_quali_results(&self, output: &mut String) -> Result<()> {
        writeln!(output, "{}", "-".repeat(self.gp_name.len()))?;
        writeln!(output, "{}", self.gp_name)?;
        writeln!(output, "{}", "-".repeat(self.gp_name.len()))?;
        writeln!(
            output,
            "{:<3} {:<20} {:^8} | {:^8} | {:^8}",
            "", "", "Q1", "Q2", "Q3"
        )?;
        for driver in &self.results {
            let d_q1 = driver.q1.clone().unwrap_or("".to_owned());
            let d_q2 = driver.q2.clone().unwrap_or("".to_owned());
            let d_q3 = driver.q3.clone().unwrap_or("".to_owned());
            writeln!(
                output,
                "{:<3} {:<20} {:<8} | {:<8} | {:<8}",
                driver.position, driver.name, d_q1, d_q2, d_q3
            )?;
        }
        Ok(())
    }
}
