pub mod driver_standings;
pub mod team_standings;

use scraper::ElementRef;

use crate::error::{Error, Result};
use crate::utils::PositionInfo;

pub const BASE_URL: &str = "https://www.formula1.com/en/results.html";

fn parse_standings_html_table(
    html: &str,
    parse_row: &dyn Fn(ElementRef) -> Result<PositionInfo>,
) -> Result<Vec<PositionInfo>> {
    // Currently same table css selector can be used
    // for DRIVER_STANDINGS & TEAM_STANDINGS html pages
    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse("table.resultsarchive-table > tbody > tr")
        .map_err(|_| Error::Scraper)?;

    let table_body = document.select(&selector);

    let mut curr_standings: Vec<PositionInfo> = Vec::new();
    for element in table_body {
        let driver_details = parse_row(element)?;
        curr_standings.push(driver_details);
    }
    Ok(curr_standings)
}
