use serde::{Deserialize, Serialize};
use std::fmt;

use super::event::*;

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, PartialEq)]
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

impl Define {
    pub fn calculate_endtime(&mut self) {
        match self {
            Define::vaEvent(ref mut event) => event.calculate_endtime(),
            Define::siEvent(ref mut event) => event.calculate_endtime(),
            Define::logoEvent(ref mut event) => event.calculate_endtime(),
            Define::layoutEvent(ref mut event) => event.calculate_endtime(),
        }
    }
}
