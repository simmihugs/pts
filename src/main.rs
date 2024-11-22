mod commandline;
mod pts_loader;
mod utils;

use crate::utils::fluid::Fluid;
use commandline::commandline::Commandline;
use commandline::summary::Summary;
use pts_loader::dataset::DataSet;
use utils::fluid;

fn main() -> std::io::Result<()> {
    let cmd = Commandline::parse();

    println!("{:?}", cmd.day());

    if cmd.update_fluid_data_base() {
        match fluid::download_fluid_data_base("test") {
            Ok(file_path) => println!("success: {}", file_path),
            Err(err) => eprintln!("{}", err),
        }
    }

    if cmd.filename() == "YOU_PICK_A_FILE" {
        println!("{:?}", "file not found");
        Commandline::print_help();
    } else {
        match DataSet::init(cmd.filename()) {
            Ok(mut dataset) => {
                let mut summary = Summary::new();

                let mut fluid_data_base = Fluid::init();

                if cmd.fluid_csv().is_some() {
                    fluid_data_base.load(cmd.fluid_csv().unwrap());
                }

                if cmd.all() || cmd.ps_event() {
                    dataset.print_special_events(&mut summary, &cmd, &fluid_data_base);
                }

                if cmd.all() || cmd.vaerrors() {
                    dataset.print_va_errors(&mut summary, &cmd);
                }

                if cmd.all() || cmd.sierrors() {
                    dataset.print_si_errors(&mut summary, &cmd);
                }

                if cmd.all() || cmd.missing_texts() {
                    dataset.print_missing_text_errors(&mut summary, &cmd);
                }

                if cmd.display_sievents() {
                    println!("\nSiEvents");
                    dataset.display_sievents(&cmd);
                }

                if cmd.display_trailers() {
                    println!("\nTrailer");
                    dataset.display_trailers(&cmd);
                }

                if cmd.all() || cmd.check_all_contentids() {
                    println!("{}", "\nAll content ids");
                    dataset.display_all_content_id_errors(&mut summary, &cmd);
                }

                if cmd.all() && cmd.fluid_csv().is_some() {
                    dataset.list_vaevents_with_length_errors(&mut summary, &cmd, &fluid_data_base);
                }

                summary.print(&cmd);

                if cmd.write_csv() {
                    match dataset.write_special_events_csv(&cmd, &fluid_data_base) {
                        Err(e) => {
                            if cmd.debug() {
                                println!("{}", e);
                            }
                        }
                        Ok(..) => println!("Wrote csv to {}", cmd.csv()),
                    }
                }

                if cmd.update_werbungen() {
                    match dataset.update_commercials(&cmd) {
                        Err(e) => println!("{}", e),
                        Ok(file) => println!("updated pts file: {}", file),
                    }
                }

                match cmd.illegalevents() {
                    None => (),
                    Some(illegals) => dataset.look_for_illegals(&illegals, &cmd),
                }

                if cmd.no_option() {
                    Commandline::print_help();
                }
            }
            Err(e) => {
                if format!("{}", e).contains("os error 2") {
                    println!(
                        "Das System kann die angegebene Datei {:?} nicht finden.",
                        cmd.filename()
                    );
                } else if cmd.debug() {
                    println!("{}", e);
                }
                Commandline::print_help()
            }
        }
    }

    Ok(())
}
