use crate::pts_loader::dataset::DataSet;
use std::io::Write;

#[derive(Clone)]
pub struct Repl;

macro_rules! strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

impl Repl {
    fn print_help() {
        println!(":h | :help --> print help");
        println!(":q | :quit --> quit");
        println!(":l | :f <FILE>| :load <FILE> --> list or try loading new file");
        println!(":a | :all --> looking for all errors");
        println!(":p | :special-event --> looking for special events");
        println!(":s | :si-error --> looking for si-errors");
        println!(":u | :utc  --> utc flag");
        println!(":v | :verbose  --> verbose flag");
        println!("\nexample: :l C:\\Users\\sgraetz\\Documents\\exportiert__15-09-2023--02-10-2023\\hdplus_20230915_26886.pts");
    }

    fn utc(user_io: &String) -> bool {
        strings![":utc", ":u"]
            .iter()
            .map(|x| user_io.contains(x))
            .any(|x| x == true)
    }

    fn verbose(user_io: &String) -> bool {
        strings![":verbose", ":v"]
            .iter()
            .map(|x| user_io.contains(x))
            .any(|x| x == true)
    }

    fn contains(list: &Vec<String>, user_io: &String) -> bool {
        list.iter().map(|x| user_io.contains(x)).any(|x| x == true)
    }

    pub fn parse<'a>(user_io: String, files: &mut Vec<DataSet>) -> Result<String, ()> {
        let help: Vec<String> = strings![":help", ":h", "-h", "-help"];
        let utc: bool = Repl::utc(&user_io);
        let verbose: bool = Repl::verbose(&user_io);
        let quit: Vec<String> = strings![":quit", ":q", "-q", "-quit"];
        let load: Vec<String> = strings!["ls", ":load", ":l", "-l", ":f", "-f ", "-load"];
        let all: Vec<String> = strings![":all", ":a", "-a", "-all"];
        let si_errors: Vec<String> = strings![":si", ":s", ":si-error"];
        let special_events: Vec<String> = strings![":special-event", ":p"];
        let user_io_clean = user_io
            .replace("\r", "")
            .replace("\n", "")
            .trim_end()
            .to_string();
        let words = user_io_clean
            .split(' ')
            .map(|x| String::from(x))
            .collect::<Vec<String>>();

        if Repl::contains(&help, &user_io) {
            Repl::print_help();
            Ok("help".to_string())
        } else if Repl::contains(&quit, &user_io) {
            Err(())
        } else if Repl::contains(&load, &user_io) {
            match words.iter().position(|x| load.contains(x)) {
                Some(index) => {
                    if words.len() > index + 1 {
                        let new_file = &words[index + 1];
                        if !files
                            .iter()
                            .map(|x| String::from(x.get_filename()))
                            .collect::<Vec<String>>()
                            .contains(&new_file)
                        {
                            match DataSet::init(&new_file) {
                                Err(e) => println!("{}", e),
                                Ok(data) => {
                                    println!("Loaded file: {}", new_file);
                                    files.push(data);
                                }
                            }
                        } else {
                            println!("file {} is allready loaded ", new_file);
                        }
                    } else {
                        for file in files {
                            println!("{}", file.get_filename());
                        }
                        return Ok("listed files".to_string());
                    }
                }
                None => (),
            }
            Ok("loaded file".to_string())
        } else if Repl::contains(&all, &user_io) {
            match words.iter().position(|x| all.contains(x)) {
                Some(index) => {
                    if words.len() > index + 1 {
                        match words[index + 1].parse::<i32>() {
                            Ok(file_index) => match file_index.try_into() {
                                Ok(_index) => {
                                    if files.len() > _index {
                                        files[_index].print_si_errors(verbose, utc);
                                        files[_index].print_special_events(verbose, utc);
                                    } else {
                                        println!("invalid index {}", file_index);
                                    }
                                }
                                _ => (),
                            },
                            Err(..) => {
                                files[0].print_si_errors(verbose, utc);
                                files[0].print_special_events(verbose, utc);
                            }
                        }
                    } else if files.len() == 1 {
                        files[0].print_si_errors(verbose, utc);
                        files[0].print_special_events(verbose, utc);
                    }
                }
                None => (),
            }
            Ok("all".to_string())
        } else if Repl::contains(&si_errors, &user_io) || Repl::contains(&special_events, &user_io)
        {
            match words.iter().position(|x| special_events.contains(x)) {
                Some(index) => {
                    if words.len() > index + 1 {
                        match words[index + 1].parse::<i32>() {
                            Ok(file_index) => match file_index.try_into() {
                                Ok(_index) => {
                                    if files.len() > _index {
                                        files[_index].print_special_events(verbose, utc);
                                    } else {
                                        println!("invalid index {}", file_index);
                                    }
                                }
                                _ => (),
                            },
                            Err(..) => {
                                files[0].print_special_events(verbose, utc);
                            }
                        }
                    } else if files.len() == 1 {
                        files[0].print_special_events(verbose, utc);
                    }
                }
                None => (),
            }
            match words.iter().position(|x| si_errors.contains(x)) {
                Some(index) => {
                    if words.len() > index + 1 {
                        match words[index + 1].parse::<i32>() {
                            Ok(file_index) => match file_index.try_into() {
                                Ok(_index) => {
                                    if files.len() > _index {
                                        files[_index].print_si_errors(verbose, utc);
                                    } else {
                                        println!("invalid index {}", file_index);
                                    }
                                }
                                _ => (),
                            },
                            Err(..) => {
                                files[0].print_si_errors(verbose, utc);
                            }
                        }
                    } else if files.len() == 1 {
                        files[0].print_si_errors(verbose, utc);
                    }
                }
                None => (),
            }
            Ok("".to_string())
        } else {
            println!("Reache else branch");
            Ok(format!("{}", user_io_clean))
        }
    }

    fn run_repl(files: &mut Vec<DataSet>) {
        use std::io;
        let mut user_io = String::new();
        let rocket = emojis::get("🚀").unwrap();
	println!("{}", "Welcome to pts repl! 🚀");
        loop {
            io::stdout()
                .write_all(b"pts-repl")
                .expect("Failed to write line");
            io::stdout().flush().expect("flush failed!");
            io::stdin()
                .read_line(&mut user_io)
                .expect("Failed to read line");

            match Repl::parse(user_io, files) {
                Ok(..) => (),
                Err(..) => break,
            }
            user_io = "".to_string();
        }
    }

    pub fn start_without_data() {
        let mut files = Vec::new();
        Repl::run_repl(&mut files);
    }

    pub fn start(dataset: &DataSet) {
        let mut files = vec![dataset.clone()];
        Repl::run_repl(&mut files);
    }
}
