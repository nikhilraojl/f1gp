mod error;
mod results;
mod schedule;
mod standings;
mod utils;

use chrono::Local;
use std::fs::{read_dir, remove_file};

use error::{Error, Result};
use results::CompletedRace;
use schedule::Schedule;
use standings::driver_standings::DriverStandings;
use standings::team_standings::TeamStandings;
use utils::TMP_DIR_NAME;

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

                for (idx, race) in Schedule::race_schedule(false)?.iter().enumerate() {
                    race.pp_race_title(&mut output, curr_dt, idx + 1)?;
                }
                println!("{output}");
            }
            "next" => {
                let mut num_to_show: u8 = if let Some(arg) = args.next() {
                    arg.parse()?
                } else {
                    1
                };
                let mut output = String::new();

                for race in Schedule::race_schedule(false)? {
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
                let round_number: u8 = if let Some(arg) = args.next() {
                    arg.parse()?
                } else {
                    0
                };
                let mut output = String::new();

                if round_number == 0 {
                    for race in Schedule::race_schedule(false)? {
                        race.pp_race_schedule(&mut output)?;
                        output.push('\n');
                    }
                } else {
                    let schedule = Schedule::race_schedule(false)?;

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
                for driver in DriverStandings::standings(false)? {
                    println!("{:<20} {}", driver.name, driver.points)
                }
            }
            "teams" => {
                println!("TEAM STANDINGS:");
                println!("---------------");
                for team in TeamStandings::standings(false)? {
                    println!("{:<30} {}", team.name, team.points)
                }
            }
            "result" => {
                let mut output = String::new();
                let completed_gp = CompletedRace::get_completed_results(false)?;
                let round: usize = if let Some(arg) = args.next() {
                    arg.parse()?
                } else {
                    completed_gp.len()
                };

                if round > completed_gp.len() {
                    eprintln!("Round {} does not have any results", round);
                    return Ok(());
                }
                if let Some(race_result) = completed_gp.get(round - 1) {
                    race_result.pp_completed_race_results(&mut output)?;
                    println!("{output}");
                };
            }
            "pull" => {
                Schedule::race_schedule(true)?;
                TeamStandings::standings(true)?;
                DriverStandings::standings(true)?;
                CompletedRace::get_completed_results(true)?;
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
                println!(
                    "{:<16}: Pull latest data from sources. Required for updated standings",
                    "pull"
                );
                println!("{:<16}: Removes all cached files", "clean");
                println!("{:<16}: Shows files which will be deleted", "clean --dry-run");
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
