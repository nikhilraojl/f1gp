use chrono::{DateTime, Local};
use serde::Deserialize;
use std::fmt::Write;
use std::fs;

use crate::error::Result;

const STR_FMT: &str = "%a %d/%m/%Y %H:%M";

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

#[derive(Deserialize, Debug)]
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
            pp_session("FP 1", self.fp1, curr_dt, session_width, line_width)
        )?;
        writeln!(
            output,
            "{}",
            pp_session("FP 2", self.fp2, curr_dt, session_width, line_width)
        )?;
        writeln!(
            output,
            "{}",
            pp_session("FP 3", self.fp3, curr_dt, session_width, line_width)
        )?;
        writeln!(
            output,
            "{}",
            pp_session("Quali", self.qualifying, curr_dt, session_width, line_width)
        )?;
        writeln!(
            output,
            "{}",
            pp_session("Race", self.gp, curr_dt, session_width, line_width)
        )?;
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
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
            pp_session("FP 1", self.fp1, curr_dt, session_width, line_width)
        )?;
        writeln!(
            output,
            "{}",
            pp_session(
                "Spr Quali",
                self.sprintQualifying,
                curr_dt,
                session_width,
                line_width
            )
        )?;
        writeln!(
            output,
            "{}",
            pp_session("Sprint", self.sprint, curr_dt, session_width, line_width)
        )?;
        writeln!(
            output,
            "{}",
            pp_session("Quali", self.qualifying, curr_dt, session_width, line_width)
        )?;
        writeln!(
            output,
            "{}",
            pp_session("Race", self.gp, curr_dt, session_width, line_width)
        )?;
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Weekend {
    Normal(NormalWeekend),
    Sprint(SprintWeekend),
}

impl Weekend {
    pub fn gp_start_dt(&self) -> DateTime<Local> {
        match self {
            Self::Normal(sessions) => sessions.gp,
            Self::Sprint(sessions) => sessions.gp,
        }
    }
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct GP {
    name: String,
    location: String,
    sessions: Weekend,
}
impl GP {
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

        // format GP sessions
        match self.sessions {
            Weekend::Normal(ref session) => session.pp_normal(output, line_width),
            Weekend::Sprint(ref session) => session.pp_sprint(output, line_width),
        }?;

        // closing border
        writeln!(output, "+{}+", "-".repeat(border_width))?;
        Ok(())
    }
    pub fn gp_start_dt(&self) -> DateTime<Local> {
        self.sessions.gp_start_dt()
    }
}

#[derive(Deserialize, Debug)]
pub struct Races {
    pub races: Vec<GP>,
}

pub fn race_schedule(force_save: bool) -> Result<Races> {
    let schedule_dir = std::env::temp_dir().join("f1_2024_schedule");
    if !schedule_dir.exists() {
        std::fs::create_dir(&schedule_dir)?;
    }
    let schedule_file = schedule_dir.join("2024_schedule.json");
    if !schedule_file.exists() || force_save {
        let raw_data = get_data_from_github()?;
        fs::write(schedule_file, &raw_data)?;
        let races: Races = serde_json::from_str(&raw_data)?;
        Ok(races)
    } else {
        let data = fs::read_to_string(schedule_file)?;
        let races: Races = serde_json::from_str(&data)?;
        Ok(races)
    }
}

fn get_data_from_github() -> Result<String> {
    println!("Fetching schedule from internet");
    let body: String =
        ureq::get("https://raw.githubusercontent.com/sportstimes/f1/main/_db/f1/2024.json")
            .call()?
            .into_string()?;
    Ok(body)
}
