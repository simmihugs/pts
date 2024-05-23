use super::define::*;
use crate::commandline::commandline::{Commandline, Range};
use crate::commandline::summary::Summary;
use crate::pts_loader::block::Block;
use crate::pts_loader::special_event::SpecialEvent;
use crate::utils::table_print;
use crate::utils::take::Take;
use crate::Fluid;
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
    let mut file = File::open(filename)?;
    let mut content = String::from("");
    file.read_to_string(&mut content)?;

    Ok(content)
}

impl DataSet {
    pub fn init_from_data(xml_text: String) -> Result<DataSet, serde_xml_rs::Error> {
        let mut dataset: DataSet = serde_xml_rs::from_str(&xml_text)?;
        dataset.calculate_endtimes();
        Ok(dataset)
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

    #[allow(unused_variables, dead_code)]
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
            println!("{}", "Illegal events:");
            va_events.iter().for_each(|x| println!("{:?}", x));
        } else {
            va_events.iter().for_each(|x| println!("{}", x));
        }
    }

    #[allow(unused_variables, dead_code)]
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
            } else {
                si_events.iter().for_each(|x| println!("{}", x));
            }
        }
    }

    #[allow(unused_variables, dead_code)]
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
            } else {
                logo_events.iter().for_each(|x| println!("{}", x));
            }
        }
    }

    #[allow(unused_variables, dead_code)]
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
            } else {
                layout_events.iter().for_each(|x| println!("{}", x));
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

    // fn print_line(&self, verbose: bool) {
    //     if verbose {
    //         println!("|{}|", self.line(158 + 53));
    //     }
    // }

    // fn line(&self, n: u64) -> String {
    //     let mut line = String::new();
    //     for _ in 0..n {
    //         line += "-";
    //     }

    //     line
    // }

    // fn print_line_cross(&self, verbose: bool) {
    //     if verbose {
    //         println!(
    //             "|{}+{}+{}+{}+{}+{}+{}+{}|",
    //             table_print::line(32),
    //             table_print::line(52),
    //             table_print::line(17),
    //             table_print::line(25),
    //             table_print::line(25),
    //             table_print::line(14),
    //             table_print::line(22),
    //             table_print::line(17),
    //         );
    //     }
    // }

    // fn print_head(&self, verbose: bool) {
    //     if verbose {
    //         println!(
    //             "| {:30} | {:50} | {:15} | {:23} | {:23} | {:12} | {:20} | {:15} |",
    //             "title", "filename", "programid", "start", "end", "duration", "contentid", "logo",
    //         );
    //     }
    // }

    pub fn update_werbungen(&self, cmd: &Commandline) -> std::io::Result<String> {
        let new_filename = format!("{}", cmd.filename().replace(".\\", ""));
        let (special_events, _) = &self.get_special_events();
        let werbungen_liste: Vec<_> = special_events.iter().map(|e| e.get_werbungen()).collect();
        if !werbungen_liste.is_empty() {
            let mut source = File::open(cmd.filename())?;
            let mut data = String::new();
            source.read_to_string(&mut data)?;
            drop(source);

            for werbungen in werbungen_liste {
                if !werbungen.is_empty() {
                    for werbung in werbungen {
                        let contentid = "UHD1_WERBUNG-01";
                        let newtitle = werbung
                            .replace(contentid, "")
                            .replace(" ", "")
                            .replace("-", "");
                        let oldstr = format!("\r\n\t\t\t\ttitle=\"{}\"", werbung);
                        let newstr = format!("\r\n\t\t\t\ttitle=\"{}\"", newtitle);

                        data = data.replace(&*oldstr, &*newstr);
                    }
                }
            }
            let mut dest = File::create(&new_filename)?;
            dest.write(data.as_bytes())?;
            drop(dest);
        }

        Ok(new_filename)
    }

    pub fn write_special_events_csv(
        &self,
        cmd: &Commandline,
        fluid_data_set: &Fluid,
    ) -> std::io::Result<()> {
        use std::env;

        let (special_events, _errors) = &self.get_special_events();
        let mut file = File::create(cmd.csv())?;
        match file.write_all(b"title;filename;start;end;duration;contentid;logo;\n") {
            _ => (),
        }
        special_events.iter().for_each(|special_event| {
            if cmd.encoding() == "windows1252"
                || cmd.encoding().contains("1252")
                || cmd.encoding().contains("win")
            {
                match self.write_1252(&mut file, &special_event, cmd, fluid_data_set) {
                    Err(e) => println!("{}", e),
                    Ok(..) => (),
                }
            } else if cmd.encoding() == "utf-8" || cmd.encoding().contains("linux") {
                match file.write_all(special_event.to_string(cmd, fluid_data_set).as_bytes()) {
                    Err(e) => println!("{}", e),
                    Ok(..) => (),
                }
            } else if env::consts::OS == "windows" {
                match self.write_1252(&mut file, &special_event, cmd, fluid_data_set) {
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
        cmd: &Commandline,
        fluid_data_set: &Fluid,
    ) -> std::io::Result<()> {
        let text = special_event.to_string(cmd, fluid_data_set);
        let (windows_1252_encoded_string, _, _) = encoding_rs::WINDOWS_1252.encode(&text);

        file.write_all(&windows_1252_encoded_string.as_ref())
    }

    // fn missing_text_header(&self) {
    //     let len = 122;
    //     println!("|{}|", "-".repeat(len as usize));
    //     println!(
    //         "| {} | {} | {} | {} |",
    //         String::from("title").take(50),
    //         String::from("progarmid").take(15),
    //         String::from("start").take(23),
    //         String::from("end").take(23)
    //     );
    //     println!("|{}|", "-".repeat(len as usize));
    // }

    pub fn print_va_errors(&self, summary: &mut Summary, cmd: &Commandline) {
        let va_events = &self.get_va_events_with_errors();
        summary.va_errors = va_events.len() as i64;
        if summary.va_errors != 0 && cmd.verbose() {
            println!("VaEvent errors:");
            table_print::print_header_short();
            for (time_error, event) in va_events {
                event.print_va_event_verbose(time_error, cmd.utc(), cmd.fps());
            }
            table_print::print_header_short();
        }
    }

    pub fn print_special_events(
        &self,
        summary: &mut Summary,
        cmd: &Commandline,
        fluid_data_set: &Fluid,
    ) {
        let (special_events, special_event_errors) = &self.get_special_events();
        summary.special_event_errors = special_event_errors.len() as i64;
        let mut new_special_events = Vec::new();
        if cmd.only_errors() {
            special_events.iter().for_each(|special_event| {
                if special_event.has_id_errors() || special_event.has_logo_errors(cmd) {
                    let event = special_event.clone();
                    new_special_events.push(event);
                }
            });
        } else {
            new_special_events = special_events.to_vec();
        }
        let special_events = new_special_events;

        if special_events.len() > 0 && cmd.verbose() {
            println!("Special events:");
            table_print::print_line(158 + 53);
            table_print::print_head();
            table_print::print_line_cross();
            special_events.iter().for_each(|special_event| {
                let terrors = special_event.get_time_errors();
                let (lerrors, ierrors, length_errors) =
                    special_event.print_table(&terrors, cmd, fluid_data_set);
                table_print::print_line_cross();
                summary.id_errors += ierrors;
                summary.logo_errors += lerrors;
                summary.time_errors += terrors.len() as i64;
                summary.length_error += length_errors;
            });
            table_print::print_head();
            table_print::print_line(158 + 53);
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
            let mut length_errors = Vec::new();
            let mut si_errors = Vec::new();
            let head = si_events[0];
            si_events.drain(1..).into_iter().fold(head, |acc, value| {
                match acc.get_si_error(value, cmd) {
                    SiError::SomeError(err, display_err) => {
                        si_errors.push((err, display_err, acc, value));
                        value
                    }
                    SiError::Under5 => {
                        length_errors.push(acc);
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

            if cmd.verbose() {
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

            if cmd.verbose() {
                for err in &length_errors {
                    println!("{:?}", err);
                }
            }

            summary.si_length_error = length_errors.len() as i64;
        }
    }

    #[allow(dead_code)]
    pub fn print_missing_text_errors(&mut self, summary: &mut Summary, cmd: &Commandline) {
        let mut store = Vec::new();
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
                let event = s.get_event();
                store.push(format!(
                    "| {} | {} | {} | {} |",
                    event.get_title().take(50),
                    event.get_programid().take(15),
                    event.starttime_to_string(cmd.utc(), cmd.fps()).take(23),
                    event.endtime_to_string(cmd.utc(), cmd.fps()).take(23)
                ));
            }
        }
        if store.len() > 0 && cmd.verbose() {
            let len = 122;
            println!("{}", "Missings texts:".red());
            table_print::missing_text_header();
            store.iter().enumerate().for_each(|(i, x)| {
                println!("{}", x);
                if i < store.len() - 1 {
                    println!("|{}|", "-".repeat(len as usize));
                }
            });
            table_print::missing_text_header();
        }
    }

    pub fn print_range(&mut self, range: &Range) {
        for s in self.get_si_events().iter().filter(|event| {
            let e = event.get_event();
            match e.get_starttime() {
                Some(start) => range.start_time < start && start <= range.end_time,
                _ => false,
            }
        }) {
            println!("{:?}\n", s);
        }
    }
}
