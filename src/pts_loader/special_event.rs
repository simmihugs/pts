use crate::pts_loader::define::Define;
use crate::pts_loader::event::Event;
use colored::{ColoredString, Colorize};

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
            if event.get_starttime() == layout.get_event().get_starttime() {
                if event.get_endtime() == layout.get_event().get_endtime() {
                    logos.push(**layout);
                }
            }
        }

        for logo in &logo_events {
            if event.get_starttime() <= logo.get_event().get_starttime()
                && logo.get_event().get_starttime() <= event.get_endtime()
            {
                logos.push(**logo);
            }
        }

        logos
    }

    pub fn to_string(&self, utc: bool, fps: Option<i64>) -> String {
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
                            logostr += &format!("{}", logo.get_event().get_logo());
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
                        event.starttime_to_string(utc, fps),
                        event.endtime_to_string(utc, fps),
                        event.duration_to_string(fps),
                        event.get_contentid(),
                        logostr,
                    );
                }
                _ => (),
            }
        }
        special_event += &format!(";;;;;\n");

        special_event
    }

    fn color_title(
        event: &Event,
        found_first_event: &mut bool,
        found_dran_bleiben: &mut bool,
        utc: bool,
        fps: Option<i64>,
    ) -> ColoredString {
        if *found_first_event {
            *found_first_event = false;
            *found_dran_bleiben = false;
            event.starttime_to_string(utc, fps).cyan()
        } else {
            event.starttime_to_string(utc, fps).cyan().clear()
        }
    }

    pub fn print_table(&self, verbose: bool, utc: bool, fps: Option<i64>) -> (i64, i64) {
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

        let mut found_first_event: bool = false;
        let mut found_dran_bleiben: bool = false;
        for s in &self.vec {
            match s {
                Define::vaEvent(event) => {
                    let mut layouts = Vec::new();
                    let mut logos = Vec::new();
                    for layout in &layout_events {
                        if event.get_starttime() == layout.get_event().get_starttime() {
                            if event.get_endtime() != layout.get_event().get_endtime() {
                                logoerror += 1;
                                layouts.push((true, layout));
                            } else {
                                layouts.push((false, layout));
                            }
                        }
                    }
                    for logo in &logo_events {
                        if event.get_starttime() <= logo.get_event().get_starttime()
                            && logo.get_event().get_starttime() <= event.get_endtime()
                        {
                            if event.get_endtime() <= logo.get_event().get_endtime() {
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
                        || event.get_duration() < 60_0000
                    {
                        logostr = format!("{}", "");
                    } else {
                        for (error, logo) in &logos {
                            if *error {
                                logostr += &format!("ERROR {}", logo.get_event().get_logo());
                            } else {
                                logostr += &format!(" {}", logo.get_event().get_logo());
                            }
                        }
                        for (error, laoyut) in &layouts {
                            if *error {
                                logostr += &format!("ERROR {}", laoyut.get_event().get_logo());
                            } else {
                                logostr += &format!(" {}", laoyut.get_event().get_logo());
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

                    if verbose {
                        println!(
                            "| {:30} | {:15} | {:23} | {:23} | {:12} | {:20} | {:15} |",
                            title,
                            event.programid_to_string(),
                            SpecialEvent::color_title(
                                event,
                                &mut found_first_event,
                                &mut found_dran_bleiben,
                                utc,
                                fps
                            ),
                            event.endtime_to_string(utc, fps),
                            if title == "Werbung" {
                                event.duration_to_string(fps).yellow()
                            } else {
                                event.duration_to_string(fps).yellow().clear()
                            },
                            if contentid.contains("-") && contentid != "UHD1_WERBUNG-01" {
                                contentid.red()
                            } else {
                                contentid.red().clear()
                            },
                            if logostr.contains("ERROR") && contentid != "UHD1_WERBUNG-01" {
                                logostr.red()
                            } else if logos.len() + layouts.len() > 1 {
                                logoerror += 1;
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
