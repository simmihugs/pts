use super::define::SiError;
use super::sistandard::*;
use chrono::{DateTime, Duration, Local, Utc};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    #[serde(rename = "eventId")]
    eventid: String,

    #[serde(rename = "serviceId")]
    serviceid: String,

    #[serde(rename = "programId")]
    programid: String,

    #[serde(rename = "startTime")]
    #[serde(deserialize_with = "starttime_from_str")]
    starttime: DateTime<Utc>,

    title: String,

    #[serde(rename = "siStandard")]
    sistandard: Option<SiStandard>,

    #[serde(deserialize_with = "duration_from_str")]
    duration: i64,

    endtime: Option<DateTime<Utc>>,

    #[serde(rename = "contentId")]
    contentid: Option<String>,
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_event_verbose(f, "Event")
    }
}

trait Take {
    fn take(&mut self, length: usize) -> String;
}

impl Take for String {
    fn take(&mut self, length: usize) -> String {
        if self.len() >= length {
            self.drain(0..length).collect::<String>().to_string()
        } else {
            for _ in 0..length - self.len() {
                *self += " ";
            }
            self.to_string()
        }
    }
}

impl Event {
    pub fn get_logo(&self) -> String {
        let mut title = self.get_title();
        match title.as_str() {
            "HDPLUHD_LOGO_1" => "Astra links".to_string(),
            "HDPLUHD_LOGO_2" => "Astra rechts".to_string(),
            "HDPLUHD_LOGO_3" => "HD Plus links".to_string(),
            "HDPLUHD_LOGO_4" => "HD Plus rechts".to_string(),
            "HDPLUHD_LOGO_5" => "Ran Live".to_string(),
            "HDPLUHD_LOGO_6" => "Ran Fighting".to_string(),
            "HDPLUHD_LOGO_7" => "K1_RUN".to_string(),
            "HDPLUHD_LOGO_8" => "P7_RUN".to_string(),
            "HDPLUHD_LOGO_9" => "HR Menorca".to_string(),
            "HDPLUHD_LOGO_10" => "Pro7 Ran Clean".to_string(),
            "HDPLUHD_LOGO_11" => "RTLZWEI_UHD".to_string(),
            "HDPLUHD_LOGO_13" => "P7_MAXX_UHD".to_string(),
            "HDPLUHD_LOGO_14" => "K1_DOKU_UHD".to_string(),
            "HDPLUHD_LOGO_15" => "P7_MAXX_RAN_UHD".to_string(),
            "HDPLUHD_LOGO_16" => "Sat.1 UHD ranBUNDESLIGA LIVE".to_string(),
            "HDPLUHD_LOGO_17" => "SAT.1 - Gold UHD".to_string(),
            "HDPLUHD_LOGO_18" => "Sixx UHD Logo".to_string(),
            "HDPLUHD_LOGO_19" => "P7MX_RAN_NHL".to_string(),
            s => {
                if s.contains("Pro7") || s.contains("Sat1") || s.contains("Kabel") {
                    title.take(15)
                } else {
                    println!("Logo error: {}", s);
                    "ERROR NO LOGO".to_string()
                }
            }
        }
    }

    pub fn get_duration(&self) -> i64 {
        self.duration
    }

    pub fn get_title(&self) -> String {
        self.title.to_string()
    }

    pub fn get_contentid(&self) -> String {
        self.contentid.clone().unwrap()
    }

    pub fn get_starttime(&self) -> Option<DateTime<Utc>> {
        Some(self.starttime)
    }

    pub fn get_endtime(&self) -> Option<DateTime<Utc>> {
        self.endtime
    }

    pub fn get_dstarttime(&self) -> Option<DateTime<Utc>> {
        match &self.sistandard {
            None => None,
            Some(sistandard) => sistandard.get_starttime(),
        }
    }

    pub fn get_dendtime(&self) -> Option<DateTime<Utc>> {
        match &self.sistandard {
            None => None,
            Some(sistandard) => sistandard.get_endtime(),
        }
    }

    pub fn calculate_endtime(&mut self) {
        self.endtime = Some(self.starttime + Duration::milliseconds(self.duration));
        match &mut self.sistandard {
            None => (),
            Some(ref mut sistandard) => sistandard.calculate_endtime(),
        }
    }

    pub fn fmt_event_verbose(&self, f: &mut fmt::Formatter<'_>, kind: &str) -> fmt::Result {
        let starttime = format!(
            "\n\t{:10} {}",
            "startime:",
            self.starttime.format("%Y-%m-%dT%H:%M:%S%.3fZ")
        );
        let endtime = match &self.endtime {
            None => "None".to_string(),
            Some(endtime) => format!(
                "\n\t{:10} {}",
                "endtime:",
                endtime.format("%Y-%m-%dT%H:%M:%S%.3fZ")
            ),
        };
        let title = format!("\n\ttitle: {}", &self.title);
        let eventid = format!("\n\teventid: {}", &self.eventid);
        let serviceid = format!("\n\tserviceid: {}", &self.serviceid);
        let programid = format!("\n\tprogramid: {}", &self.programid);
        let sistandard: String = match &self.sistandard {
            None => "".to_string(),
            Some(sistandard) => format!("\n\t{:?}", sistandard),
        };
        let contentid: String = match &self.contentid {
            None => "".to_string(),
            Some(sistandard) => format!("\n\tcontentId: {}", sistandard),
        };
        write!(
            f,
            "{kind}: {{{title}{eventid}{serviceid}{programid}{starttime}{endtime}{contentid}{sistandard}\n}}"
        )
    }

    pub fn print_si_events_verbose(
        &self,
        event: &Event,
        err: &Box<SiError>,
        display_err: &Box<SiError>,
        verbose: bool,
        utc: bool,
    ) {
        self.print_si_event_verbose(true, err, display_err, verbose, utc);
        event.print_si_event_verbose(false, err, display_err, verbose, utc);
        println!("");
    }

    pub fn print_event_verbose(
        &self,
        kind: &str,
        first: bool,
        err: &Box<SiError>,
        display_err: &Box<SiError>,
        verbose: bool,
        utc: bool,
    ) {
        let starttime = format!(
            "{:10} {}",
            "startime:",
            self.starttime.format("%Y-%m-%dT%H:%M:%S%.3fZ")
        );
        let endtime = match &self.endtime {
            None => "None".to_string(),
            Some(endtime) => format!(
                "{:10} {}",
                "endtime:",
                endtime.format("%Y-%m-%dT%H:%M:%S%.3fZ")
            ),
        };
        let title = format!("\n\ttitle: {}", &self.title);
        let eventid = format!("\n\teventid: {}", &self.eventid);
        let serviceid = format!("\n\tserviceid: {}", &self.serviceid);
        let programid = format!("\n\tprogramid: {}", &self.programid);

        let s = if !first {
            match **err {
                SiError::Gap => format!("{} #<-- Time gap", starttime)
                    .red()
                    .on_custom_color(colored::CustomColor {
                        r: 45,
                        g: 45,
                        b: 45,
                    }),
                SiError::Overlap => format!("{} #<-- Time overlap", starttime)
                    .red()
                    .on_custom_color(colored::CustomColor {
                        r: 45,
                        g: 45,
                        b: 45,
                    }),
                _ => starttime.on_red().clear(),
            }
        } else {
            starttime.on_red().clear()
        };
        let e = if first {
            match **err {
                SiError::Gap => format!("{} #<-- Time gap", endtime).red().on_custom_color(
                    colored::CustomColor {
                        r: 45,
                        g: 45,
                        b: 45,
                    },
                ),
                SiError::Overlap => format!("{} #<-- Time overlap", endtime)
                    .red()
                    .on_custom_color(colored::CustomColor {
                        r: 45,
                        g: 45,
                        b: 45,
                    }),
                _ => endtime.on_red().clear(),
            }
        } else {
            endtime.on_red().clear()
        };
        let si = match &self.sistandard {
            None => "".to_string(),
            Some(sistandard) => {
                sistandard.print_si_standard_verbose(first, display_err, verbose, utc)
            }
        };
        if verbose {
            println!(
                "{kind}: {{{title}{eventid}{serviceid}{programid}\n\t{}\n\t{}{}}}",
                s, e, si
            );
        } else {
            println!("{}\n{}\n{}\n{}", &self.programid, s, e, si);
        }
    }

    pub fn duration_to_string(&self, fps: Option<i64>) -> String {
        let mut milliseconds = self.duration;
        let hours = milliseconds / 3600_000;
        milliseconds -= hours * 3600_000;

        let minutes = milliseconds / 60_000;
        milliseconds -= minutes * 60_000;

        let seconds = milliseconds / 1000;
        milliseconds -= seconds * 1000;
        let milliseconds = match fps {
            None => {
                if milliseconds < 10 {
                    format!("00{milliseconds}")
                } else if milliseconds < 100 {
                    format!("0{milliseconds}")
                } else {
                    format!("{milliseconds}")
                }
            }
            Some(fps_number) => {
                let frames_per_second = milliseconds / (1000 / fps_number);
                if frames_per_second < 10 {
                    format!("0{}", frames_per_second)
                } else {
                    format!("{}", frames_per_second)
                }
            }
        };

        format!(
            "{}:{}:{}.{}",
            if hours < 10 {
                format!("0{hours}")
            } else {
                format!("{hours}")
            },
            if minutes < 10 {
                format!("0{minutes}")
            } else {
                format!("{minutes}")
            },
            if seconds < 10 {
                format!("0{seconds}")
            } else {
                format!("{seconds}")
            },
            milliseconds
        )
    }

    fn time_to_string(&self, time: DateTime<Utc>, utc: bool, fps: Option<i64>) -> String {
        let fps_str = format!("{}", time.format("%.3f")).replace(".", "");
        let fps_str = match fps {
            Some(fps_number) => match fps_str.parse::<i64>() {
                Ok(number) => {
                    let frames_per_second = number / (1000 / fps_number);
                    if frames_per_second < 10 {
                        format!("0{}", frames_per_second)
                    } else {
                        format!("{}", frames_per_second)
                    }
                }
                Err(..) => fps_str,
            },
            None => fps_str,
        };
        let time_str = format!("{}", time.format("%d.%m.%Y %H:%M:%S"));

        if utc {
            format!("{}.{}", time_str, fps_str)
        } else {
            let time: DateTime<Local> = DateTime::from(time);
            let time_str = format!("{}", time.format("%d.%m.%Y %H:%M:%S"));
            format!("{}.{}", time_str, fps_str)
        }
    }

    pub fn starttime_to_string(&self, utc: bool, fps: Option<i64>) -> String {
        self.time_to_string(self.starttime, utc, fps)
    }

    pub fn endtime_to_string(&self, utc: bool, fps: Option<i64>) -> String {
        match self.endtime {
            None => String::from("No endtime"),
            Some(time) => self.time_to_string(time, utc, fps),
        }
    }

    pub fn programid_to_string(&self) -> String {
        format!("{}", self.programid)
    }

    pub fn title_to_string(&self) -> String {
        let title = &self.title;
        if title.len() < 30 {
            format!("{}", title)
        } else {
            format!("{}", &title[..30])
        }
    }

    pub fn print_si_event_verbose(
        &self,
        first: bool,
        err: &Box<SiError>,
        display_err: &Box<SiError>,
        verbose: bool,
        utc: bool,
    ) {
        self.print_event_verbose("SiEvent", first, err, display_err, verbose, utc);
    }
}
