use super::define::*;
use crate::commandline::Commandline;
use crate::pts_loader::block::Block;
use crate::pts_loader::special_event::SpecialEvent;
use crate::summary::Summary;
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
            .for_each(|define| define.get_event_mut().calculate_endtime());
    }

    #[allow(unused_variables)]
    fn look_for_illegals_va_events(&self, events: &Vec<&Define>, cmd: &Commandline) {
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
        if cmd.verbose() {
            va_events.iter().for_each(|x| println!("{:?}", x));
        }
    }

    #[allow(unused_variables)]
    fn look_for_illegals_si_events(&self, events: &Vec<&Define>, cmd: &Commandline) {
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
            if cmd.verbose() {
                si_events.iter().for_each(|x| println!("{:?}", x));
            }
        }
    }

    #[allow(unused_variables)]
    fn look_for_illegals_logo_events(&self, events: &Vec<&Define>, cmd: &Commandline) {
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
            if cmd.verbose() {
                logo_events.iter().for_each(|x| println!("{:?}", x));
            }
        }
    }

    #[allow(unused_variables)]
    fn look_for_illegals_layout_events(&self, events: &Vec<&Define>, cmd: &Commandline) {
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
            if cmd.verbose() {
                layout_events.iter().for_each(|x| println!("{:?}", x));
            }
        }
    }

    #[allow(dead_code)]
    pub fn look_for_illegals(&self, illegals: &Vec<String>, cmd: &Commandline) {
        let events: &Vec<&Define> = &self
            .eventcommands
            .define
            .iter()
            .filter(|x| {
                for illegal in illegals {
                    if x.get_event().get_title().contains(illegal) {
                        return true;
                    }
                }
                return false;
            })
            .collect();

        if events.len() == 0 {
            println!("{:3} illegal events found.", format!("0").green());
        } else {
            self.look_for_illegals_si_events(events, cmd);
            self.look_for_illegals_va_events(events, cmd);
            self.look_for_illegals_logo_events(events, cmd);
            self.look_for_illegals_layout_events(events, cmd);
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

    fn get_si_events(&self) -> Vec<&Define> {
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

    fn get_va_events_with_errors(&self) -> Vec<(bool, &Define)> {
        let mut va_events = self
            .eventcommands
            .define
            .iter()
            .filter(|x| {
                if let Define::vaEvent(..) = x {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<&Define>>();

        if va_events.len() >= 1 {
            let mut va_errors = Vec::new();
            let head = va_events[0];
            va_events.drain(1..).into_iter().fold(head, |acc, value| {
                if acc.get_event().get_endtime() != value.get_event().get_starttime() {
                    va_errors.push((true, value));
                } else if acc.get_event().get_contentid().contains("-")
                    && !acc.get_event().get_contentid().contains("WERBUNG")
                {
                    va_errors.push((false, acc));
                }
                value
            });
            va_errors
        } else {
            Vec::new()
        }
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
                x.get_event().get_contentid() == "cb7a119f84cb7b117b1b"
                    || x.get_event().get_contentid() == "392654926764849cd5dc"
            })
            .map(|(i, x)| {
                if x.get_event().get_contentid() == "cb7a119f84cb7b117b1b" {
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
        let mut special_event_errors = Vec::new();
        let mut pairs = Vec::new();
        for (i, block) in blocks.iter().enumerate() {
            if block.is_begin() && i + 1 < blocks.len() && blocks[i + 1].is_end() {
                pairs.push((block, &blocks[i + 1]));
            } else if block.is_begin() {
                special_event_errors.push(block.clone());
            } else if block.is_end() && i == 0 {
                special_event_errors.push(block.clone());
            }
        }

        let mut result = Vec::new();
        for vec in &pairs {
            let (begin, end) = vec;
            result.push(SpecialEvent::new(
                special_events[begin.index()..=end.index()].to_vec(),
            ));
        }

        (result, special_event_errors)
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
        fps: Option<i64>,
    ) -> std::io::Result<()> {
        use std::env;

        let (special_events, _errors) = &self.get_special_events();
        let mut file = File::create(filename)?;
        match file.write_all(b"title;start;end;duration;contentid;logo;\n") {
            _ => (),
        }
        special_events.iter().for_each(|special_event| {
            if encoding == "windows1252" || encoding.contains("1252") || encoding.contains("win") {
                match self.write_1252(&mut file, &special_event, utc, fps) {
                    Err(e) => println!("{}", e),
                    Ok(..) => (),
                }
            } else if encoding == "utf-8" || encoding.contains("linux") {
                match file.write_all(special_event.to_string(utc, fps).as_bytes()) {
                    Err(e) => println!("{}", e),
                    Ok(..) => (),
                }
            } else if env::consts::OS == "windows" {
                match self.write_1252(&mut file, &special_event, utc, fps) {
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
        fps: Option<i64>,
    ) -> std::io::Result<()> {
        let text = special_event.to_string(utc, fps);
        let (windows_1252_encoded_string, _, _) = encoding_rs::WINDOWS_1252.encode(&text);

        file.write_all(&windows_1252_encoded_string.as_ref())
    }

    fn print_header_short(&self) {
        println!("|--------------------------------+-----------------+-------------------------+-------------------------+--------------+----------------------|");
        println!("| title                          | programid       | start                   | end                     | duration     | contentid            |");
        println!("|--------------------------------+-----------------+-------------------------+-------------------------+--------------+----------------------|");
    }

    pub fn print_va_errors(&self, summary: &mut Summary, cmd: &Commandline) {
        let va_events = &self.get_va_events_with_errors();
        summary.va_errors = va_events.len() as i64;
        if summary.va_errors != 0 && cmd.verbose() {
            println!("VaEvent errors:");
            self.print_header_short();
            for (time_error, event) in va_events {
                event.print_va_event_verbose(time_error, cmd.utc(), cmd.fps());
            }
            self.print_header_short();
        }
    }

    pub fn print_special_events(&self, summary: &mut Summary, cmd: &Commandline) {
        let (special_events, special_event_errors) = &self.get_special_events();

        summary.special_event_errors = special_event_errors.len() as i64;
        let mut new_special_events = Vec::new();
        if cmd.only_errors() {
            special_events.iter().for_each(|special_event| {
                if special_event.has_id_errors() || special_event.has_logo_errors() {
                    let event = special_event.clone();
                    new_special_events.push(event);
                }
            });
        } else {
            new_special_events = special_events.to_vec();
        }
        let special_events = new_special_events;

        if special_events.len() > 0 {
            println!("Special events:");
            self.print_line(cmd.verbose());
            self.print_head(cmd.verbose() && special_events.len() > 0);
            self.print_line_cross(cmd.verbose());
            special_events.iter().for_each(|special_event| {
                let terrors = special_event.get_time_errors();
                let (lerrors, ierrors) =
                    special_event.print_table(&terrors, cmd.verbose(), cmd.utc(), cmd.fps());
                self.print_line_cross(cmd.verbose());
                summary.id_errors += ierrors;
                summary.logo_errors += lerrors;
                summary.time_errors += terrors.len() as i64;
            });
            self.print_head(cmd.verbose() && special_events.len() > 0);
            self.print_line(cmd.verbose());
        }

        if cmd.verbose() {
            for block in special_event_errors {
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
    pub fn print_si_errors(&mut self, summary: &mut Summary, cmd: &Commandline) {
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

            summary.si_errors = si_errors.iter().fold(0, |mut acc, value| {
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
            });

            if summary.si_errors > 0 {
                println!("SiEvent errors:");
            }

            for (err, display_err, event, next_event) in si_errors {
                self.print_si_error_verbose(
                    err,
                    display_err,
                    event,
                    next_event,
                    cmd.verbose(),
                    cmd.utc(),
                );
            }
        }
    }

    #[allow(dead_code)]
    pub fn print_missing_text_errors(&mut self, summary: &mut Summary, cmd: &Commandline) {
        for s in self.get_si_events().iter().filter(|event| {
            !(vec![
                "Derzeit keine UHD-Sendung im Programm",
                "Derzeit kein UHD Event",
                "Majestic Nature",
                "Free Fenster",
                "Nachtschleife",
                "Costa Rica",
                "Moglis Jungle Teil 3",
                "Sendepause",
                "Tomorrowland Movie",
                "Tomorrowland 2018 Aftermovie_1",
                "Red Bull Flying Bach",
                "Moglis Jungle Teil 1",
                "Moglis Jungle Teil 2",
                "Der Weg nach oben",
                "The Shot",
                "Schlagerkreuzfahrt",
                "Marco Polo Reisereportage",
                "Marco Polo Reisereportage: Mumbai",
                "African Animals",
                "Makerspace - Paradies der Prototypen",
                "DEMO-HLG-ARTE-2018 v2",
                "Nasa Highlights",
                "Kajaking im Fluss",
                "Autonotizen.de",
                "Ferrari - The Big 5",
                "Daytona",
                "DLXM Session: Madeleine Juno",
                "DLXM Session: Michael Schulte",
                "DXLM Session: Elif",
                "DLXM Session: Tim Bendzko",
                "DLXM Session: Max Giesinger",
                "DLXM Session: Lea",
                "DLXM Session: Bausa",
                "DLXM Session: Wincent Weis",
                "DXLM Session: Clueso",
                "DLXM Session: James Blunt",
                "DLXM Session: Sportfreunde Stiller",
                "DLXM Session: Malik Harris",
                "DLXM Session: Freya Ridings",
            ]
            .iter()
            .map(|x| x.to_string() == event.get_event().get_title())
            .fold(false, |acc, value| acc || value))
                && (match event.get_event().get_text() {
                    None => false,
                    Some(text) => text == "",
                })
        }) {
            summary.text_error += 1;
            if cmd.verbose() {
                println!("{:?}\n", s);
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
