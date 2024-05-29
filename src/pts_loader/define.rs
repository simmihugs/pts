use crate::commandline::commandline::Commandline;
use chrono::{DateTime, LocalResult, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::event::*;

#[allow(non_camel_case_types)]
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum Define {
    vaEvent(Event),
    logoEvent(Event),
    layoutEvent(Event),
    siEvent(Event),
}

impl fmt::Debug for Define {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Define::vaEvent(event) => event.fmt_event_verbose(f, "vaEvent"),
            Define::siEvent(event) => event.fmt_event_verbose(f, "siEvent"),
            Define::logoEvent(event) => event.fmt_event_verbose(f, "logoEvent"),
            Define::layoutEvent(event) => event.fmt_event_verbose(f, "layoutEvent"),
        }
    }
}

impl fmt::Display for Define {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Define::vaEvent(event) => event.fmt_event(f),
            Define::siEvent(event) => event.fmt_event(f),
            Define::logoEvent(event) => event.fmt_event(f),
            Define::layoutEvent(event) => event.fmt_event(f),
        }
    }
}

#[derive(PartialEq)]
pub enum SiError {
    NoError,
    Overlap,
    Gap,
    Under5,
    SomeError(Box<SiError>, Box<SiError>),
}

pub fn create_time(s: &str) -> Option<DateTime<Utc>> {
    let dt = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.3fZ");
    match dt {
        Ok(dtt) => {
            let dt2: LocalResult<DateTime<Utc>> = Utc.from_local_datetime(&dtt);
            Some(dt2.unwrap())
        }
        Err(_e) => None,
    }
}

fn si_time_error(first: &Define) -> Option<SiError> {
    let datetime = first.get_event().get_starttime()?;
    let date = format!("{}", datetime.date_naive().format("%Y-%m-%d"));
    let begin = create_time(format! {"{}T08:00:00.000Z", date}.as_str())?;
    let end = create_time(format! {"{}T20:00:00.000Z", date}.as_str())?;
    if !(begin <= datetime && datetime <= end) {
        Some(SiError::Under5)
    } else {
        None
    }
}

impl SiError {
    pub fn determine(first: &Define, second: &Define, cmd: &Commandline) -> Self {
        let endtime = first.get_event().get_endtime();
        let starttime = second.get_event().get_starttime();
        let dendtime = first.get_event().get_dendtime();
        let dstarttime = second.get_event().get_dstarttime();

        if first.get_event().get_duration() < cmd.minimum()
            && first.get_event().get_displayed_duration(cmd) < cmd.minimum()
        {
            match si_time_error(first) {
                Some(err) => return err,
                None => return SiError::NoError,
            }
        } else if dendtime == dstarttime && endtime == starttime {
            return SiError::NoError;
        } else {
            return SiError::SomeError(
                if endtime > starttime {
                    Box::new(SiError::Overlap)
                } else if endtime < starttime {
                    Box::new(SiError::Gap)
                } else {
                    Box::new(SiError::NoError)
                },
                if dendtime > dstarttime {
                    Box::new(SiError::Overlap)
                } else if dendtime < dstarttime {
                    Box::new(SiError::Gap)
                } else {
                    Box::new(SiError::NoError)
                },
            );
        }
    }
}

impl Define {
    pub fn get_si_error(&self, next: &Define, cmd: &Commandline) -> SiError {
        match self {
            Define::siEvent(..) => match next {
                Define::siEvent(..) => SiError::determine(self, &next, cmd),
                _ => SiError::NoError,
            },
            _ => SiError::NoError,
        }
    }

    pub fn print_si_events_verbose(
        &self,
        other: &Define,
        err: &Box<SiError>,
        display_err: &Box<SiError>,
        verbose: bool,
        utc: bool,
    ) {
        match self {
            Define::siEvent(event1) => match other {
                Define::siEvent(event2) => {
                    event1.print_si_events_verbose(event2, err, display_err, verbose, utc)
                }
                _ => (),
            },
            _ => (),
        }
    }

    pub fn print_va_event_verbose(&self, time_error: &bool, utc: bool, fps: Option<i64>) {
        match self {
            Define::vaEvent(event) => {
                event.print_vaevent_verbose(time_error, utc, fps);
            }
            _ => (),
        }
    }

    pub fn get_event(&self) -> &Event {
        match self {
            Define::vaEvent(ref event)
            | Define::siEvent(ref event)
            | Define::logoEvent(ref event)
            | Define::layoutEvent(ref event) => event,
        }
    }

    pub fn get_event_mut(&mut self) -> &mut Event {
        match self {
            Define::vaEvent(ref mut event)
            | Define::siEvent(ref mut event)
            | Define::logoEvent(ref mut event)
            | Define::layoutEvent(ref mut event) => event,
        }
    }
}
