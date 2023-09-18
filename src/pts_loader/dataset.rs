use super::define::*;
use crate::pts_loader::event::Event;
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

pub struct SpecialEvent<'a> {
    vec: Vec<&'a Define>,
}

impl<'a> SpecialEvent<'a> {
    pub fn new(vec: Vec<&'a Define>) -> Self {
        Self { vec }
    }

    fn find_logo(&self, event: &Event) -> Vec<&Define> {
        let mut logos = Vec::new();

        let layout_events: Vec<_> = self
            .vec
            .iter()
            .filter(|x| match x {
                Define::layoutEvent(..) => true,
                _ => false,
            })
            .collect();

        let logo_events: Vec<_> = self
            .vec
            .iter()
            .filter(|x| match x {
                Define::logoEvent(..) => true,
                _ => false,
            })
            .collect();

        for layout in &layout_events {
            if event.get_starttime() == layout.get_starttime() {
                if event.get_endtime() == layout.get_endtime() {
                    logos.push(**layout);
                }
            }
        }

        for logo in &logo_events {
            if event.get_starttime() <= logo.get_starttime()
                && logo.get_starttime() <= event.get_endtime()
            {
                logos.push(**logo);
            }
        }

        logos
    }

    pub fn to_string(&self, utc: bool) -> String {
        let mut special_event = String::new();
        for s in &self.vec {
            match s {
                Define::vaEvent(event) => {
                    let logos = self.find_logo(event);
                    let mut logostr = String::new();
                    if event.get_contentid() == "cb7a119f84cb7b117b1b"
                        || event.get_contentid() == "392654926764849cd5dc"
                        || event.get_contentid() == "e90dfb84e30edf611e32"
                        || event.get_contentid() == "b1735b7c5101727b3c6c"
                        || event.get_contentid().contains("WERBUNG")
                    {
                        logostr = format!("{}", "");
                    } else {
                        for logo in &logos {
                            logostr += &format!("{}", logo.get_title());
                        }
                        if logostr.len() == 0 {
                            logostr = format!("{}", "ERROR_NO_LOGO");
                        }
                    }

                    let mut title = event.get_title();
                    if title == " -  UHD1_WERBUNG-01" {
                        title = String::from("Werbung");
                    } else if title.contains(",") {
                        title = title.replace(",", "-");
                    } else if event.get_contentid() == "cb7a119f84cb7b117b1b" {
                        title += " - Dranbleiben";
                    } else if event.get_contentid() == "392654926764849cd5dc" {
                        title += " - Pausetafel";
                    }

                    special_event += &format!(
                        "{};{};{};{};{};{}\n",
                        title,
                        event.starttime_to_string(utc),
                        event.endtime_to_string(utc),
                        event.duration_to_string(),
                        event.get_contentid(),
                        logostr,
                    );
                }
                _ => (),
            }
        }

        println!("{}", special_event);
        special_event += &format!(";;;;;\n");

        special_event
    }

    pub fn print_table(&self, verbose: bool, utc: bool) -> (i64, i64) {
        let mut logoerror = 0;
        let mut iderror = 0;
        let layout_events: Vec<_> = self
            .vec
            .iter()
            .filter(|x| match x {
                Define::layoutEvent(..) => true,
                _ => false,
            })
            .collect();

        let logo_events: Vec<_> = self
            .vec
            .iter()
            .filter(|x| match x {
                Define::logoEvent(..) => true,
                _ => false,
            })
            .collect();

        for s in &self.vec {
            match s {
                Define::vaEvent(event) => {
                    let mut layouts = Vec::new();
                    let mut logos = Vec::new();
                    for layout in &layout_events {
                        if event.get_starttime() == layout.get_starttime() {
                            if event.get_endtime() != layout.get_endtime() {
                                logoerror += 1;
                                layouts.push((true, layout));
                            } else {
                                layouts.push((false, layout));
                            }
                        }
                    }
                    for logo in &logo_events {
                        if event.get_starttime() <= logo.get_starttime()
                            && logo.get_starttime() <= event.get_endtime()
                        {
                            if event.get_endtime() <= logo.get_endtime() {
                                logoerror += 1;
                                logos.push((true, logo));
                            } else {
                                logos.push((false, logo));
                            }
                        }
                    }

                    let mut logostr = String::new();
                    if event.get_contentid() == "cb7a119f84cb7b117b1b"
                        || event.get_contentid() == "392654926764849cd5dc"
                        || event.get_title().contains("Black")
                        || event.get_title().contains("NK_Maschinen_der_Superlat")
                        || event.get_contentid().contains("WERBUNG")
                    {
                        logostr = format!("{}", "");
                    } else {
                        for (error, logo) in &logos {
                            if *error {
                                logostr += &format!("ERROR {}", logo.get_title());
                            } else {
                                logostr += &format!("{}", logo.get_title());
                            }
                        }
                        for (error, laoyut) in &layouts {
                            if *error {
                                logostr += &format!("ERROR {}", laoyut.get_title());
                            } else {
                                logostr += &format!("{}", laoyut.get_title());
                            }
                        }
                        if logostr.len() == 0 {
                            logostr = format!("{}", "ERROR_NO_LOGO");
                        } else if logostr.chars().count() > 20 {
                            logostr = format!(
                                "{}",
                                logostr
                                    .chars()
                                    .into_iter()
                                    .enumerate()
                                    .filter(|(i, _)| *i < 20)
                                    .fold(String::new(), |mut acc, (_, c)| {
                                        acc += &format!("{}", c);
                                        return acc;
                                    })
                            );
                        }
                    }

                    let contentid = event.get_contentid();
                    if contentid.contains("-") && contentid != "UHD1_WERBUNG-01" {
                        iderror += 1;
                    }
                    if logostr.contains("ERROR") && logoerror == 0 && contentid != "UHD1_WERBUNG-01"
                    {
                        logoerror = 1;
                    }

                    let mut title = event.title_to_string();
                    if title == " -  UHD1_WERBUNG-01" {
                        title = "Werbung".to_string();
                    } else if event.get_contentid() == "cb7a119f84cb7b117b1b" {
                        title = "Dranbleiben".to_string();
                    } else if event.get_contentid() == "392654926764849cd5dc" {
                        title = "Pausentafel ".to_string();
                    }

                    if verbose {
                        println!(
                            "| {:30} | {:15} | {:23} | {:23} | {:12} | {:20} | {:15} |",
                            title,
                            event.programid_to_string(),
                            event.starttime_to_string(utc),
                            event.endtime_to_string(utc),
                            event.duration_to_string(),
                            if contentid.contains("-") && contentid != "UHD1_WERBUNG-01" {
                                contentid.red()
                            } else {
                                contentid.red().clear()
                            },
                            if logostr.contains("ERROR") && contentid != "UHD1_WERBUNG-01" {
                                logostr.red()
                            } else {
                                logostr.red().clear()
                            },
                        );
                    }
                }
                _ => (),
            }
        }

        (logoerror, iderror)
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
            "{:3} VaEvents",
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
            println!("{} SiEvents", si_events.len());
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
            println!("{} LogoEvents", logo_events.len());
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
            println!("{} LayoutEvents", layout_events.len());
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
            println!("{:3} events found.", format!("0").green());
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

    fn get_special_events(&self) -> Vec<SpecialEvent> {
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

        let indeces: Vec<Vec<usize>> = special_events
            .iter()
            .enumerate()
            .filter(|(_, &x)| {
                x.get_contentid() == "cb7a119f84cb7b117b1b"
                    || x.get_contentid() == "392654926764849cd5dc"
            })
            .map(|(i, _)| i as usize)
            .collect::<Vec<usize>>()
            .chunks(2)
            .map(|s| s.into())
            .collect();

        let mut result = Vec::new();
        for vec in &indeces {
            result.push(SpecialEvent::new(special_events[vec[0]..=vec[1]].to_vec()));
        }

        result
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

    pub fn write_special_events_csv(&self, filename: &str, utc: bool) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::prelude::*;
        let special_events = &self.get_special_events();

        let mut file = File::create(filename)?;

        match file.write_all(b"title;start;end;duration;contentid;logo;\n") {
            _ => (),
        }

        special_events.iter().for_each(|special_event| {
            match file.write_all(special_event.to_string(utc).as_bytes()) {
                Err(..) => (),
                Ok(..) => (),
            }
        });

        Ok(())
    }

    pub fn print_special_events(&self, verbose: bool, utc: bool) {
        let special_events = &self.get_special_events();
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
