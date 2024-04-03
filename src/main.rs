mod error;
mod race;

use chrono::Local;
use std::fs::read_to_string;

use error::Result;
use race::Races;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
    }
}

fn run() -> Result<()> {
    // let now = std::time::Instant::now();
    let mut args = std::env::args();

    // skipping command arg
    args.next();

    if let Some(arg) = args.next() {
        if arg == "next" {
            // let data = read_to_string("2024_schedule.json")?;
            let data = read_to_string("small.json")?;
            let races: Races = serde_json::from_str(&data)?;
            let curr_dt = Local::now();
            let mut output = String::new();
            for race in races.races {
                if curr_dt < race.gp_start_dt() {
                    race.pp_race_name(&mut output)?;
                    break;
                }
            }
            println!("{output}");
        }
    }
    Ok(())
    // println!("{}", now.elapsed().as_millis());
}
