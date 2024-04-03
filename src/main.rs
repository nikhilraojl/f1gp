mod driver_standings;
mod error;
mod schedule;

use chrono::Local;

use driver_standings::driver_standings;
use error::Result;
use schedule::race_schedule;

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
        } else if arg == "standings" {
            for driver in driver_standings()? {
                println!("{:<20} {}", driver.name, driver.points)
            }
        }
    }
    Ok(())
    // println!("{}", now.elapsed().as_millis());
}
