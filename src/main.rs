mod commandline;
mod pts_loader;

use commandline::Commandline;
use pts_loader::dataset::DataSet;

fn main() -> std::io::Result<()> {
    let cmd = Commandline::parse();
    if cmd.filename() == "DEFAULT" && !cmd.repl() {
        Commandline::print_help();
    } else {
        let filename = cmd.filename();
        match DataSet::init(filename) {
            Ok(mut dataset) => {
                if cmd.all() || cmd.sierror() {
                    dataset.print_si_errors(cmd.verbose(), cmd.utc());
                }

                if cmd.all() || cmd.ps_event() {
                    dataset.print_special_events(cmd.verbose(), cmd.utc());
                }

                if cmd.write_csv() {
                    match dataset.write_special_events_csv(cmd.csv(), cmd.utc()) {
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
            Err(e) => {println!("{}", e); Commandline::print_help()},
        }
    }
    Ok(())
}
