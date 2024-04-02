mod race;

use std::fs::read_to_string;

use chrono::Local;
use race::{NormalWeekend, Races, SprintWeekend, Weekend, GP};
use std::fmt::Write;

fn main() {
    let mut args = std::env::args();

    // skipping command arg
    args.next();

    if let Some(arg) = args.next() {
        if arg == "next" {
            let now = std::time::Instant::now();
            let data = read_to_string("2024_schedule.json").unwrap();
            // let data = read_to_string("small.json").unwrap();
            let races: Races = serde_json::from_str(&data).unwrap();
            let dt = Local::now();
            let mut output = String::new();
            for race in races.races {
                match race.sessions {
                    race::Weekend::Normal(ref session) => {
                        if dt < session.gp {
                            pp_race_name(&mut output, &race);
                            pp_normal(&mut output, session);
                            break;
                        }
                    }
                    race::Weekend::Sprint(ref session) => {
                        if dt < session.gp {
                            pp_race_name(&mut output, &race);
                            pp_sprint(&mut output, &session);
                            break;
                        }
                    }
                }
            }
            println!("{output}");
            // println!("{}", now.elapsed().as_millis());
        }
    }
}

fn pp_race_name(output: &mut String, race: &GP) {
    writeln!(output, "+{}+", "-".repeat(30)).unwrap();
    writeln!(
        output,
        "| {} Grand Prix / {} |",
        race.name, race.location
    )
    .unwrap();
    writeln!(output, "+{}+", "-".repeat(30)).unwrap();
}

const STR_FMT: &'static str = "%a %d/%m/%Y %H:%M";

fn pp_normal(output: &mut String, session: &NormalWeekend) {
    writeln!(output, "FP 1 : {}", session.fp1.format(STR_FMT)).unwrap();
    writeln!(output, "FP 2 : {}", session.fp2.format(STR_FMT)).unwrap();
    writeln!(output, "FP 3 : {}", session.fp3.format(STR_FMT)).unwrap();
    writeln!(output, "Quali: {}", session.qualifying.format(STR_FMT)).unwrap();
    writeln!(output, "Race : {}", session.gp.format(STR_FMT)).unwrap();
}

fn pp_sprint(output: &mut String, session: &SprintWeekend) {
    writeln!(output, "FP 1        : {}", session.fp1.format(STR_FMT)).unwrap();
    writeln!(
        output,
        "Sprint Quali: {}",
        session.sprintQualifying.format(STR_FMT)
    )
    .unwrap();
    writeln!(output, "Sprint      : {}", session.sprint.format(STR_FMT)).unwrap();
    writeln!(
        output,
        "Quali       : {}",
        session.qualifying.format(STR_FMT)
    )
    .unwrap();
    writeln!(output, "Race        : {}", session.gp.format(STR_FMT)).unwrap();
}
