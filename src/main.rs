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
        let curr_dt = Local::now();
        match arg.as_ref() {
            "list" => {
                let mut output = String::new();

                for race in race_schedule(false)?.races {
                    race.pp_race_title(&mut output, curr_dt)?;
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

                for race in race_schedule(false)?.races {
                    if curr_dt < race.gp_start_dt() {
                        race.pp_race_schedule(&mut output)?;
                        num_to_show -= 1;
                        if num_to_show == 0 {
                            break;
                        }
                    }
                }
                println!("{output}");
            }
            "drivers" => {
                for driver in driver_standings(false)? {
                    println!("{:<20} {}", driver.name, driver.points)
                }
            }
            "pull" => {
                race_schedule(true)?;
                driver_standings(true)?;
            }
            _ => {
                eprintln!("Not a valid command")
            }
        }
    }
    Ok(())
    // println!("{}", now.elapsed().as_millis());
}
