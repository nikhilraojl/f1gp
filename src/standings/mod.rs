pub mod driver_standings;
pub mod team_standings;

use scraper::ElementRef;

use crate::error::{Error, Result};
use crate::utils::{PositionInfo, F1_TABLE_SELECTOR};

pub const STANDINGS_BASE_URL: &str = "https://www.formula1.com/en/results.html";

fn parse_standings_html_table(
    html: &str,
    parse_row: &dyn Fn(ElementRef) -> Result<PositionInfo>,
) -> Result<Vec<PositionInfo>> {
    // Currently same table css selector can be used
    // for DRIVER_STANDINGS & TEAM_STANDINGS html pages
    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse(F1_TABLE_SELECTOR).map_err(|_| Error::Scraper)?;

    let table_row = document.select(&selector);

    let mut standings: Vec<PositionInfo> = Vec::new();
    for element in table_row {
        let parsed_row = parse_row(element)?;
        standings.push(parsed_row);
    }
    Ok(standings)
}
