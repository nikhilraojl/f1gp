use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NormalWeekend {
    pub fp1: DateTime<Local>,
    pub fp2: DateTime<Local>,
    pub fp3: DateTime<Local>,
    pub qualifying: DateTime<Local>,
    pub gp: DateTime<Local>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct SprintWeekend {
    pub fp1: DateTime<Local>,
    pub sprintQualifying: DateTime<Local>,
    pub sprint: DateTime<Local>,
    pub qualifying: DateTime<Local>,
    pub gp: DateTime<Local>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Weekend {
    Normal(NormalWeekend),
    Sprint(SprintWeekend),
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct GP {
    pub name: String,
    pub location: String,
    latitude: f32,
    longitude: f32,
    round: usize,
    slug: String,
    localeKey: String,
    pub sessions: Weekend,
}

#[derive(Deserialize, Debug)]
pub struct Races {
    pub races: Vec<GP>,
}
