use super::{define::*, special_event};
use crate::commandline::commandline::Commandline;
use crate::commandline::summary::Summary;
use crate::pts_loader::block::Block;
use crate::pts_loader::event::Event;
use crate::pts_loader::sistandard::a_duration_from_string;
use crate::pts_loader::special_event::SpecialEvent;
use crate::utils::fluid::QueryType;
use crate::utils::table_print::{self, print_line};
use crate::utils::take::Take;
use crate::Fluid;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
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
    fs::read_to_string(filename)
}

impl DataSet {
    pub fn list_vaevents_with_length_errors(
        &self,
        summary: &mut Summary,
        cmd: &Commandline,
        fluid_data_set: &Fluid,
    ) {
        let vaevents: Vec<_> = self
            .eventcommands
            .define
            .iter()
            .filter(|x| {
                if let Define::vaEvent(..) = x {
                    return true;
                } else {
                    return false;
                }
            })
            .map(|event| event.get_event())
            .collect();

        let mut content_length_errors = vec![];

        for event in vaevents {
            match fluid_data_set.query(&event, QueryType::Duration) {
                None => (),
                Some(duration) => {
                    let event_duration: i64 = event.get_duration();
                    let dbase_duration: i64 = match a_duration_from_string(duration) {
                        Ok(i) => i,
                        Err(..) => 0,
                    };
                    if event_duration > dbase_duration {
                        content_length_errors.push((event, dbase_duration));
                        /*                         summary.content_to_long_error += 1;
                                               println!(
                                                   "ERROR: Title: {:?} Id: {:?} has duration: {:?} > fluid_duration: {}",
                                                   event.get_contentid(),
                                                   event.get_title(),
                                                   Event::a_duration_to_string(event_duration, cmd.fps()),
                                                   Event::a_duration_to_string(dbase_duration, cmd.fps()),
                                               )
                        */
                    }
                }
            }
        }

        if content_length_errors.len() > 0 {
            let length = 182;
            println!(
                "\n{}:\n|{}|",
                "Content length error".to_string().red(),
                "-".repeat(length)
            );
            println!(
                "| {} | {} | {} | {} | {} | {} |",
                "title".to_string().take(40),
                "starttime".to_string().take(30),
                "programid".to_string().take(25),
                "id".to_string().take(20),
                "duration".to_string().take(20),
                "database duration".to_string().take(30)
            );
            println!("|{}|", "-".repeat(length));
            for (event, dbase_duration) in content_length_errors.iter() {
                println!(
                    "| {} | {} | {} | {} | {} | {} |",
                    event.get_title().to_string().take(40),
                    event.starttime_to_string(cmd.utc(), cmd.fps()).take(30),
                    event.get_programid().to_string().take(25),
                    event.get_contentid().to_string().take(20),
                    Event::a_duration_to_string(event.get_duration(), cmd.fps())
                        .to_string()
                        .take(20)
                        .red(),
                    Event::a_duration_to_string(*dbase_duration, cmd.fps())
                        .to_string()
                        .take(30)
                );
                println!("|{}|", "-".repeat(length));
            }
            println!("");
        }
        summary.content_to_long_error += content_length_errors.len();
    }

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

    fn get_si_events(&self) -> SiEvents {
        SiEvents {
            events: self
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
                .collect::<Vec<&Define>>(),
        }
    }

    pub fn display_sievents(&self, cmd: &Commandline) {
        let mut si_events = self.get_si_events();
        si_events.print(cmd);
    }

    pub fn display_trailers(&self, cmd: &Commandline) {
        let mut events = SiEvents {
            events: self
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
                .filter(|x| x.get_event().get_duration() <= 50000)
                .collect(),
        };
        if events.len() > 0 {
            events.print(cmd);
        }
    }

    pub fn display_all_content_id_errors(&self, summary: &mut Summary, cmd: &Commandline) {
        let mut events = SiEvents {
            events: self
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
                .filter(|x| {
                    let id = x.get_event().get_contentid();
                    let title = x.get_event().get_title();
                    let id_error: bool = (id.contains("-") && "1529410-0".len() == id.len())
                        || ((id.contains("-") && !title.starts_with(" - 00"))
                            && (id.contains("-") && !title.starts_with(" - 00"))
                            && (id.contains("-") && "1529410-0".len() == id.len())
                            && !title.split(" ").collect::<Vec<&str>>()[0]
                                .to_string()
                                .parse::<i64>()
                                .is_ok()
                            && !id.contains("WERBUNG"));

                    id_error
                })
                .collect(),
        };

        if cmd.debug() {
            events.print(cmd);
            events.events.iter().for_each(|x| println!("{:?}", x));
        }
        //summary.invalid_content_id_error = events.events.len();
        summary.id_errors = events.events.len() as i64;
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
                let id = acc.get_event().get_contentid();
                let title = acc.get_event().get_title();
                let id_error = (id.contains("-") && !title.starts_with(" - 00"))
                    && !title.split(" ").collect::<Vec<&str>>()[0]
                        .to_string()
                        .parse::<i64>()
                        .is_ok()
                    && !id.contains("WERBUNG");
                if acc.get_event().get_endtime() != value.get_event().get_starttime() {
                    va_errors.push((true, value));
                } else if id_error {                    
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

    pub fn update_commercials(&self, cmd: &Commandline) -> std::io::Result<String> {
        let new_filename = format!("{}", cmd.filename().replace(".\\", ""));
        let (special_events, _) = &self.get_special_events();
        let commercials_liste: Vec<_> =
            special_events.iter().map(|e| e.get_commercials()).collect();
        if !commercials_liste.is_empty() {
            let mut source = File::open(cmd.filename())?;
            let mut data = String::new();
            source.read_to_string(&mut data)?;
            drop(source);

            for commercials in commercials_liste {
                if !commercials.is_empty() {
                    for commercial in commercials {
                        let oldstr = format!("\r\n\t\t\t\ttitle=\"{}\"", commercial);

                        // TODO update new commercials
                        if commercial.starts_with(" - 00") {
                            let newstr = format!(
                                "\r\n\t\t\t\ttitle=\"{}\"",
                                commercial.replace(" - 00", "00")
                            );
                            data = data.replace(&*oldstr, &*newstr);
                        } else {
                            let newstr = format!(
                                "\r\n\t\t\t\ttitle=\"{}\"",
                                commercial
                                    .replace("UHD1_WERBUNG-01", "")
                                    .replace(" ", "")
                                    .replace("-", "")
                            );
                            data = data.replace(&*oldstr, &*newstr);
                        }
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
        match file.write_all(b"title;filename;start;end;duration;tcin;tcout;contentid;logo;\n") {
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

    #[allow(dead_code)]
    pub fn trailers_and_balls_mixup(&self, _summary: &mut Summary, cmd: &Commandline) {
        let events: Vec<_> = self
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

        events.iter().enumerate().for_each(|(i, x)| {
            let event = x.get_event();
            if event.get_contentid().contains("5675d8c63df2424bf286") {
                let before = events[i - 1].get_event();
                let after = events[i + 1].get_event();

                if 10 * 1_000 < before.get_duration() && before.get_duration() < 30 * 1_000 {
                    //before is trailer
                } else if 3 * 60_000 < before.get_duration() {
                    // before is segment
                } else {
                }
                println!("{}", before.get_duration());
                //                assert!(before.get_duration() < 30 * 1_000);

                print_line(100);
                println!(
                    "| {} | {} | {} |",
                    before.get_title().take(50),
                    before.starttime_to_string(cmd.utc(), cmd.fps()),
                    before.duration_to_string(cmd.fps()).take(20),
                );

                print_line(100);
                println!(
                    "| {} | {} | {} |",
                    event.get_title().take(50).blue(),
                    event.starttime_to_string(cmd.utc(), cmd.fps()).blue(),
                    event.duration_to_string(cmd.fps()).take(20).blue(),
                );

                print_line(100);

                println!(
                    "| {} | {} | {} |",
                    after.get_title().take(50),
                    after.starttime_to_string(cmd.utc(), cmd.fps()),
                    after.duration_to_string(cmd.fps()).take(20),
                );

                print_line(100);
                println!("");
            }
        });
    }

    pub fn print_va_errors(&self, summary: &mut Summary, cmd: &Commandline) {
        let va_events = &self.get_va_events_with_errors();
        summary.va_errors = va_events
            .iter()
            .filter(|(time, _)| *time)
            .map(|_| 1 as usize)
            .collect::<Vec<usize>>()
            .len() as i64;
        summary.id_errors = va_events
            .iter()
            .filter(|(time, _)| !time)
            .map(|_| 1 as usize)
            .collect::<Vec<usize>>()
            .len() as i64;

        if (summary.va_errors != 0 || summary.id_errors != 0) && cmd.verbose() {
            println!("VaEvent errors and id errors:");
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
        if cmd.verbose() {
            let (special_events, special_event_errors) = &self.get_special_events();

            summary.special_event_errors = special_event_errors.len() as i64;
            let special_events: Vec<&SpecialEvent<'_>> = special_events
                .iter()
                .filter(|x| {
                    if cmd.only_errors() {
                        if x.has_id_errors() || x.has_logo_errors(cmd) {
                            true
                        } else {
                            false
                        }
                    } else {
                        true
                    }
                })
                .collect::<Vec<_>>();

            // TODO hier ist der wurm drin
            special_event::print_special_events(
                special_events,
                special_event_errors,
                fluid_data_set,
                summary,
                cmd,
            );
        }
    }

    pub fn print_si_errors(&mut self, summary: &mut Summary, cmd: &Commandline) {
        let mut si_events: Vec<&Define> = self.get_si_events().events;

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

    // TODO move to external file
    // TODO astra logo no real error
    pub fn print_missing_text_errors(&mut self, summary: &mut Summary, cmd: &Commandline) {
        let mut store = Vec::new();
        for s in self.get_si_events().events.iter().filter(|event| {
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
                "DLXM Session: Loi",
            ]
            .iter()
            .map(|x| {
                x.to_string() == event.get_event().get_title()
                    || event.get_event().get_title().contains("Olympia")
            })
            .fold(false, |acc, value| acc || value))
                && (match event.get_event().get_text() {
                    None => {
                        return false;
                    }
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
}
