use std::path::Path;
use std::sync::LazyLock;

use scraper::ElementRef;

use super::{parse_standings_html_table, STANDINGS_BASE_URL};
use crate::error::{Error, Result};
use crate::utils::{DataFetcher, PositionInfo};
use crate::CURR_YEAR;

static TEAM_STANDINGS_FETCH_URL: LazyLock<String> =
    LazyLock::new(|| format!("{}/team.html", *CURR_YEAR));

fn parse_team_table_row(element: ElementRef) -> Result<PositionInfo> {
    // NOTE: Parsing based on current website layout, may need to modify parsing
    // if layout changes
    let td_selector = scraper::Selector::parse("td").map_err(|_| Error::Scraper)?;
    let mut iter = element.select(&td_selector);

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
    let name = name.first().ok_or_else(|| Error::ParseTeamInfo)?;
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

pub struct TeamStandings;

impl DataFetcher for TeamStandings {
    type A = Vec<PositionInfo>;

    fn cache_file_name() -> String {
        format!("{}_team_standings.json", *CURR_YEAR)
    }

    fn resource_url() -> String {
        println!("Fetching Team standings");
        format!("{}/{}", STANDINGS_BASE_URL, *TEAM_STANDINGS_FETCH_URL)
    }

    fn process_data(raw_data: String, _file_path: &Path) -> Result<Self::A> {
        parse_standings_html_table(&raw_data, &parse_team_table_row)
    }
}
