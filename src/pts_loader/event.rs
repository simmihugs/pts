use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::sistandard::*;

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
}

