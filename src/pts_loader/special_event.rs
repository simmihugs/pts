use crate::pts_loader::block::Block;
use crate::pts_loader::event::Event;
use crate::utils::table_print;
use crate::utils::take::Take;
use crate::Fluid;
use crate::Summary;
use crate::{commandline::commandline::Commandline, pts_loader::define::Define};
use colored::{ColoredString, Colorize};

#[derive(Clone)]
enum LengthError {
    Trailer,
    LengthError,
    NoError,
}

#[derive(Clone)]
pub struct SpecialEvent<'a> {
    vec: Vec<&'a Define>,
}

pub fn print_special_events(
    special_events: Vec<&SpecialEvent<'_>>,
    special_event_errors: &Vec<Block<'_>>,
    fluid_data_set: &Fluid,
    summary: &mut Summary,
    cmd: &Commandline,
) {
    if special_events.len() > 0 {
        println!("Special events:");
        table_print::print_line(158 + 53 + 1 + 30);
        table_print::print_head();
        table_print::print_line_cross();
        special_events.iter().for_each(|special_event| {
            let terrors = special_event.get_time_errors();
            let (lerrors, length_errors) =
                special_event.print_table(&terrors, summary, cmd, fluid_data_set);
            table_print::print_line_cross();
            summary.logo_errors += lerrors;
            summary.time_errors += terrors.len() as i64;
            summary.length_error += length_errors;
        });
        table_print::print_head();
        table_print::print_line(158 + 53 + 1 + 30);
    }
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

impl<'a> SpecialEvent<'a> {
    pub fn new(vec: Vec<&'a Define>) -> Self {
        Self { vec }
    }

    pub fn get_time_errors(&self) -> Vec<String> {
        let mut events: Vec<_> = self
            .vec
            .iter()
            .filter(|x| match x {
                Define::vaEvent(..) => true,
                _ => false,
            })
            .collect();
        if events.len() == 0 {
            Vec::new()
        } else {
            let mut errors = Vec::new();
            let head = events[0];
            events.drain(1..).into_iter().fold(head, |acc, value| {
                if acc.get_event().get_endtime() != value.get_event().get_starttime() {
                    errors.push(value.get_event().get_programid());
                }
                value
            });
            errors
        }
    }

    pub fn get_commercials(&self) -> Vec<String> {
        let mut store = Vec::new();

        for s in &self.vec {
            match s {
                Define::vaEvent(event) => {
                    let id = event.get_contentid();
                    let title = event.get_title();
                    if id == "UHD1_WERBUNG-01" && title != " -  UHD1_WERBUNG-01" {
                        store.push(format!("{}", title));
                    }
                }
                _ => (),
            }
        }

        store
    }

    pub fn has_id_errors(&self) -> bool {
        for s in &self.vec {
            match s {
                Define::vaEvent(event) => {
                    let contentid = event.get_contentid();
                    if contentid.contains("-") && contentid != "UHD1_WERBUNG-01" {
                        return true;
                    }
                }
                _ => (),
            }
        }
        false
    }

    pub fn has_logo_errors(&self, cmd: &Commandline) -> bool {
        for s in &self.vec {
            match s {
                Define::vaEvent(event) => {
                    let (logos, logostr) = self.find_logo_str(event, cmd);

                    if logostr.contains("ERROR") {
                        return true;
                    }
                    if logos.len() != 0 && logos[0].get_event().get_endtime() > event.get_endtime()
                    {
                        return true;
                    }
                }
                _ => (),
            }
        }
        false
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
            if layout.get_event().get_programid() == event.get_programid() {
                logos.push(**layout);
            }
        }

        for logo in &logo_events {
            if logo.get_event().get_programid() == event.get_programid() {
                logos.push(**logo);
            }
        }

        logos
    }

    fn find_logo_str(&self, event: &Event, cmd: &Commandline) -> (Vec<&Define>, String) {
        let debug_me = false;
        let logos = self.find_logo(event);
        let mut answer: String = String::new();
        if event.get_contentid() == "cb7a119f84cb7b117b1b"
            || event.get_contentid() == "392654926764849cd5dc"
        // Black
            || event.get_contentid() == "e90dfb84e30edf611e32"
            || event.get_contentid() == "b1735b7c5101727b3c6c"
        // Werbung 
            || event.get_contentid().contains("WERBUNG")
        //live
            || event.get_contentid() == "UHD_IN2"
        // Baelle
            || event.get_contentid() == "5675d8c63df2424bf286"
        // galileo
            || event.get_contentid() == "64bb104f8aa130071723"
        // nachklappe pro7
            || event.get_contentid() == "29996549985440a20fa1"
        // nachklappe k1
            || event.get_contentid() == "563f387cf4cfd279039a"
        // Trailer75d1d4afe3f26b6412d4
            || event.get_title().contains("Trailer")
            || event.get_contentid() == "b52d22eeb30a63a4518f"
            || event.get_title().contains("ST2QX9MQ")
            || event.get_contentid().contains("e4a2e62d68e2ad9bfaae")
            || event.get_contentid().contains("75d1d4afe3f26b6412d4")
            || event.get_contentid().contains("e48363d83407359a6dd2")
        {
            if logos.len() != 0 {
                if debug_me {
                    println!("{:?}", event);
                    println!("Should not have logos, has: {:?}", logos);
                }
                answer = String::from("ERROR_LOGO_FOUND");
            }
        } else {
            if logos.len() > 1 {
                if event.get_contentid() == "UHD_LIVE" {
                } else {
                    if debug_me {
                        println!("Should have 1 logos, has: {:?}", logos);
                    }
                    answer = String::from("ERROR_MORE_THAN_ONE_LOGO");
                }
            } else if logos.len() == 0 {
                if debug_me {
                    println!("Should have logos, has 0");
                }
                answer = String::from("ERROR_NO_LOGO_FOUND");
            } else {
                for logo in &logos {
                    match logo {
                        Define::layoutEvent(levent) => {
                            if levent.get_starttime() != event.get_starttime()
                                || levent.get_endtime() != event.get_endtime()
                            {
                                answer = String::from("ERROR_TIME_LOGO");
                            } else {
                                if cmd.debug() {
                                    println!("{:?}", event);
                                    println!("{:?}", levent);
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
        (logos, answer)
    }

    pub fn to_string(&self, cmd: &Commandline, fluid_data_set: &Fluid) -> String {
        let mut special_event = String::new();
        for s in &self.vec {
            match s {
                Define::vaEvent(event) => {
                    let (logos, logostr): (Vec<_>, String) = self.find_logo_str(event, cmd);

                    let mut title = event.get_title();

                    let contentid = event.get_contentid();
                    if title.contains(",") {
                        title = title.replace(",", "-");
                    } else if contentid == "cb7a119f84cb7b117b1b" {
                        title += " - Dranbleiben";
                    } else if contentid == "392654926764849cd5dc" {
                        title += " - Pausetafel";
                    } else if contentid == "UHD1_WERBUNG-01" {
                        title = event.get_title();
                        if title == " -  UHD1_WERBUNG-01" {
                            title = "UHD1_WERBUNG-01".to_string();
                        } else {
                            let new_title =
                                title.replace(" - ", "").replace(" UHD1_WERBUNG-01", "");
                            title = new_title;
                        }
                    }
                    let contentid = event.get_contentid();

                    let tcin_tcout = if event.get_duration() < 3 * 60_000
                        || (event.get_contentid() == "cb7a119f84cb7b117b1b"
                            || event.get_contentid() == "392654926764849cd5dc"
                            || event.get_contentid() == "UHD1_WERBUNG-01")
                    {
                        format!("{};{}", " ".repeat(12), " ".repeat(12))
                    } else {
                        match &event.get_tcin_tcout() {
                            None => format!("{};{}", " ".repeat(12), " ".repeat(12)),
                            Some((a, b)) => format!(
                                "{};{}",
                                Event::standalone_duration_to_string(a, cmd.fps()).take(12),
                                Event::standalone_duration_to_string(b, cmd.fps()).take(12),
                            ),
                        }
                    };

                    special_event += &format!(
                        "{};{};{};{};{};{};{};{}\n",
                        title,
                        if event.get_duration() > 30 * 1000
                            && event.get_contentid() != "cb7a119f84cb7b117b1b"
                            && event.get_contentid() != "392654926764849cd5dc"
                        {
                            match fluid_data_set.query(&contentid) {
                                None => "".to_string(),
                                Some(s) => s.to_string(),
                            }
                        } else {
                            "".to_string()
                        },
                        event.starttime_to_string(cmd.utc(), cmd.fps()),
                        event.endtime_to_string(cmd.utc(), cmd.fps()),
                        event.duration_to_string(cmd.fps()),
                        tcin_tcout,
                        contentid,
                        logostr,
                    );
                    for logo in &logos {
                        special_event += &format!(
                            "{};{};{};{};{};{};{};{}\n",
                            "",
                            "",
                            logo.get_event().starttime_to_string(cmd.utc(), cmd.fps()),
                            logo.get_event().endtime_to_string(cmd.utc(), cmd.fps()),
                            logo.get_event().duration_to_string(cmd.fps()),
                            format!("{};{}", " ".repeat(12), " ".repeat(12)),
                            logo.get_event().get_contentid(),
                            logo.get_event().get_logo(),
                        );
                    }
                }
                _ => (),
            }
        }
        special_event += &format!(";;;;;\n");
        special_event += &format!(";;;;;\n");

        special_event
    }

    fn color_starttime(
        time_errors: &Vec<String>,
        event: &Event,
        found_first_event: &mut bool,
        found_dran_bleiben: &mut bool,
        utc: bool,
        fps: Option<i64>,
        length: usize,
    ) -> ColoredString {
        let mut time_error = false;
        for id in time_errors {
            if event.get_programid().contains(&*id) {
                time_error = true;
            }
        }

        if *found_first_event {
            *found_first_event = false;
            *found_dran_bleiben = false;
            if time_error {
                event.starttime_to_string(utc, fps).take(length).red()
            } else {
                event.starttime_to_string(utc, fps).take(length).cyan()
            }
        } else {
            if time_error {
                event.starttime_to_string(utc, fps).take(length).red()
            } else {
                event
                    .starttime_to_string(utc, fps)
                    .take(length)
                    .cyan()
                    .clear()
            }
        }
    }

    pub fn print_table(
        &self,
        time_errors: &Vec<String>,
        summary: &mut Summary,
        cmd: &Commandline,
        fluid_data_set: &Fluid,
    ) -> (i64, i64) {
        let werbungen = &cmd.werbungen();
        //let tcins_tcouts = &cmd.tcins_tcouts();

        let mut logoerrors = 0;
        //let mut iderrors = 0;
        let mut length_errors: i64 = 0;

        let mut found_first_event: bool = false;
        let mut found_dran_bleiben: bool = false;

        let _1min = 60 * 1000;
        let _30sec = _1min / 2;
        let _5min = 5 * _1min;
        let _15min = 15 * _1min;

        for s in &self.vec {
            match s {
                Define::vaEvent(event) => {
                    let event_length_error: LengthError = {
                        match event.get_contentid().as_str() {
                            "392654926764849cd5dc" => {
                                if !(_5min <= event.get_duration()
                                    && event.get_duration() <= _15min)
                                {
                                    length_errors += 1;
                                    LengthError::LengthError
                                } else {
                                    LengthError::NoError
                                }
                            }
                            "cb7a119f84cb7b117b1b" => {
                                if !(_5min <= event.get_duration()
                                    && event.get_duration() <= _15min)
                                {
                                    length_errors += 1;
                                    LengthError::LengthError
                                } else {
                                    LengthError::NoError
                                }
                            }
                            "e90dfb84e30edf611e32" => {
                                if !(event.get_duration() <= _30sec) {
                                    length_errors += 1;
                                    LengthError::LengthError
                                } else {
                                    LengthError::NoError
                                }
                            }
                            _ => {
                                if event.get_duration() <= _1min {
                                    LengthError::Trailer
                                } else {
                                    LengthError::NoError
                                }
                            }
                        }
                    };

                    let (logos, mut logostr) = self.find_logo_str(event, cmd);

                    if cmd.debug() {
                        println!("Logo: {}", logostr);
                        if logostr.is_empty()
                            && event
                                .get_title()
                                .contains("Abenteuer Leben am Sonntag: Cornel")
                        {
                            println!("{:?}", event);
                        }
                    }

                    let mut title = event.get_title();

                    // if title.contains(":") || title.contains("-") {
                    //     println!(
                    //         "{:?} has length: {} and number of chars: {}",
                    //         title,
                    //         title.len(),
                    //         title.chars().count(),
                    //     )
                    // }

                    let contentid = event.get_contentid();
                    // if contentid.contains("-") && contentid != "UHD1_WERBUNG-01" {
                    //     iderrors += 1;
                    // }
                    if contentid == "UHD1_WERBUNG-01" {
                        if title == " -  UHD1_WERBUNG-01" {
                            title = "UHD1_WERBUNG-01".to_string();
                        } else {
                            title = title.replace(" - ", "").replace(" UHD1_WERBUNG-01", "");
                        }
                    } else if event.get_contentid() == "cb7a119f84cb7b117b1b" {
                        title = "Dranbleiben".to_string();
                    } else if event.get_contentid() == "392654926764849cd5dc" {
                        title = "Pausentafel ".to_string();
                    }
                    title = title.take(30).take(30);

                    if title == "Dranbleiben" {
                        //Init starttime coloring with dranbleiben
                        found_dran_bleiben = true;
                    } else if found_dran_bleiben && event.get_duration() >= 60000 {
                        //Found first event after dranbleiben with length in minutes
                        found_first_event = true;
                    }

                    if logostr.contains("ERROR") && contentid != "UHD1_WERBUNG-01" {
                        if cmd.debug() {
                            println!("{}", "found error");
                        }
                        logoerrors += 1;
                    }

                    let mut title_string = if title.contains("TAK") {
                        title.red()
                    } else {
                        title.red().clear()
                    };
                    let mut programid_string = event.programid_to_string().take(15).red().clear();
                    let mut starttime_string = SpecialEvent::color_starttime(
                        time_errors,
                        event,
                        &mut found_first_event,
                        &mut found_dran_bleiben,
                        cmd.utc(),
                        cmd.fps(),
                        23,
                    )
                    .red()
                    .clear();
                    let mut duration_string = {
                        let duration_length = 12;
                        match event_length_error {
                            LengthError::Trailer => event
                                .duration_to_string(cmd.fps())
                                .take(duration_length)
                                .purple(),
                            LengthError::LengthError => event
                                .duration_to_string(cmd.fps())
                                .take(duration_length)
                                .red(),
                            LengthError::NoError => {
                                if title == "Werbung" {
                                    event
                                        .duration_to_string(cmd.fps())
                                        .take(duration_length)
                                        .yellow()
                                } else {
                                    event
                                        .duration_to_string(cmd.fps())
                                        .take(duration_length)
                                        .yellow()
                                        .clear()
                                }
                            }
                        }
                    };
                    let mut contentid_string = {
                        let length = 20;
                        if contentid == "UHD1_WERBUNG-01" {
                            contentid.to_string().take(length).yellow()
                        } else if contentid.contains("-") {
                            contentid.to_string().take(length).red()
                        } else {
                            contentid.to_string().take(length).red().clear()
                        }
                    };
                    let mut logostr_string =
                        if logostr.contains("ERROR") && contentid != "UHD1_WERBUNG-01" {
                            logostr.take(16).red()
                        } else {
                            logostr.take(16).red().clear()
                        };
                    let mut content_string = if event.get_duration() > 30 * 1000
                        && event.get_contentid() != "cb7a119f84cb7b117b1b"
                        && event.get_contentid() != "392654926764849cd5dc"
                    {
                        match fluid_data_set.query(&contentid) {
                            None => "".to_string(),
                            Some(s) => s.to_string().take(50),
                        }
                    } else {
                        "".to_string()
                    }
                    .take(50)
                    .red()
                    .clear();
                    let mut endtime_string = event
                        .endtime_to_string(cmd.utc(), cmd.fps())
                        .take(23)
                        .red()
                        .clear();
                    match werbungen {
                        None => (),
                        Some(w) => {
                            for x in w.iter() {
                             if x.len() > 1 {
                                if title.contains(&x[0]) && event.duration_to_string(cmd.fps()) != x[1] {
                                        title_string = title_string.red();
                                        programid_string = programid_string.red();
                                        starttime_string = starttime_string.red();
                                        duration_string = duration_string.red();
                                        contentid_string = contentid_string.red();
                                        logostr_string = logostr_string.red();
                                        content_string = content_string.red();
                                        endtime_string = endtime_string.red();
                                        summary.commercial_error += 1;
                                    } else {
                                        title_string = title_string.cyan();
                                        programid_string = programid_string.cyan();
                                        starttime_string = starttime_string.cyan();
                                        duration_string = duration_string.cyan();
                                        contentid_string = contentid_string.cyan();
                                        logostr_string = logostr_string.cyan();
                                        content_string = content_string.cyan();
                                        endtime_string = endtime_string.cyan();
                                    }
                                }
                            }
                        }
                    }

                    let tcin_tcout = if event.get_duration() < 3 * 60_000
                        || (event.get_contentid() == "cb7a119f84cb7b117b1b"
                            || event.get_contentid() == "392654926764849cd5dc"
                            || event.get_contentid() == "UHD1_WERBUNG-01")
                    {
                        format!("{} | {}", " ".repeat(12), " ".repeat(12))
                            .red()
                            .clear()
                    } else {
                        match &event.get_tcin_tcout() {
                            None => format!("{} | {}", " ".repeat(12), " ".repeat(12))
                                .red()
                                .clear(),
                            Some((a, b)) => format!(
                                "{} | {}",
                                Event::standalone_duration_to_string(a, cmd.fps()).take(12),
                                Event::standalone_duration_to_string(b, cmd.fps()).take(12),
                            )
                            .red()
                            .clear(),
                        }
                    };

                    if cmd.verbose() {
                        println!(
                            "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                            title_string,
                            content_string,
                            programid_string,
                            starttime_string,
                            endtime_string,
                            duration_string,
                            tcin_tcout,
                            contentid_string,
                            logostr_string,
                        );
                        for logo in &logos {
                            let mut logostr = logo.get_event().get_logo();
                            if logostr.len() > 14 {
                                logostr = logostr.drain(0..14).collect::<String>();
                            }

                            println!(
                                "| {:30} | {:50} | {:15} | {:23} | {:23} | {:12} | {} | {:20} | {} |",
                                " ",
                                " ",
                                logo.get_event().programid_to_string().green(),
                                logo.get_event()
                                    .starttime_to_string(cmd.utc(), cmd.fps())
                                    .green(),
                                logo.get_event()
                                    .endtime_to_string(cmd.utc(), cmd.fps())
                                    .green(),
                                logo.get_event().duration_to_string(cmd.fps()).green(),
                                format!("{} | {}", " ".repeat(12), " ".repeat(12)),
                                logo.get_event().get_contentid().green(),
                                logostr.take(16).green(),
                            );
                        }
                    }
                }
                _ => (),
            }
        }

        (logoerrors, length_errors)
    }
}
