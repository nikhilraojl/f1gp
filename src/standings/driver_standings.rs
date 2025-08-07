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
        .ok_or(Error::ParseDriverInfo(1.0))?
        .text()
        .collect::<Vec<_>>();
    let position = position
        .get(0)
        .ok_or_else(|| Error::ParseDriverInfo(1.1))?
        .parse::<usize>()?;

    // driver name
    let driver_name = iter.next().ok_or(Error::ParseDriverInfo(2.0))?;

    let p_selector = scraper::Selector::parse("p").map_err(|_| Error::Scraper)?;
    let a_selector = scraper::Selector::parse("a").map_err(|_| Error::Scraper)?;
    let driver_span_selector = scraper::Selector::parse("span").map_err(|_| Error::Scraper)?;

    let p_driver_name = driver_name.select(&p_selector).next().unwrap();
    let a_driver_name = p_driver_name.select(&a_selector).next().unwrap();

    let mut span_iter = a_driver_name.select(&driver_span_selector);

    // skipping spans which are not required
    span_iter.next().unwrap();
    span_iter.next().unwrap();

    let first = span_iter
        .next()
        .ok_or(Error::ParseDriverInfo(3.0))?
        .text()
        .collect::<Vec<_>>();
    let first = first.get(0).ok_or_else(|| Error::ParseDriverInfo(3.1))?;

    let second = span_iter
        .next()
        .ok_or(Error::ParseDriverInfo(4.0))?
        .text()
        .collect::<Vec<_>>();
    let second = second.get(0).ok_or_else(|| Error::ParseDriverInfo(3.2))?;
    let name = format!("{} {}", first, second);

    //skipping nationaliy & team
    iter.next();
    iter.next();

    // points
    let points = iter
        .next()
        .ok_or(Error::ParseDriverInfo(5.0))?
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
