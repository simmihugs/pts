use super::define::SiError;
use super::sistandard::*;
use chrono::{DateTime, Duration, Utc, Local};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq)]
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

impl Event {
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

    pub fn duration_to_string(&self) -> String {
        let mut milliseconds = self.duration;
        let hours = milliseconds / 3600_000;
        milliseconds -= hours * 3600_000;

        let minutes = milliseconds / 60_000;
        milliseconds -= minutes * 60_000;

        let seconds = milliseconds / 1000;
        milliseconds -= seconds * 1000;

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
            if milliseconds < 10 {
                format!("00{milliseconds}")
            } else if milliseconds < 100 {
                format!("0{milliseconds}")
            } else {
                format!("{milliseconds}")
            }
        )
    }

    pub fn starttime_to_string(&self, utc: bool) -> String {
	if utc {
            format!("{}", self.starttime.format("%d.%m.%Y %H:%M:%S%.3f"))	    
	} else {
	    let starttime: DateTime<Local> = DateTime::from(self.starttime);
	    format!("{}", starttime.format("%d.%m.%Y %H:%M:%S%.3f"))
	}
    }
    
    pub fn programid_to_string(&self) -> String {
        format!("{}", self.programid)
    }
    
    pub fn endtime_to_string(&self, utc: bool) -> String {
	if utc {
            format!("{}", self.endtime.unwrap().format("%d.%m.%Y %H:%M:%S%.3f"))	    
	} else {
	    let endtime: DateTime<Local> = DateTime::from(self.endtime.unwrap());
	    format!("{}", endtime.format("%d.%m.%Y %H:%M:%S%.3f"))
	}
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
