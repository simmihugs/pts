use chrono::{DateTime, Utc};
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

#[derive(PartialEq)]
pub enum SiError {
    NoError,
    Overlap,
    Gap,
    SomeError(Box<SiError>, Box<SiError>),
}

impl SiError {
    pub fn determine(first: &Define, second: &Define) -> Self {
        let endtime = first.get_endtime();
        let starttime = second.get_starttime();
        let dendtime = first.get_dendtime();
        let dstarttime = second.get_dstarttime();

        if dendtime == dstarttime && endtime == starttime {
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
    pub fn get_si_error(&self, next: &Define) -> SiError {
        match self {
            Define::siEvent(..) => match next {
                Define::siEvent(..) => SiError::determine(self, &next),
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

    pub fn get_title(&self) -> String {
        match self {
            Define::vaEvent(ref event) => event.get_title(),
            Define::siEvent(ref event) => event.get_title(),
            Define::logoEvent(ref event) => event.get_title(),
            Define::layoutEvent(ref event) => event.get_title(),
        }
    }

    pub fn get_logo(&self) -> String {
        match self {
            Define::logoEvent(ref event) => event.get_logo(),
            Define::layoutEvent(ref event) => event.get_logo(),
            _ => panic!("No logo"),
        }
    }

    pub fn get_contentid(&self) -> String {
        match self {
            Define::vaEvent(ref event) => event.get_contentid(),
            Define::siEvent(ref event) => event.get_contentid(),
            Define::logoEvent(ref event) => event.get_contentid(),
            Define::layoutEvent(ref event) => event.get_contentid(),
        }
    }

    pub fn get_dendtime(&self) -> Option<DateTime<Utc>> {
        match self {
            Define::vaEvent(ref event) => event.get_dendtime(),
            Define::siEvent(ref event) => event.get_dendtime(),
            Define::logoEvent(ref event) => event.get_dendtime(),
            Define::layoutEvent(ref event) => event.get_dendtime(),
        }
    }

    pub fn get_dstarttime(&self) -> Option<DateTime<Utc>> {
        match self {
            Define::vaEvent(ref event) => event.get_dstarttime(),
            Define::siEvent(ref event) => event.get_dstarttime(),
            Define::logoEvent(ref event) => event.get_dstarttime(),
            Define::layoutEvent(ref event) => event.get_dstarttime(),
        }
    }

    pub fn get_endtime(&self) -> Option<DateTime<Utc>> {
        match self {
            Define::vaEvent(ref event) => event.get_endtime(),
            Define::siEvent(ref event) => event.get_endtime(),
            Define::logoEvent(ref event) => event.get_endtime(),
            Define::layoutEvent(ref event) => event.get_endtime(),
        }
    }

    pub fn get_starttime(&self) -> Option<DateTime<Utc>> {
        match self {
            Define::vaEvent(ref event) => event.get_starttime(),
            Define::siEvent(ref event) => event.get_starttime(),
            Define::logoEvent(ref event) => event.get_starttime(),
            Define::layoutEvent(ref event) => event.get_starttime(),
        }
    }

    pub fn calculate_endtime(&mut self) {
        match self {
            Define::vaEvent(ref mut event) => event.calculate_endtime(),
            Define::siEvent(ref mut event) => event.calculate_endtime(),
            Define::logoEvent(ref mut event) => event.calculate_endtime(),
            Define::layoutEvent(ref mut event) => event.calculate_endtime(),
        }
    }
}
