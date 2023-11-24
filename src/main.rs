mod commandline;
mod pts_loader;
mod summary;

use commandline::Commandline;
use pts_loader::dataset::DataSet;
use summary::Summary;

fn main() -> std::io::Result<()> {
    let cmd = Commandline::parse();
    if cmd.filename() == "YOU_PICK_A_FILE" {
        Commandline::print_help();
    } else {
        let filename = cmd.filename();
        match DataSet::init(filename) {
            Ok(mut dataset) => {
                let mut summary = Summary::new();
                if cmd.all() || cmd.ps_event() {
                    dataset.print_special_events(&mut summary, &cmd);
                }

                if cmd.all() || cmd.vaerrors() {
                    dataset.print_va_errors(&mut summary, &cmd);
                }

                if cmd.all() || cmd.sierrors() {
                    dataset.print_si_errors(&mut summary, &cmd);
                }
                summary.print(&cmd);

                if cmd.write_csv() {
                    match dataset.write_special_events_csv(
                        cmd.csv(),
                        cmd.encoding(),
                        cmd.utc(),
                        cmd.fps(),
                    ) {
                        Err(e) => println!("{}", e),
                        Ok(..) => println!("Wrote csv to {}", cmd.csv()),
                    }
                }

                match cmd.illegalevents() {
                    None => (),
                    Some(illegals) => {
                        dataset.look_for_illegals(&illegals, cmd.verbose(), cmd.utc())
                    }
                }

                if cmd.no_option() {
                    Commandline::print_help();
                }
            }
            Err(e) => {
                println!("{}", e);
                Commandline::print_help()
            }
        }
    }
    Ok(())
}
