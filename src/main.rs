mod error;
mod race;

use chrono::Local;
use std::fs;

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

    // skipping first default arg
    args.next();

    if let Some(arg) = args.next() {
        if arg == "next" {
            let races = race_schedule()?;
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

fn race_schedule() -> Result<Races> {
    let schedule_dir = std::env::temp_dir().join("f1_2024_schedule");
    if !schedule_dir.exists() {
        std::fs::create_dir(&schedule_dir)?;
    }
    let schedule_file = schedule_dir.join("2024_schedule.json");
    let races = if schedule_file.exists() {
        let data = fs::read_to_string(schedule_file)?;
        let races: Races = serde_json::from_str(&data)?;
        races
    } else {
        let raw_data = get_data_from_github()?;
        fs::write(schedule_file, &raw_data)?;
        let races: Races = serde_json::from_str(&raw_data)?;
        races
    };

    Ok(races)
}

fn get_data_from_github() -> Result<String> {
    let body: String =
        ureq::get("https://raw.githubusercontent.com/sportstimes/f1/main/_db/f1/2024.json")
            .call()?
            .into_string()?;
    Ok(body)
}
