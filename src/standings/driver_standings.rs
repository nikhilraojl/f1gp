use std::path::Path;
use std::sync::LazyLock;

use scraper::{selectable::Selectable, ElementRef};

use super::{parse_standings_html_table, STANDINGS_BASE_URL};
use crate::error::{Error, Result};
use crate::utils::{DataFetcher, PositionInfo};
use crate::CURR_YEAR;

static DRIVER_STANDINGS_FETCH_URL: LazyLock<String> =
    LazyLock::new(|| format!("{}/drivers.html", *CURR_YEAR));

fn parse_driver_table_row(element: ElementRef) -> Result<PositionInfo> {
    // NOTE: Parsing based on current website layout, may need to modify parsing
    // if layout changes
    let td_selector = scraper::Selector::parse("td").map_err(|_| Error::Scraper)?;
    let mut iter = element.select(&td_selector);

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

pub struct DriverStandings;

impl DataFetcher for DriverStandings {
    type A = Vec<PositionInfo>;

    fn cache_file_name() -> String {
        format!("{}_driver_standings.json", *CURR_YEAR)
    }

    fn resource_url() -> String {
        println!("Fetching Driver standings");
        format!("{}/{}", STANDINGS_BASE_URL, *DRIVER_STANDINGS_FETCH_URL)
    }

    fn process_data(raw_data: String, _file_path: &Path) -> Result<Self::A> {
        parse_standings_html_table(&raw_data, &parse_driver_table_row)
    }
}
