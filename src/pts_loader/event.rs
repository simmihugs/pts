use crate::commandline::commandline::Commandline;
use crate::utils::take::Take;

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

    offset: Option<String>,
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_event_verbose(f, "Event")
    }
}
impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_event(f)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
struct Tcin {
    #[serde(deserialize_with = "duration_from_str")]
    duration: i64,
}

impl Event {
    pub fn get_logo(&self) -> String {
        String::from(match self.get_contentid().as_str() {
            //Layouts
            "HDPLUHD_LAY_1" => "Kabel1 rechts",
            "HDPLUHD_LAY_2" => "Pro7 rechts",
            "HDPLUHD_LAY_3" => "Sat1 rechts",
            "HDPLUHD_LAY_4" => "Dauerwerbesendung",
            "HDPLUHD_LAY_5" => "Produktplatzierung",
            "HDPLUHD_LAY_6" => "Crawl",
            "HDPLUHD_LAY_7" => "ZDF 4k links",
            "HDPLUHD_LAY_8" => "Flag UHD Highlights",
            "HDPLUHD_LAY_9" => "Flag Promo-Elemente für HD+",
            "HDPLUHD_LAY_10" => "L-Shape",
            "HDPLUHD_LAY_11" => "Bauchbinde",
            "HDPLUHD_LAY_12" => "Dolby Atoms",
            //Logos
            "HDPLUHD_LOGO_1" => "ERROR Astra links",
            "HDPLUHD_LOGO_2" => "ERROR Astra rechts",
            "HDPLUHD_LOGO_3" => "HD Plus links",
            "HDPLUHD_LOGO_4" => "HD Plus rechts",
            "HDPLUHD_LOGO_5" => "Ran Live",
            "HDPLUHD_LOGO_6" => "Ran Fighting",
            "HDPLUHD_LOGO_7" => "K1_RUN",
            "HDPLUHD_LOGO_8" => "P7_RUN",
            "HDPLUHD_LOGO_9" => "HR Menorca",
            "HDPLUHD_LOGO_10" => "Pro7 Ran Clean",
            "HDPLUHD_LOGO_11" => "RTLZWEI_UHD",
            "HDPLUHD_LOGO_13" => "P7_MAXX_UHD",
            "HDPLUHD_LOGO_14" => "K1_DOKU_UHD",
            "HDPLUHD_LOGO_15" => "P7_MAXX_RAN_UHD",
            "HDPLUHD_LOGO_16" => "Sat.1 UHD ranBUNDESLIGA LIVE",
            "HDPLUHD_LOGO_17" => "SAT.1 - Gold UHD",
            "HDPLUHD_LOGO_18" => "Sixx UHD Logo",
            "HDPLUHD_LOGO_19" => "P7MX_RAN_NHL",
            //Unknown id
            _ => "ERROR NO LOGO",
        })
    }

    pub fn get_duration(&self) -> i64 {
        self.duration
    }

    pub fn get_tcin_tcout(&self) -> Option<(i64, i64)> {
        match &self.offset {
            None => None,
            Some(s) => {
                match serde_xml_rs::from_str::<Tcin>(&format!("<Tcin duration=\"{}\"></Tcin>", s,))
                {
                    Ok(d) => Some((d.duration, d.duration + &self.duration)),
                    Err(..) => None,
                }
            }
        }
    }

    pub fn get_title(&self) -> String {
        self.title.to_string()
    }

    pub fn get_contentid(&self) -> String {
        self.contentid.clone().unwrap()
    }

    pub fn get_programid(&self) -> String {
        self.programid.clone()
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

    pub fn get_text(&self) -> Option<String> {
        match &self.sistandard {
            None => None,
            Some(sistandard) => Some(sistandard.get_text()),
        }
    }

    pub fn get_dendtime(&self) -> Option<DateTime<Utc>> {
        match &self.sistandard {
            None => None,
            Some(sistandard) => sistandard.get_endtime(),
        }
    }

    pub fn get_displayed_duration(&self, cmd: &Commandline) -> i64 {
        match &self.sistandard {
            None => cmd.minimum(),
            Some(s) => s.get_duration(),
        }
    }

    pub fn calculate_endtime(&mut self) {
        let duration = match chrono::TimeDelta::try_milliseconds(self.duration) {
            Some(duration) => duration,
            None => Duration::new(0, 0).unwrap(),
        };
        self.endtime = Some(self.starttime + duration);
        match &mut self.sistandard {
            None => (),
            Some(ref mut sistandard) => sistandard.calculate_endtime(),
        }
    }

    pub fn print_vaevent_verbose(&self, time_error: &bool, utc: bool, fps: Option<i64>) {
        let contentid = self.get_contentid();
        let mut title = self.title_to_string();
        if title == " -  UHD1_WERBUNG-01" {
            title = "Werbung".to_string();
        } else if self.get_contentid() == "cb7a119f84cb7b117b1b" {
            title = "Dranbleiben".to_string();
        } else if self.get_contentid() == "392654926764849cd5dc" {
            title = "Pausentafel ".to_string();
        }
        println!(
            "| {:30} | {:15} | {:23} | {:23} | {:12} | {:20} |",
            title,
            self.programid_to_string(),
            if *time_error {
                self.starttime_to_string(utc, fps).red()
            } else {
                self.starttime_to_string(utc, fps).red().clear()
            },
            self.endtime_to_string(utc, fps),
            if title == "Werbung" {
                self.duration_to_string(fps).yellow()
            } else {
                self.duration_to_string(fps).yellow().clear()
            },
            if contentid.contains("-") && !contentid.contains("WERB") {
                contentid.red()
            } else {
                contentid.red().clear()
            },
        );
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

    pub fn fmt_event(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let endtime = match &self.endtime {
            None => "".to_string().take(25),
            Some(endtime) => format!("{}", endtime.format("%Y-%m-%dT%H:%M:%S%.3fZ")).take(25),
        };
        let starttime = format!("{}", self.starttime.format("%Y-%m-%dT%H:%M:%S%.3fZ")).take(25);
        let title = format!("{}", &self.title).take(25);
        write!(f, "{title:25} {starttime:25} {endtime:25}",)
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

    pub fn a_duration_to_string(duration: i64, fps: Option<i64>) -> String {
        let mut milliseconds = duration;
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
    
    pub fn duration_to_string(&self, fps: Option<i64>) -> String {
        Event::a_duration_to_string(self.duration, fps)
    }

    pub fn standalone_duration_to_string(duration: &i64, fps: Option<i64>) -> String {
        let mut milliseconds = *duration;
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
        format!("{}", &self.title).take(30)
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
