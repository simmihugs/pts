use clap::{CommandFactory, Parser};

use serde::{Deserialize, Serialize};
//use serde_json::json;
use crate::pts_loader::sistandard::*;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    #[serde(rename = "startTime")]
    #[serde(deserialize_with = "starttime_from_str")]
    pub start_time: DateTime<Utc>,

    #[serde(rename = "endTime")]
    #[serde(deserialize_with = "starttime_from_str")]
    pub end_time: DateTime<Utc>,
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.start_time, self.end_time)
    }
}

#[derive(Clone, Serialize, Deserialize, Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("YOU_PICK_A_FILE"))]
    filename: String,

    #[arg(short, long, default_value_t = false)]
    repl: bool,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[arg(short, long, default_value_t = false)]
    ps_event: bool,

    #[arg(short, long, default_value_t = false)]
    utc: bool,

    #[arg(short, long, default_value_t = false)]
    sierrors: bool,

    #[arg(short, long, default_value_t = String::from("YOU_PICK_ILLEGAL_EVENTS"))]
    illegalevents: String,

    #[arg(short, long, default_value_t = false)]
    all: bool,

    #[arg(short, long, default_value_t = false)]
    only_errors: bool,

    #[arg(short, long, default_value_t = String::from("YOU_PICK_A_CSV"))]
    csv: String,

    #[arg(short, long, default_value_t = String::from("utf-8"))]
    encoding: String,

    #[arg(long, default_value_t = -1)]
    fps: i64,

    #[arg(long, default_value_t = false)]
    vaerrors: bool,

    #[arg(short, long, default_value_t = false)]
    missing_texts: bool,

    #[arg(long, default_value_t = String::from(""))]
    valid_range: String,
    // #[arg(long)]
    // valid_range: Option<Range>,
}

pub struct Commandline {
    args: Args,
}

impl Commandline {
    pub fn parse() -> Self {
        Self {
            args: Args::parse(),
        }
    }

    pub fn valid_range(&self) -> Option<Range> {
        match serde_json::from_str::<Range>(&self.args.valid_range) {
            Err(error) => {
                println!("{:?}", error);
                None
            }
            Ok(range) => {
                if range.start_time < range.end_time {
                    Some(range)
                } else {
                    println!("Invalid range: {:?}", range);
                    None
                }
            }
        }
    }

    pub fn only_errors(&self) -> bool {
        self.args.only_errors
    }

    pub fn ps_event(&self) -> bool {
        self.args.ps_event
    }

    pub fn verbose(&self) -> bool {
        self.args.verbose
    }

    pub fn utc(&self) -> bool {
        self.args.utc
    }

    pub fn encoding(&self) -> &String {
        &self.args.encoding
    }

    pub fn sierrors(&self) -> bool {
        self.args.sierrors
    }

    pub fn vaerrors(&self) -> bool {
        self.args.vaerrors
    }

    pub fn missing_texts(&self) -> bool {
        self.args.missing_texts
    }

    pub fn all(&self) -> bool {
        self.args.all
    }

    pub fn print_help() {
        let mut cmd = Args::command();
        let _ = cmd.print_help();
    }

    pub fn filename(&self) -> &String {
        &self.args.filename
    }

    pub fn fps(&self) -> Option<i64> {
        if self.args.fps == -1 {
            None
        } else {
            Some(self.args.fps)
        }
    }

    pub fn csv(&self) -> &String {
        &self.args.csv
    }

    pub fn write_csv(&self) -> bool {
        self.args.csv != "YOU_PICK_A_CSV"
    }

    pub fn look_for_illegalevents(&self) -> bool {
        match self.illegalevents() {
            None => false,
            _ => true,
        }
    }

    pub fn no_option(&self) -> bool {
        !(self.look_for_illegalevents()
            || self.all()
            || self.write_csv()
            || self.ps_event()
            || self.sierrors()
            || self.vaerrors()
            || self.missing_texts())
    }

    pub fn illegalevents(&self) -> Option<Vec<String>> {
        let illegals = &self.args.illegalevents;
        if illegals == "YOU_PICK_ILLEGAL_EVENTS" {
            None
        } else {
            Some(
                self.args
                    .illegalevents
                    .split(';')
                    .map(|x| String::from(x))
                    .collect::<Vec<String>>()
                    .to_vec(),
            )
        }
    }
}
