mod error;
mod results;
mod schedule;
mod standings;
mod utils;

use chrono::Local;

use error::Result;
use results::CompletedRace;
use schedule::Schedule;

use standings::driver_standings::DriverStandings;
use standings::team_standings::TeamStandings;

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
            "help" => {
                println!(
                    "{:<10}: Shows all Grand Prix Races for current calendar year",
                    "list"
                );
                println!("{:<10}: Shows session schedule of next Grand Prix. Also shows time until next session", "next");
                println!(
                    "{:<10}: Shows session schedule for next #num of Grand Prix Races",
                    "next <#>"
                );
                println!("{:<10}: Shows current driver standings", "drivers");
                println!("{:<10}: Shows current team/constructor standings", "teams");
                println!("{:<10}: Shows last Grand Prix race result", "result");
                println!(
                    "{:<10}: Shows results of the requested Grand Prix race(#round)",
                    "result <#>"
                );
                println!(
                    "{:<10}: Pull latest data from sources. Required for updated standings",
                    "pull"
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
