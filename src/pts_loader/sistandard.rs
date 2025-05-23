use super::define::SiError;
use chrono::{DateTime, Duration, LocalResult, NaiveDateTime, NaiveTime, TimeZone, Utc};
use colored::Colorize;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Description {
    #[serde(rename = "languageCode")]
    languagecode: String,

    #[serde(rename = "eventName")]
    eventname: String,

    #[serde(rename = "shortDescription")]
    shortdescription: String,

    #[serde(rename = "longDescription")]
    longdescription: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct SiDescriptions {
    description: Description,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct SiStandard {
    #[serde(rename = "displayedStart")]
    #[serde(deserialize_with = "starttime_from_str")]
    starttime: DateTime<Utc>,

    #[serde(rename = "displayedDuration")]
    #[serde(deserialize_with = "duration_from_str")]
    duration: i64,

    endtime: Option<DateTime<Utc>>,

    #[serde(rename = "siDescriptions")]
    sidescriptions: SiDescriptions,
}

impl fmt::Debug for SiStandard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_verbose(f)
    }
}

impl SiStandard {
    pub fn get_text(&self) -> String {
        self.sidescriptions.description.longdescription.to_string()
    }

    #[allow(dead_code)]
    pub fn calculate_endtime(&mut self) {
        let duration: Duration = match chrono::TimeDelta::try_milliseconds(self.duration) {
            Some(duration) => duration,
            None => Duration::new(0, 0).unwrap(),
        };
        self.endtime = Some(self.starttime + duration)
    }

    pub fn get_duration(&self) -> i64 {
        self.duration
    }

    pub fn get_endtime(&self) -> Option<DateTime<Utc>> {
        self.endtime
    }

    pub fn get_starttime(&self) -> Option<DateTime<Utc>> {
        Some(self.starttime)
    }

    fn fmt_verbose(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let startime = format!(
            "\n\t\t{:10} {}",
            "starttime:",
            self.starttime.format("%Y-%m-%dT%H:%M:%S%.3fZ")
        );

        let endtime = match &self.endtime {
            None => "None".to_string(),
            Some(endtime) => {
                format!(
                    "\n\t\t{:10} {}\t",
                    "endtime:",
                    endtime.format("%Y-%m-%dT%H:%M:%S%.3fZ")
                )
            }
        };

        let text = format!(
            "\n\t\t{:10} {}\n\t",
            "longDescription:", &self.sidescriptions.description.longdescription
        );

        write!(f, "SiStandard: {{{startime}{endtime}{text}}}",)
    }

    #[allow(dead_code, unused_variables)]
    pub fn print_si_standard_verbose(
        &self,
        first: bool,
        display_err: &Box<SiError>,
        verbose: bool,
        utc: bool,
    ) -> String {
        let startime = format!(
            "{:10} {}",
            "displayed starttime:",
            self.starttime.format("%Y-%m-%dT%H:%M:%S%.3fZ")
        );
        let endtime = match &self.endtime {
            None => "None".to_string(),
            Some(endtime) => {
                format!(
                    "{:10} {}",
                    "displayed endtime:",
                    endtime.format("%Y-%m-%dT%H:%M:%S%.3fZ")
                )
            }
        };

        let s = if !first {
            match **display_err {
                SiError::Gap => format!("{} #<-- Time gap", startime).red().on_custom_color(
                    colored::CustomColor {
                        r: 10,
                        g: 50,
                        b: 50,
                    },
                ),
                SiError::Overlap => format!("{} #<-- Time overlap", startime)
                    .red()
                    .on_custom_color(colored::CustomColor {
                        r: 10,
                        g: 50,
                        b: 50,
                    }),
                _ => startime.on_red().clear(),
            }
        } else {
            startime.on_red().clear()
        };
        let e = if first {
            match **display_err {
                SiError::Gap => format!("{} #<-- Time gap", endtime).red().on_custom_color(
                    colored::CustomColor {
                        r: 10,
                        g: 50,
                        b: 50,
                    },
                ),
                SiError::Overlap => format!("{} #<-- Time overlap", endtime)
                    .red()
                    .on_custom_color(colored::CustomColor {
                        r: 10,
                        g: 50,
                        b: 50,
                    }),
                _ => endtime.on_red().clear(),
            }
        } else {
            endtime.on_red().clear()
        };

        if verbose {
            return format!("\n\tSiStandard: {{\n\t\t{}\n\t\t{}\n\t}}\n", s, e);
        } else {
            return format!("{}\n{}", s, e);
        };
    }

    #[allow(dead_code)]
    fn fmt_not_verbose(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let startime = format!(
            "\n\t\t{:10} {}",
            "starttime:",
            self.starttime.format("%Y-%m-%dT%H:%M:%S%.3fZ")
        );
        let endtime = match &self.endtime {
            None => "None".to_string(),
            Some(endtime) => {
                format!(
                    "\n\t\t{:10} {}\n\t",
                    "endtime:",
                    endtime.format("%Y-%m-%dT%H:%M:%S%.3fZ")
                )
            }
        };
        write!(f, "SiStandard: {{{startime}{endtime}}}",)
    }
}

pub fn starttime_from_str<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.3fZ");
    match dt {
        Ok(dtt) => {
            let dt2: LocalResult<DateTime<Utc>> = Utc.from_local_datetime(&dtt);
            Ok(dt2.unwrap())
        }
        Err(e) => Err(serde::de::Error::custom(format!("{}", e))),
    }
}

pub fn duration_from_str<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let step = NaiveTime::parse_from_str("00 00:00:00.000", "00 %H:%M:%S%.3f").unwrap();
    let naivetime = NaiveTime::parse_from_str(&s, "00 %H:%M:%S%.3f");
    match naivetime {
        Ok(time) => {
            let dur: Duration = time - step;
            Ok(dur.num_milliseconds())
        }
        Err(..) => Err(serde::de::Error::custom("could not calcuate the duration")),
    }
}

pub fn a_duration_from_string(string: String) -> Result<i64, String> {
    let parts = string
        .split(".")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let time_string = &format!("00 {}.{}", parts[0], &parts[1][1..]);
    let fmt = "00 %H:%M:%S%.3f";
    let step = NaiveTime::parse_from_str("00 00:00:00.000", "00 %H:%M:%S%.3f").unwrap();
    let naivetime = NaiveTime::parse_from_str(time_string, fmt);
    match naivetime {
        Ok(time) => {
            let dur: Duration = time - step;
            Ok(dur.num_milliseconds())
        }
        Err(..) => Err(format!("{}", string)),
    }
}
