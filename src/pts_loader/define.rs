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
        let endtime = first.get_event().get_endtime();
        let starttime = second.get_event().get_starttime();
        let dendtime = first.get_event().get_dendtime();
        let dstarttime = second.get_event().get_dstarttime();

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
