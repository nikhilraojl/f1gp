mod error;
mod quali;
mod results;
mod sprint_results;
mod schedule;
mod standings;
mod utils;

use chrono::{Datelike, Local};
use sprint_results::CompletedSprint;
use std::cmp::max;
use std::fs::{read_dir, remove_file};
use std::sync::LazyLock;

use error::{Error, Result};
use quali::CompletedQualifying;
use results::CompletedRace;
use schedule::Schedule;
use standings::driver_standings::DriverStandings;
use standings::team_standings::TeamStandings;
use utils::{DataFetcher, TMP_DIR_NAME};

static CURR_YEAR: LazyLock<i32> = LazyLock::new(|| Local::now().year());

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
    }
}

fn run() -> Result<()> {
    // let now = std::time::Instant::now();
    let mut args = std::env::args();

    // skipping first default arg
    args.next();

    if let Some(arg) = args.next() {
        let curr_dt = Local::now();
        match arg.as_ref() {
            "list" => {
                let mut output = String::new();
                let info = "[x] Completed || [-> This weekend || [ ] Pending || SP Sprint Weekend";

                let mut bottom_border_len: usize = info.len();
                for race in Schedule::get_data()?.iter() {
                    let race_title = race.pp_race_title(curr_dt);
                    bottom_border_len = max(bottom_border_len, race_title.len());
                    output.push_str(&race_title);
                    output.push('\n');
                }
                output.push_str(&"-".repeat(bottom_border_len));
                output.push('\n');
                output.push_str(info);
                output.push('\n');
                println!("{output}");
            }
            "next" => {
                let mut num_to_show = args.next().unwrap_or("1".to_owned()).parse::<u8>()?;
                let mut output = String::new();

                for race in Schedule::get_data()? {
                    if curr_dt < race.gp_start_dt() {
                        race.pp_race_schedule(&mut output)?;
                        num_to_show -= 1;
                        if num_to_show == 0 {
                            break;
                        }
                    }
                }
                if output.is_empty() {
                    eprintln!("No more Grand Prix races scheduled");
                } else {
                    println!("{output}");
                }
            }
            "schedule" => {
                let round_number = args.next().unwrap_or("0".to_owned()).parse::<u8>()?;
                let mut output = String::new();

                if round_number == 0 {
                    for race in Schedule::get_data()? {
                        race.pp_race_schedule(&mut output)?;
                        output.push('\n');
                    }
                } else {
                    let schedule = Schedule::get_data()?;

                    let gp_race = schedule
                        .get(round_number as usize - 1)
                        .ok_or(Error::InvalidArgs)?;
                    gp_race.pp_race_schedule(&mut output)?;
                }

                if output.is_empty() {
                    eprintln!("No more Grand Prix races scheduled");
                } else {
                    println!("{output}");
                }
            }
            "drivers" => {
                println!("DRIVER STANDINGS:");
                println!("-----------------");
                for driver in DriverStandings::get_data()? {
                    println!("{:<20} {}", driver.name, driver.points)
                }
            }
            "teams" => {
                println!("TEAM STANDINGS:");
                println!("---------------");
                for team in TeamStandings::get_data()? {
                    println!("{:<30} {}", team.name, team.points)
                }
            }
            "quali" => {
                let mut output = String::new();
                let completed_quali = CompletedQualifying::get_data()?;
                let round: usize = if let Some(arg) = args.next() {
                    arg.parse()?
                } else {
                    completed_quali.len()
                };

                if round < 1 || round > 25 {
                    eprintln!("Invalid round value given {}", round);
                    return Ok(());
                }
                if round > completed_quali.len() {
                    eprintln!("Round {} does not have any quali results", round);
                    return Ok(());
                }

                if let Some(race_result) = completed_quali.get(round - 1) {
                    race_result.pp_completed_quali_results(&mut output)?;
                    println!("{output}");
                };
            }
            "result" => {
                let mut output = String::new();
                let completed_gp = CompletedRace::get_data()?;
                let round: usize = if let Some(arg) = args.next() {
                    arg.parse()?
                } else {
                    completed_gp.len()
                };

                if round < 1 || round > 25 {
                    eprintln!("Invalid round value given {}", round);
                    return Ok(());
                }
                if round > completed_gp.len() {
                    eprintln!("Round {} does not have any results", round);
                    return Ok(());
                }
                if let Some(race_result) = completed_gp.get(round - 1) {
                    race_result.pp_completed_race_results(&mut output)?;
                    println!("{output}");
                };
            }
            "sprint" => {
                let _completed_sprint = CompletedSprint::get_data();
                println!("Sprint weekend fetching done");
            }
            "pull" => {
                Schedule::pull()?;
                // TeamStandings::pull()?;
                // DriverStandings::pull()?;
                // CompletedRace::pull()?;
                // CompletedQualifying::pull()?;
            }
            "clean" => {
                let dry_run = match args.next() {
                    Some(arg) => arg.as_str() == "--dry-run",
                    None => false,
                };
                if dry_run {
                    println!("DRY RUN:");
                    println!("--------");
                }
                let tmp_dir = std::env::temp_dir().join(TMP_DIR_NAME);
                println!("{:?}", tmp_dir);
                for entry in read_dir(tmp_dir)? {
                    let entry = entry?;
                    println!("Removing {:?}", entry.file_name());
                    if !dry_run {
                        remove_file(entry.path())?;
                    }
                }
            }
            "help" => {
                println!(
                    "{:<16}: Shows all Grand Prix Races for current calendar year",
                    "list"
                );
                println!("{:<16}: Shows session schedule of next Grand Prix. Also shows time until next session", "next");
                println!(
                    "{:<16}: Shows session schedule for next #num of Grand Prix Races",
                    "next <#>"
                );
                println!("{:<16}: Shows current driver standings", "drivers");
                println!("{:<16}: Shows current team/constructor standings", "teams");
                println!("{:<16}: Shows last Grand Prix race result", "result");
                println!(
                    "{:<16}: Shows results of the requested Grand Prix race(#round)",
                    "result <#>"
                );
                println!("{:<16}: Shows last Grand Prix qualifying result", "quali");
                println!(
                    "{:<16}: Shows qualifying results of the requested Grand Prix(#round)",
                    "quali <#>"
                );
                println!(
                    "{:<16}: Pull latest data from sources. Required for updated standings",
                    "pull"
                );
                println!("{:<16}: Removes all cached files", "clean");
                println!(
                    "{:<16}: Shows files which will be deleted",
                    "clean --dry-run"
                );
            }
            _ => {
                eprintln!("Not a valid command. Run `f1gp help` for possible commands")
            }
        }
    } else {
        eprintln!("Not a valid command. Run `f1gp help` for possible commands")
    }
    // println!("{}", now.elapsed().as_millis());
    Ok(())
}
