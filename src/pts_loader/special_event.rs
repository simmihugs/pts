use crate::pts_loader::event::Event;
use crate::{commandline::Commandline, pts_loader::define::Define};
use colored::{ColoredString, Colorize};

#[derive(Clone)]
pub struct SpecialEvent<'a> {
    vec: Vec<&'a Define>,
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
                    } else if logos.len() != 0
                        && logos[0].get_event().get_endtime() > event.get_endtime()
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
            || event.get_contentid() == "e90dfb84e30edf611e32"
            || event.get_contentid() == "b1735b7c5101727b3c6c"
            || event.get_contentid().contains("WERBUNG")
            || event.get_duration() < 60_000
            || event.get_contentid() == "UHD_IN2"
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
                    //answer = String::from("UHD_LIVE_LOGOS");
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
                //answer = format!("{}", logos[0].get_event().get_logo());
            }
        }
        return (logos, answer);
    }

    pub fn to_string(&self, cmd: &Commandline) -> String {
        let mut special_event = String::new();
        for s in &self.vec {
            match s {
                Define::vaEvent(event) => {
                    let (logos, logostr): (Vec<_>, String) = self.find_logo_str(event, cmd);

                    let mut title = event.get_title();
                    let contentid = event.get_contentid();
                    if contentid == "UHD1_WERBUNG-01" {
                        title = String::from("Werbung");
                    } else if title.contains(",") {
                        title = title.replace(",", "-");
                    } else if contentid == "cb7a119f84cb7b117b1b" {
                        title += " - Dranbleiben";
                    } else if contentid == "392654926764849cd5dc" {
                        title += " - Pausetafel";
                    }

                    special_event += &format!(
                        "{};{};{};{};{};{}\n",
                        title,
                        event.starttime_to_string(cmd.utc(), cmd.fps()),
                        event.endtime_to_string(cmd.utc(), cmd.fps()),
                        event.duration_to_string(cmd.fps()),
                        event.get_contentid(),
                        logostr,
                    );
                    for logo in &logos {
                        special_event += &format!(
                            "{};{};{};{};{};{}\n",
                            "",
                            logo.get_event().starttime_to_string(cmd.utc(), cmd.fps()),
                            logo.get_event().endtime_to_string(cmd.utc(), cmd.fps()),
                            logo.get_event().duration_to_string(cmd.fps()),
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
                event.starttime_to_string(utc, fps).red()
            } else {
                event.starttime_to_string(utc, fps).cyan()
            }
        } else {
            if time_error {
                event.starttime_to_string(utc, fps).red()
            } else {
                event.starttime_to_string(utc, fps).cyan().clear()
            }
        }
    }

    pub fn print_table(&self, time_errors: &Vec<String>, cmd: &Commandline) -> (i64, i64) {
        let mut logoerror = 0;
        let mut iderror = 0;

        let debug_me = false;
        let mut found_first_event: bool = false;
        let mut found_dran_bleiben: bool = false;
        for s in &self.vec {
            match s {
                Define::vaEvent(event) => {
                    let (logos, mut logostr) = self.find_logo_str(event, cmd);

                    if debug_me {
                        println!("{}", logostr);
                        if logostr.is_empty()
                            && event
                                .get_title()
                                .contains("Abenteuer Leben am Sonntag: Cornel")
                        {
                            println!("{:?}", event);
                        }
                    }

                    let contentid = event.get_contentid();
                    if contentid.contains("-") && contentid != "UHD1_WERBUNG-01" {
                        iderror += 1;
                    }

                    let mut title = event.title_to_string();
                    if title == " -  UHD1_WERBUNG-01" {
                        title = "Werbung".to_string();
                    } else if event.get_contentid() == "cb7a119f84cb7b117b1b" {
                        title = "Dranbleiben".to_string();
                    } else if event.get_contentid() == "392654926764849cd5dc" {
                        title = "Pausentafel ".to_string();
                    }

                    if logostr.len() < 15 {
                        for _ in 0..15 - logostr.len() {
                            logostr += " ";
                        }
                    }
                    logostr = logostr.drain(0..14).collect::<String>();

                    if title == "Dranbleiben" {
                        //Init starttime coloring with dranbleiben
                        found_dran_bleiben = true;
                    } else if found_dran_bleiben && event.get_duration() >= 60000 {
                        //Found first event after dranbleiben with length in minutes
                        found_first_event = true;
                    }

                    if logostr.contains("ERROR") && contentid != "UHD1_WERBUNG-01" {
                        if debug_me {
                            println!("{}", "found error");
                        }
                        logoerror += 1;
                    }

                    if cmd.verbose() {
                        println!(
                            "| {:30} | {:15} | {:23} | {:23} | {:12} | {:20} | {:15} |",
                            title,
                            event.programid_to_string(),
                            SpecialEvent::color_starttime(
                                time_errors,
                                event,
                                &mut found_first_event,
                                &mut found_dran_bleiben,
                                cmd.utc(),
                                cmd.fps()
                            ),
                            event.endtime_to_string(cmd.utc(), cmd.fps()),
                            if title == "Werbung" {
                                event.duration_to_string(cmd.fps()).yellow()
                            } else {
                                event.duration_to_string(cmd.fps()).yellow().clear()
                            },
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
                        for logo in &logos {
                            let mut logostr = logo.get_event().get_logo();
                            if logostr.len() > 14 {
                                logostr = logostr.drain(0..14).collect::<String>();
                            }

                            println!(
                                "| {:30} | {:15} | {:23} | {:23} | {:12} | {:20} | {:15} |",
                                " ",
                                logo.get_event().programid_to_string().green(),
                                logo.get_event()
                                    .starttime_to_string(cmd.utc(), cmd.fps())
                                    .green(),
                                logo.get_event()
                                    .endtime_to_string(cmd.utc(), cmd.fps())
                                    .green(),
                                logo.get_event().duration_to_string(cmd.fps()).green(),
                                logo.get_event().get_contentid().green(),
                                logostr.green(),
                            );
                        }
                    }
                }
                _ => (),
            }
        }

        (logoerror, iderror)
    }
}
