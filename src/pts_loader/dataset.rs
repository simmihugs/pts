use super::define::*;
use crate::pts_loader::block::Block;
use crate::pts_loader::special_event::SpecialEvent;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DataSet {
    filename: Option<String>,

    #[serde(rename = "eventCommands")]
    eventcommands: EventCommands,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EventCommands {
    #[serde(rename = "DEFINE")]
    define: Vec<Define>,
}

fn load_file(filename: &str) -> std::io::Result<String> {
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

    #[allow(unused_variables)]
    fn look_for_illegals_va_events(&self, events: &Vec<&Define>, verbose: bool, utc: bool) {
        let va_events: Vec<_> = events
            .iter()
            .filter(|x| {
                if let Define::vaEvent(..) = x {
                    return true;
                }
                return false;
            })
            .collect();
        let length = va_events.len();
        println!(
            "{:3} illegal vaEvents found",
            if length == 0 {
                format!("{}", length).green()
            } else {
                format!("{}", length).red()
            }
        );
        if verbose {
            va_events.iter().for_each(|x| println!("{:?}", x));
        }
    }

    #[allow(unused_variables)]
    fn look_for_illegals_si_events(&self, events: &Vec<&Define>, verbose: bool, utc: bool) {
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
            println!(
                "{:3} illegal siEvents found",
                if 0 == si_events.len() {
                    format!("{}", si_events.len()).green()
                } else {
                    format!("{}", si_events.len()).red()
                }
            );
            if verbose {
                si_events.iter().for_each(|x| println!("{:?}", x));
            }
        }
    }

    #[allow(unused_variables)]
    fn look_for_illegals_logo_events(&self, events: &Vec<&Define>, verbose: bool, utc: bool) {
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
            println!("{} illegal logoEvents found", logo_events.len());
            if verbose {
                logo_events.iter().for_each(|x| println!("{:?}", x));
            }
        }
    }

    #[allow(unused_variables)]
    fn look_for_illegals_layout_events(&self, events: &Vec<&Define>, verbose: bool, utc: bool) {
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
            println!("{} illegal layoutEvents found", layout_events.len());
            if verbose {
                layout_events.iter().for_each(|x| println!("{:?}", x));
            }
        }
    }

    #[allow(dead_code)]
    pub fn look_for_illegals(&self, illegals: &Vec<String>, verbose: bool, utc: bool) {
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
            println!("{:3} illegal events found.", format!("0").green());
        } else {
            self.look_for_illegals_si_events(events, verbose, utc);
            self.look_for_illegals_va_events(events, verbose, utc);
            self.look_for_illegals_logo_events(events, verbose, utc);
            self.look_for_illegals_layout_events(events, verbose, utc);
        }
    }

    fn print_si_error_verbose(
        &self,
        err: Box<SiError>,
        display_err: Box<SiError>,
        event: &Define,
        next_event: &Define,
        verbose: bool,
        utc: bool,
    ) {
        event.print_si_events_verbose(next_event, &err, &display_err, verbose, utc);
    }

    pub fn get_si_events(&self) -> Vec<&Define> {
        self.eventcommands
            .define
            .iter()
            .filter(|x| {
                if let Define::siEvent(..) = x {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<&Define>>()
    }

    fn get_special_events(&self) -> (Vec<SpecialEvent>, Vec<Block<'_>>) {
        let special_events = self
            .eventcommands
            .define
            .iter()
            .filter(|x| {
                if let Define::siEvent(..) = x {
                    false
                } else {
                    true
                }
            })
            .collect::<Vec<&Define>>();

        let blocks = special_events
            .iter()
            .enumerate()
            .filter(|(_, &x)| {
                x.get_contentid() == "cb7a119f84cb7b117b1b"
                    || x.get_contentid() == "392654926764849cd5dc"
            })
            .map(|(i, x)| {
                if x.get_contentid() == "cb7a119f84cb7b117b1b" {
                    Block::Begin {
                        index: i as usize,
                        event: x,
                    }
                } else {
                    Block::End {
                        index: i as usize,
                        event: x,
                    }
                }
            })
            .collect::<Vec<Block<'_>>>();
        let mut errors = Vec::new();
        let mut pairs = Vec::new();
        for (i, block) in blocks.iter().enumerate() {
            if block.is_begin() && i + 1 < blocks.len() && blocks[i + 1].is_end() {
                pairs.push((block, &blocks[i + 1]));
            } else if block.is_begin() {
                errors.push(block.clone());
            } else if block.is_end() && i == 0 {
                errors.push(block.clone());
            }
        }

        let mut result = Vec::new();
        for vec in &pairs {
            let (begin, end) = vec;
            result.push(SpecialEvent::new(
                special_events[begin.index()..=end.index()].to_vec(),
            ));
        }

        (result, errors)
    }

    fn print_line(&self, verbose: bool) {
        if verbose {
            println!("|{}|", self.line(158));
        }
    }

    fn line(&self, n: u64) -> String {
        let mut line = String::new();
        for _ in 0..n {
            line += "-";
        }

        line
    }

    fn print_line_cross(&self, verbose: bool) {
        if verbose {
            println!(
                "|{}+{}+{}+{}+{}+{}+{}|",
                self.line(32),
                self.line(17),
                self.line(25),
                self.line(25),
                self.line(14),
                self.line(22),
                self.line(17),
            );
        }
    }

    fn print_head(&self, verbose: bool) {
        if verbose {
            println!(
                "| {:30} | {:15} | {:23} | {:23} | {:12} | {:20} | {:15} |",
                "title", "programid", "start", "end", "duration", "contentid", "logo",
            );
        }
    }

    pub fn write_special_events_csv(
        &self,
        filename: &str,
        encoding: &String,
        utc: bool,
    ) -> std::io::Result<()> {
        use std::env;

        let (special_events, _errors) = &self.get_special_events();
        let mut file = File::create(filename)?;
        match file.write_all(b"title;start;end;duration;contentid;logo;\n") {
            _ => (),
        }
        special_events.iter().for_each(|special_event| {
            if encoding == "windows1252" || encoding.contains("1252") || encoding.contains("win") {
                match self.write_1252(&mut file, &special_event, utc) {
                    Err(e) => println!("{}", e),
                    Ok(..) => (),
                }
            } else if encoding == "utf-8" || encoding.contains("linux") {
                match file.write_all(special_event.to_string(utc).as_bytes()) {
                    Err(e) => println!("{}", e),
                    Ok(..) => (),
                }
            } else if env::consts::OS == "windows" {
                match self.write_1252(&mut file, &special_event, utc) {
                    Err(e) => println!("{}", e),
                    Ok(..) => (),
                }
            } else {
                println!("only available encodings are utf-8 and windows1252\nNo file written.");
            }
        });

        Ok(())
    }

    fn write_1252(
        &self,
        file: &mut File,
        special_event: &SpecialEvent<'_>,
        utc: bool,
    ) -> std::io::Result<()> {
        let text = special_event.to_string(utc);
        let (windows_1252_encoded_string, _, _) = encoding_rs::WINDOWS_1252.encode(&text);

        file.write_all(&windows_1252_encoded_string.as_ref())
    }

    pub fn print_special_events(&self, verbose: bool, utc: bool) {
        let (special_events, errors) = &self.get_special_events();
        let mut id_errors = 0;
        let mut logo_errors = 0;
        self.print_line(verbose);
        self.print_head(verbose && special_events.len() > 0);
        self.print_line_cross(verbose);
        special_events.iter().for_each(|special_event| {
            let (lerrors, ierrors) = special_event.print_table(verbose, utc);
            self.print_line_cross(verbose);
            id_errors += ierrors;
            logo_errors += lerrors;
        });
        self.print_head(verbose && special_events.len() > 0);
        self.print_line(verbose);

        println!(
            "{:3} id errors",
            if id_errors > 0 {
                format!("{}", id_errors).red()
            } else {
                format!("{}", id_errors).green()
            }
        );
        println!(
            "{:3} logo errors",
            if logo_errors > 0 {
                format!("{}", logo_errors).red()
            } else {
                format!("{}", logo_errors).green()
            }
        );
        println!(
            "{:3} special event block errors",
            if errors.len() == 0 {
                format!("{}", 0).green()
            } else {
                format!("{}", errors.len()).red()
            }
        );
        if verbose {
            for block in errors {
                if block.is_begin() {
                    println!("{}", "missing end to event:".red());
                    println!("{:?}", block.event());
                } else {
                    println!("{}", "missing begin to event:".red());
                    println!("{:?}", block.event());
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn print_si_errors(&mut self, verbose: bool, utc: bool) {
        let mut si_events: Vec<&Define> = self.get_si_events();

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
                "{:3} sierrors",
                if 0 == si_errors.len() {
                    nerrors.green()
                } else {
                    nerrors.red()
                }
            );
            for (err, display_err, event, next_event) in si_errors {
                self.print_si_error_verbose(err, display_err, event, next_event, verbose, utc);
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
