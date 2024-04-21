use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::path::Path;

use crate::error::Result;
use crate::utils::DataFetcher;

// for date time formatting
pub const STR_FMT: &str = "%a %d/%m/%Y %H:%M";

// internet resources
pub const SCHEDULE: &str = "https://raw.githubusercontent.com/sportstimes/f1/main/_db/f1/2024.json";

// str constants
pub const FP1: &str = "FP 1";
pub const FP2: &str = "FP 2";
pub const FP3: &str = "FP 2";
pub const QUALI: &str = "Quali";
pub const SPR_QUALI: &str = "Spr Quali";
pub const SPRINT: &str = "Sprint";
pub const RACE: &str = "Race";

fn pp_session(
    session_name: &str,
    session_dt: DateTime<Local>,
    curr_dt: DateTime<Local>,
    pad_session: usize,
    line_width: usize,
) -> String {
    let is_past = if curr_dt > session_dt { "x" } else { " " };

    format!(
        "| {:<line_width$} |",
        format!(
            "[{}] {:<pad_session$}: {}",
            is_past,
            session_name,
            session_dt.format(STR_FMT)
        )
    )
}

fn time_until_next_session(sessions: [&DateTime<Local>; 5], curr_dt: DateTime<Local>) -> String {
    let mut next_session: Option<DateTime<Local>> = None;
    for session_dt in sessions {
        if curr_dt < *session_dt {
            next_session = Some(*session_dt);
            break;
        }
    }

    let mut output = String::new();
    if let Some(dt) = next_session {
        let y = dt - curr_dt;
        output = format!(
            "Next session in: {} days, {} hours, {} minutes",
            y.num_days(),
            y.num_hours() % 24,
            y.num_minutes() % 60
        );
    }
    output
}

#[derive(Deserialize, Serialize, Debug)]
struct NormalWeekend {
    fp1: DateTime<Local>,
    fp2: DateTime<Local>,
    fp3: DateTime<Local>,
    qualifying: DateTime<Local>,
    gp: DateTime<Local>,
}
impl NormalWeekend {
    pub fn pp_normal(&self, output: &mut String, line_width: usize) -> Result<()> {
        let session_width = 5;
        let curr_dt = Local::now();
        writeln!(
            output,
            "{}",
            pp_session(FP1, self.fp1, curr_dt, session_width, line_width)
        )?;
        writeln!(
            output,
            "{}",
            pp_session(FP2, self.fp2, curr_dt, session_width, line_width)
        )?;
        writeln!(
            output,
            "{}",
            pp_session(FP3, self.fp3, curr_dt, session_width, line_width)
        )?;
        writeln!(
            output,
            "{}",
            pp_session(QUALI, self.qualifying, curr_dt, session_width, line_width)
        )?;
        writeln!(
            output,
            "{}",
            pp_session(RACE, self.gp, curr_dt, session_width, line_width)
        )?;
        Ok(())
    }

    fn pp_until_next_normal(&self) -> String {
        let curr_dt = Local::now();
        let sessions = [&self.fp1, &self.fp2, &self.fp3, &self.qualifying, &self.gp];
        time_until_next_session(sessions, curr_dt)
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
struct SprintWeekend {
    fp1: DateTime<Local>,
    sprintQualifying: DateTime<Local>,
    sprint: DateTime<Local>,
    qualifying: DateTime<Local>,
    gp: DateTime<Local>,
}
impl SprintWeekend {
    fn pp_sprint(&self, output: &mut String, line_width: usize) -> Result<()> {
        let session_width = 9;
        let curr_dt = Local::now();
        writeln!(
            output,
            "{}",
            pp_session(FP1, self.fp1, curr_dt, session_width, line_width)
        )?;
        writeln!(
            output,
            "{}",
            pp_session(
                SPR_QUALI,
                self.sprintQualifying,
                curr_dt,
                session_width,
                line_width
            )
        )?;
        writeln!(
            output,
            "{}",
            pp_session(SPRINT, self.sprint, curr_dt, session_width, line_width)
        )?;
        writeln!(
            output,
            "{}",
            pp_session(QUALI, self.qualifying, curr_dt, session_width, line_width)
        )?;
        writeln!(
            output,
            "{}",
            pp_session(RACE, self.gp, curr_dt, session_width, line_width)
        )?;
        Ok(())
    }

    fn pp_until_next_sprint(&self) -> String {
        let curr_dt = Local::now();
        let sessions = [
            &self.fp1,
            &self.sprintQualifying,
            &self.sprint,
            &self.qualifying,
            &self.gp,
        ];
        time_until_next_session(sessions, curr_dt)
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum Sessions {
    Normal(NormalWeekend),
    Sprint(SprintWeekend),
}

impl Sessions {
    pub fn gp_start_dt(&self) -> DateTime<Local> {
        match self {
            Self::Normal(sessions) => sessions.gp,
            Self::Sprint(sessions) => sessions.gp,
        }
    }
    fn pp_time_until_next_session(&self) -> String {
        match self {
            Self::Normal(sessions) => sessions.pp_until_next_normal(),
            Self::Sprint(sessions) => sessions.pp_until_next_sprint(),
        }
    }

    fn pp_session(&self, output: &mut String, line_width: usize) -> Result<()> {
        match self {
            Self::Normal(ref session) => session.pp_normal(output, line_width),
            Self::Sprint(ref session) => session.pp_sprint(output, line_width),
        }?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GrandPrix {
    name: String,
    location: String,
    sessions: Sessions,
}
impl GrandPrix {
    pub fn pp_race_title(
        &self,
        output: &mut String,
        curr_dt: DateTime<Local>,
        round: usize,
    ) -> Result<()> {
        let race_name = format!("{} Grand Prix / {}", self.name, self.location);
        let is_past = if curr_dt > self.sessions.gp_start_dt() {
            "[x]"
        } else if (self.sessions.gp_start_dt() - curr_dt).num_days() < 7 {
            "[->"
        } else {
            "[ ]"
        };
        writeln!(output, "{}  {:>2}. {}", is_past, round, race_name)?;
        Ok(())
    }

    pub fn pp_race_schedule(&self, output: &mut String) -> Result<()> {
        let race_name = format!("{} Grand Prix / {}", self.name, self.location);
        let line_width = if race_name.len() < 38 {
            38
        } else {
            race_name.len()
        };
        let border_width = line_width + 2;
        // format GP title
        writeln!(output, "+{}+", "-".repeat(border_width))?;
        writeln!(output, "| {race_name:^line_width$} |")?;
        writeln!(output, "+{}+", "-".repeat(border_width))?;

        // format all GP sessions
        self.sessions.pp_session(output, line_width)?;

        // closing border
        writeln!(output, "+{}+", "-".repeat(border_width))?;

        // time until next session
        let until_next = self.sessions.pp_time_until_next_session();
        if !until_next.is_empty() {
            writeln!(output, "{}", until_next)?;
        }
        Ok(())
    }
    pub fn gp_start_dt(&self) -> DateTime<Local> {
        self.sessions.gp_start_dt()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Schedule {
    races: Vec<GrandPrix>,
}

impl DataFetcher for Schedule {
    type A = Self;

    fn cache_file_name() -> String {
        "2024_schedule.json".to_owned()
    }

    fn resource_url() -> String {
        println!("Fetching schedule");
        SCHEDULE.to_owned()
    }

    fn process_data(raw_data: String, _file_path: &Path) -> Result<Self::A> {
        let data: Self::A = serde_json::from_str(&raw_data)?;
        Ok(data)
    }
}

impl Schedule {
    pub fn race_schedule(force_save: bool) -> Result<Vec<GrandPrix>> {
        Ok(Self::get_data(force_save)?.races)
    }
}
