use crate::pts_loader::{event::Event, sistandard::*};
use chrono::{DateTime, NaiveDate, Utc};
use clap::{CommandFactory, Parser};
use serde::{Deserialize, Serialize};
use serde_json::json;

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

const DEFAULT_VALID_RANGE: &str = "DEFAULT_VALID_RANGE";

const DEFAULT_FLUID_DATABASE: &str =
    "C:\\Users\\SimonGraetz\\OneDrive - CreateCtrl AG\\uhd1-plannung\\uhd_fluid_database.csv";

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

    #[arg(long, default_value_t = String::from(""))]
    werbungen: String,

    #[arg(long, default_value_t = String::from(""))]
    tcins_and_tcouts: String,

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

    #[arg(long, default_value_t = String::from(DEFAULT_VALID_RANGE))]
    valid_range: String,

    #[arg(short, long, default_value_t = false)]
    debug: bool,

    #[arg(short, long, default_value_t = false)]
    today: bool,

    #[arg(long)]
    day: Option<String>,

    #[arg(long, default_value_t = 5 * 60 * 1000)]
    minimum: i64,

    #[arg(long, default_value_t = false)]
    update_werbungen: bool,

    #[arg(long)]
    fluid: Option<Option<String>>,

    #[arg(long, default_value_t = false)]
    display_sievents: bool,

    #[arg(long, default_value_t = false)]
    display_trailers: bool,

    #[arg(long, default_value_t = false)]
    only_sendepausen: bool,

    #[arg(long, default_value_t = false)]
    check_all_contentids: bool,

    #[arg(long, default_value_t = false)]
    update_fluid_data_base: bool,
}

pub struct Commandline {
    args: Args,
}

impl Commandline {
    #[allow(dead_code)]
    pub fn copy(&self) -> Commandline {
        Self {
            args: Args::parse().clone(),
        }
    }

    pub fn day(&self) -> Option<NaiveDate> {
        match &self.args.day {
            Some(s) => {
                let date = NaiveDate::parse_from_str(&s, "%d.%m.%Y");
                match date {
                    Ok(d) => {
                        Some(d)
                    }
                    Err(err) => {
                        println!("{:?}", err);
                        println!("required format is dd.mm.yyyy");
                        None
                    }
                }
            }
            None => None,
        }
    }

    pub fn parse() -> Self {
        Self {
            args: Args::parse(),
        }
    }

    pub fn update_fluid_data_base(&self) -> bool {
        self.args.update_fluid_data_base
    }

    pub fn update_werbungen(&self) -> bool {
        self.args.update_werbungen
    }

    pub fn check_all_contentids(&self) -> bool {
        self.args.check_all_contentids
    }

    pub fn minimum(&self) -> i64 {
        self.args.minimum
    }

    #[allow(dead_code)]
    pub fn set_debug(&mut self, value: bool) {
        self.args.debug = value;
    }

    pub fn debug(&self) -> bool {
        self.args.debug
    }

    pub fn today(&self) -> bool {
        self.args.today
    }

    pub fn display_sievents(&self) -> bool {
        self.args.display_sievents
    }

    pub fn werbungen(&self) -> Option<Vec<Vec<String>>> {
        let werbungen: Vec<Vec<String>> = self
            .args
            .werbungen
            .split(";")
            .map(|x| {
                x.to_string()
                    .split("--")
                    .map(|y| y.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();
        if werbungen.len() == 0 {
            None
        } else {
            Some(werbungen)
        }
    }

    #[allow(dead_code)]
    pub fn tcins_tcouts(&self) -> Option<Vec<Vec<String>>> {
        let tcins_tcouts: Vec<Vec<String>> = self
            .args
            .tcins_and_tcouts
            .split(";")
            .map(|x| {
                x.to_string()
                    .split("--")
                    .map(|y| y.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();
        if tcins_tcouts.len() == 0 {
            None
        } else {
            Some(tcins_tcouts)
        }
    }

    pub fn display_sievents_only_sendepausen(&self) -> bool {
        self.args.only_sendepausen
    }

    pub fn display_trailers(&self) -> bool {
        self.args.display_trailers
    }

    pub fn fluid_csv(&self) -> Option<String> {
        match &self.args.fluid {
            None => None,
            Some(None) => Some(String::from(DEFAULT_FLUID_DATABASE)),
            Some(s) => s.clone(),
        }
    }

    pub fn valid_range(&self) -> Option<Range> {
        let default_range: Range = match serde_json::from_value::<Range>(json!({
            "startTime": "2000-01-01T01:00:00.000Z",
            "endTime": "2100-01-01T01:00:00.000Z",
        })) {
            Err(err) => {
                panic!("{}", err);
            }
            Ok(range) => range,
        };
        let range_str = self.args.valid_range.to_string();
        match serde_json::from_str::<Range>(&range_str) {
            Err(error) => {
                if range_str == DEFAULT_VALID_RANGE {
                    if self.debug() {
                        println!("{}", "default value");
                    }
                    Some(default_range)
                } else if error.is_eof() {
                    None
                } else if error.is_syntax() {
                    if self.debug() {
                        println!("{}", "error is syntax");
                    }
                    let range_parts: Vec<Vec<String>> = self
                        .args
                        .valid_range
                        .to_string()
                        .split(";")
                        .map(|x| x.to_string().split("=").map(|y| y.to_string()).collect())
                        .collect();

                    if range_parts.len() == 2
                        && range_parts[0].len() == 2
                        && range_parts[1].len() == 2
                    {
                        let json = json!({
                            "startTime": format!("{}", range_parts[0][1]),
                            "endTime": format!("{}", range_parts[1][1])
                        });

                        match serde_json::from_value::<Range>(json) {
                            Err(err) => {
                                if self.debug() {
                                    println!("{:?}", err);
                                }
                                None
                            }
                            Ok(json) => {
                                if self.debug() {
                                    println!("{:?}", json);
                                }
                                if json.start_time < json.end_time {
                                    Some(json)
                                } else {
                                    if self.debug() {
                                        println!("Invalid range: {:?}", json);
                                    }
                                    None
                                }
                            }
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Ok(range) => {
                if range.start_time < range.end_time {
                    Some(range)
                } else {
                    if self.debug() {
                        println!("Invalid range: {:?}", range);
                    }
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
            || self.missing_texts()
            || self.display_sievents())
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
