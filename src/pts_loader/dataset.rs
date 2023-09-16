use super::define::*;
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DataSet {
    filename: Option<String>,

    #[serde(rename = "eventCommands")]
    eventcommands: EventCommands,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EventCommands {
    #[serde(rename = "DEFINE")]
    define: Vec<Define>,
}

fn load_file(filename: &str) -> std::io::Result<String> {
    use std::fs::File;
    use std::io::Read;
    match File::open(filename) {
        Err(e) => Err(e),
        Ok(mut file) => {
            let mut s = String::new();
            file.read_to_string(&mut s)?;

            Ok(s)
        }
    }
}

impl DataSet {
    #[allow(dead_code)]
    pub fn get_filename(&self) -> &str {
        match &self.filename {
            None => "None",
            Some(_str) => _str,
        }
    }

    pub fn init_from_data(xml_text: String) -> Result<DataSet, serde_xml_rs::Error> {
        let maybe_dataset: Result<DataSet, serde_xml_rs::Error> = serde_xml_rs::from_str(&xml_text);
        match maybe_dataset {
            Ok(mut dataset) => {
                dataset.calculate_endtimes();
                Ok(dataset)
            }
            Err(e) => Err(e),
        }
    }

    pub fn init(filename: &str) -> Result<DataSet, serde_xml_rs::Error> {
        match load_file(filename) {
            Err(e) => {
                let err = serde_xml_rs::Error::Custom {
                    field: format!("{}", e),
                };
                Err(err)
            }
            Ok(xml_text) => match DataSet::init_from_data(xml_text) {
                Ok(mut dataset) => {
                    dataset.filename = Some(filename.to_string());
                    Ok(dataset)
                }
                err => err,
            },
        }
    }

    fn calculate_endtimes(&mut self) {
        self.eventcommands
            .define
            .iter_mut()
            .for_each(|define| define.calculate_endtime());
    }

    fn look_for_illegals_va_events(&self, events: &Vec<&Define>, verbose: bool) {
        let va_events: Vec<_> = events
            .iter()
            .filter(|x| {
                if let Define::vaEvent(..) = x {
                    return true;
                }
                return false;
            })
            .collect();
        if va_events.len() > 0 {
            println!("{} VaEvents", va_events.len());
            if verbose {
                va_events.iter().for_each(|x| println!("{:?}", x));
            }
        }
    }
    fn look_for_illegals_si_events(&self, events: &Vec<&Define>, verbose: bool) {
        let si_events: Vec<_> = events
            .iter()
            .filter(|x| {
                if let Define::siEvent(..) = x {
                    return true;
                }
                return false;
            })
            .collect();
        if si_events.len() > 0 {
            println!("{} SiEvents", si_events.len());
            if verbose {
                si_events.iter().for_each(|x| println!("{:?}", x));
            }
        }
    }
    fn look_for_illegals_logo_events(&self, events: &Vec<&Define>, verbose: bool) {
        let logo_events: Vec<_> = events
            .iter()
            .filter(|x| {
                if let Define::logoEvent(..) = x {
                    return true;
                }
                return false;
            })
            .collect();
        if logo_events.len() > 0 {
            println!("{} LogoEvents", logo_events.len());
            if verbose {
                logo_events.iter().for_each(|x| println!("{:?}", x));
            }
        }
    }
    fn look_for_illegals_layout_events(&self, events: &Vec<&Define>, verbose: bool) {
        let layout_events: Vec<_> = events
            .iter()
            .filter(|x| {
                if let Define::layoutEvent(..) = x {
                    return true;
                }
                return false;
            })
            .collect();
        if layout_events.len() > 0 {
            println!("{} LayoutEvents", layout_events.len());
            if verbose {
                layout_events.iter().for_each(|x| println!("{:?}", x));
            }
        }
    }

    pub fn look_for_illegals(&self, illegals: &Vec<String>, verbose: bool) {
        let events: &Vec<&Define> = &self
            .eventcommands
            .define
            .iter()
            .filter(|x| {
                for illegal in illegals {
                    if x.get_title().contains(illegal) {
                        return true;
                    }
                }
                return false;
            })
            .collect();

        if events.len() == 0 {
            println!("0 events found.");
        } else {
            self.look_for_illegals_si_events(events, verbose);
            self.look_for_illegals_va_events(events, verbose);
            self.look_for_illegals_logo_events(events, verbose);
            self.look_for_illegals_layout_events(events, verbose);	    
        }
    }

    fn print_si_error_verbose(
        &self,
        err: Box<SiError>,
        display_err: Box<SiError>,
        event: &Define,
        next_event: &Define,
        verbose: bool,
    ) {
        event.print_si_events_verbose(next_event, &err, &display_err, verbose);
    }

    #[allow(dead_code)]
    pub fn print_si_errors(&self, verbose: bool) {
        let si_events: &mut Vec<&Define> = &mut self
            .eventcommands
            .define
            .iter()
            .filter(|x| {
                if let Define::siEvent(..) = x {
                    true
                } else {
                    false
                }
            })
            .collect();

        if si_events.len() > 1 {
            let mut si_errors = Vec::new();
            let head = si_events[0];
            si_events.drain(1..).into_iter().fold(head, |acc, value| {
                match acc.get_si_error(value) {
                    SiError::SomeError(err, display_err) => {
                        si_errors.push((err, display_err, acc, value));
                        value
                    }
                    _ => value,
                }
            });

            let nerrors = format!(
                "{}",
                &si_errors.iter().fold(0, |mut acc, value| {
                    let (err, display_err, _, _) = value;

                    if let SiError::Overlap = **err {
                        acc += 1;
                    } else if let SiError::Gap = **err {
                        acc += 1;
                    }

                    if let SiError::Gap = **display_err {
                        acc += 1;
                    } else if let SiError::Overlap = **display_err {
                        acc += 1;
                    }

                    acc
                })
            );

            println!(
                "{} sierrors",
                if 0 == si_errors.len() {
                    nerrors.green()
                } else {
                    nerrors.red()
                }
            );
            for (err, display_err, event, next_event) in si_errors {
                self.print_si_error_verbose(err, display_err, event, next_event, verbose);
            }
        }
    }

    fn print_n_events(&self, all: bool, _define: &str, n: u64) {
        let mut i = 0;
        for define in self.eventcommands.define.iter() {
            if i < n || all {
                match define {
                    Define::siEvent(..) => {
                        if _define == "siEvent" {
                            i += 1;
                            println!("{:?}", define)
                        }
                    }
                    Define::vaEvent(..) => {
                        if _define == "vaEvent" {
                            i += 1;
                            println!("{:?}", define)
                        }
                    }
                    Define::logoEvent(..) => {
                        if _define == "logoEvent" {
                            i += 1;
                            println!("{:?}", define)
                        }
                    }
                    Define::layoutEvent(..) => {
                        if _define == "layoutEvent" {
                            i += 1;
                            println!("{:?}", define)
                        }
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn print_si(&self) {
        self.print_n_events(true, "siEvent", 0);
    }

    #[allow(dead_code)]
    pub fn print_va(&self) {
        self.print_n_events(true, "vaEvent", 0);
    }

    #[allow(dead_code)]
    pub fn print_layout(&self) {
        self.print_n_events(true, "layoutEvent", 0);
    }

    #[allow(dead_code)]
    pub fn print_logo(&self) {
        self.print_n_events(true, "logoEvent", 0);
    }

    #[allow(dead_code)]
    pub fn print_n_si(&self, n: u64) {
        self.print_n_events(false, "siEvent", n);
    }
}
