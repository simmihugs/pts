use crate::pts_loader::sistandard::starttime_from_str;
use chrono::{DateTime, NaiveDate, Utc};
use clap::{CommandFactory, Parser};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::Read;

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

const CONTENT_IDS_DATABASE: &str =
    "C:\\Users\\SimonGraetz\\OneDrive - CreateCtrl AG\\uhd1-plannung\\content_ids.txt";
static CONTENT_IDS: &'static [&str; 21] = &[
    "cb7a119f84cb7b117b1b",
    "392654926764849cd5dc",
    "e90dfb84e30edf611e32",
    "b1735b7c5101727b3c6c",
    "5675d8c63df2424bf286",
    "64bb104f8aa130071723",
    "29996549985440a20fa1",
    "563f387cf4cfd279039a",
    "b52d22eeb30a63a4518f",
    "e4a2e62d68e2ad9bfaae",
    "75d1d4afe3f26b6412d4",
    "e48363d83407359a6dd2",
    "34500e2e4a0d1a0806bb",
    "WERBUNG",
    "cb7a119f84cb7b117b1b",
    "ec12fb722064b74776d6",
    "98bcc270bf534db740b8",
    "a81fe4c3875d5ab4bfa5",
    "2d9aec2d4a2e12c0b8bc",
    "33e36ad39c3bc14d66b3",
    "UHD_LIVE",
];

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

    #[arg(short, long)]
    csv: Option<Option<String>>,

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

    #[arg(long)]
    content_ids_to_ignore: Option<String>,

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
    content_ids_vec: Vec<String>,
}

impl Commandline {
    #[allow(dead_code)]
    pub fn copy(&self) -> Commandline {
        Self {
            args: Args::parse().clone(),
            content_ids_vec: self.content_ids_vec.clone(),
        }
    }

    pub fn get_content_ids_to_ignore(&self) -> Vec<String> {
        self.content_ids_vec.clone()
    }

    pub fn day(&self) -> Option<NaiveDate> {
        match &self.args.day {
            Some(s) => {
                let date = NaiveDate::parse_from_str(&s, "%d.%m.%Y");
                match date {
                    Ok(d) => Some(d),
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
        let args: Args = Args::parse();

        let mut content_ids_vec: Vec<String> = CONTENT_IDS.iter().map(|&s| s.to_string()).collect();
        match File::open(CONTENT_IDS_DATABASE) {
            Ok(mut f) => {
                let mut tmp = String::new();
                let _ = f.read_to_string(&mut tmp);
                for value in tmp.lines() {
                    content_ids_vec.push(value.trim().to_string());
                }
            }
            Err(err) => {
                if args.debug {
                    println!("{:?}", err)
                }
            }
        }
        match args.content_ids_to_ignore {
            None => (),
            Some(ref s) => match File::open(s) {
                Ok(mut f) => {
                    let mut tmp = String::new();
                    let _ = f.read_to_string(&mut tmp);
                    for line in tmp.lines() {
                        content_ids_vec.push(line.trim().to_string());
                    }
                }
                _ => (),
            },
        }
        content_ids_vec.sort();
        content_ids_vec.dedup();

        if args.debug {
            println!("{}", "content ids to ignore for logos");
            for (index, value) in content_ids_vec.iter().enumerate() {
                println!("{}\t{}", index, value);
            }
        }
        Self {
            args,
            content_ids_vec,
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

    pub fn csv(&self) -> String {
        match &self.args.csv {
            None => String::from("YOU_PICK_A_CSV"),
            Some(None) => String::from("bloecke.csv"),
            Some(file_name) => format!("{}", file_name.clone().unwrap()),
        }
    }

    pub fn write_csv(&self) -> bool {
        self.csv() != "YOU_PICK_A_CSV"
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
