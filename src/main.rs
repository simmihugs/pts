mod commandline;
mod pts_loader;
mod utils;

use crate::utils::fluid::Fluid;
use commandline::commandline::Commandline;
use commandline::summary::Summary;
use pts_loader::dataset::DataSet;

fn main() -> std::io::Result<()> {
    let cmd = Commandline::parse();

    if cmd.filename() == "YOU_PICK_A_FILE" {
        Commandline::print_help();
    } else {
        let filename = cmd.filename();
        match DataSet::init(filename) {
            Ok(mut dataset) => {
                let mut summary = Summary::new();

                let mut fluid_data_base = Fluid::init();

                match cmd.fluid_csv() {
                    None => (),
                    Some(path) => {
                        fluid_data_base.load(path);
                    }
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

                if cmd.all() || cmd.display_trailers() {
                    println!("\nTrailer");
                    dataset.display_trailers(&cmd);
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
                if cmd.debug() {
                    println!("{}", e);
                }
                Commandline::print_help()
            }
        }
    }

    Ok(())
}
