use super::define::SiError;
use super::sistandard::*;
use chrono::{DateTime, Duration, Utc};
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
            Some(sistandard) => format!("\n\t{:?}\n", sistandard),
        };
        write!(
            f,
            "{kind}: {{{title}{eventid}{serviceid}{programid}{starttime}{endtime}{sistandard}}}"
        )
    }

    pub fn print_si_events_verbose(
        &self,
        event: &Event,
        err: &Box<SiError>,
        display_err: &Box<SiError>,
        verbose: bool,
    ) {
        self.print_si_event_verbose(true, err, display_err, verbose);
        event.print_si_event_verbose(false, err, display_err, verbose);
	println!("");
    }

    pub fn print_si_event_verbose(
        &self,
        first: bool,
        err: &Box<SiError>,
        display_err: &Box<SiError>,
        verbose: bool,
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
            Some(sistandard) => sistandard.print_si_standard_verbose(first, display_err, verbose),
        };
        if verbose {
            println!(
                "SiEvent: {{{title}{eventid}{serviceid}{programid}\n\t{}\n\t{}{}}}",
                s, e, si
            );
        } else {
            println!("{}\n{}\n{}\n{}", &self.programid, s, e, si);
        }
    }
}
