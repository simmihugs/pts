use crate::{commandline::Commandline, pts_loader::dataset::DataSet};
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
        println!(":e | :encoding  --> specify encoding for csv export");
        println!(":c | :csv <OUT> | :csv <FILE> <OUT> --> write to csv");
    }

    fn extract(term: &Vec<String>, user_io: &String) -> Result<String, ()> {
        let words: Vec<String> = user_io
            .split(' ')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        match words.iter().position(|x| term.contains(x)) {
            Some(index) => {
                if words.len() > index + 1 {
                    return Ok(words[index + 1].to_string());
                }
            }
            _ => (),
        }

        Err(())
    }

    fn utc(user_io: &String) -> bool {
        strings![":utc", ":u"]
            .iter()
            .map(|x| user_io.contains(x))
            .any(|x| x == true)
    }

    fn fps(user_io: &String) -> Option<i64> {
        match Repl::extract(&(strings![":fps"]), user_io) {
            Ok(value) => match value.parse::<i64>() {
                Ok(number) => Some(number),
                Err(..) => None,
            },
            Err(()) => None,
        }
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

    pub fn parse<'a>(user_io: String, files: &mut Vec<DataSet>) -> Result<(), String> {
        let help: Vec<String> = strings![":help", ":h"];
        let enc: Vec<String> = strings![":e", ":encoding"];
        let csv: Vec<String> = strings![":c", ":csv"];
        let utc: bool = Repl::utc(&user_io);
        let fps: Option<i64> = Repl::fps(&user_io);
        let verbose: bool = Repl::verbose(&user_io);
        let quit: Vec<String> = strings![":quit", ":q"];
        let load: Vec<String> = strings!["ls", ":load", ":l", ":f"];
        let all: Vec<String> = strings![":all", ":a"];
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
            Ok(())
        } else if Repl::contains(&quit, &user_io) {
            Err(String::from("quit"))
        } else {
            if Repl::contains(&load, &user_io) {
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
                            return Ok(());
                        }
                    }
                    None => (),
                }
            }
            if Repl::contains(&csv, &user_io) {
                let encoding = match Repl::extract(&enc, &user_io) {
                    Ok(s) => s,
                    Err(..) => "utf-8".to_string(),
                };
                match words.iter().position(|x| csv.contains(x)) {
                    Some(index) => {
                        if words.len() > index + 1 {
                            let csv_file = &words[index + 1];
                            if files.len() > 0 {
                                match files[0]
                                    .write_special_events_csv(&csv_file, &encoding, utc, fps)
                                {
                                    Err(e) => println!("{}", e),
                                    _ => println!(
                                        "wrote {}-csv to file {}",
                                        files[0].get_filename(),
                                        csv_file
                                    ),
                                }
                            }
                        } else if words.len() > index + 2 {
                            match words[index + 1].parse::<i32>() {
                                Ok(file_index) => match file_index.try_into() {
                                    Ok(_index) => {
                                        if files.len() > _index {
                                            let csv_file = &words[index + 2];
                                            match files[_index].write_special_events_csv(
                                                &csv_file, &encoding, utc, fps,
                                            ) {
                                                Err(e) => println!("{}", e),
                                                _ => println!(
                                                    "wrote {}-csv to file {}",
                                                    files[_index].get_filename(),
                                                    csv_file
                                                ),
                                            }
                                        }
                                    }
                                    _ => (),
                                },
                                _ => (),
                            }
                        }
                    }
                    None => (),
                }
            }
            if Repl::contains(&all, &user_io) {
                match words.iter().position(|x| all.contains(x)) {
                    Some(index) => {
                        if words.len() > index + 1 {
                            match words[index + 1].parse::<i32>() {
                                Ok(file_index) => match file_index.try_into() {
                                    Ok(_index) => {
                                        if files.len() > _index {
                                            files[_index].print_si_errors(verbose, utc);
                                            //files[_index].print_special_events(verbose, utc, false, fps);
                                            files[_index].print_special_events(&Commandline::new(
                                                verbose,
                                                utc,
                                                false,
                                                fps.unwrap(),
                                            ));
                                        } else {
                                            println!("invalid index {}", file_index);
                                        }
                                    }
                                    _ => (),
                                },
                                Err(..) => {
                                    files[0].print_si_errors(verbose, utc);
                                    //files[0].print_special_events(verbose, utc, false, fps);
                                    files[0].print_special_events(&Commandline::new(
                                        verbose,
                                        utc,
                                        false,
                                        fps.unwrap(),
                                    ));
                                }
                            }
                        } else if files.len() == 1 {
                            files[0].print_si_errors(verbose, utc);
                            //files[0].print_special_events(verbose, utc, false, fps);
                            files[0].print_special_events(&Commandline::new(
                                verbose,
                                utc,
                                false,
                                fps.unwrap(),
                            ));
                        }
                    }
                    None => (),
                }
            } else if Repl::contains(&si_errors, &user_io)
                || Repl::contains(&special_events, &user_io)
            {
                match words.iter().position(|x| special_events.contains(x)) {
                    Some(index) => {
                        if words.len() > index + 1 {
                            match words[index + 1].parse::<i32>() {
                                Ok(file_index) => match file_index.try_into() {
                                    Ok(_index) => {
                                        if files.len() > _index {
                                            /*                                             files[_index]
                                            .print_special_events(verbose, utc, false, fps);*/
                                            files[_index].print_special_events(&Commandline::new(
                                                verbose,
                                                utc,
                                                false,
                                                fps.unwrap(),
                                            ));
                                        } else {
                                            println!("invalid index {}", file_index);
                                        }
                                    }
                                    _ => (),
                                },
                                Err(..) => {
                                    //files[0].print_special_events(verbose, utc, false, fps);
                                    files[0].print_special_events(&Commandline::new(
                                        verbose,
                                        utc,
                                        false,
                                        fps.unwrap(),
                                    ));
                                }
                            }
                        } else if files.len() == 1 {
                            //files[0].print_special_events(verbose, utc, false, fps);
                            files[0].print_special_events(&Commandline::new(
                                verbose,
                                utc,
                                false,
                                fps.unwrap(),
                            ));
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
            }
            Ok(())
        }
    }

    fn run_repl(files: &mut Vec<DataSet>) {
        use std::io;
        let mut user_io = String::new();
        println!("Welcome to pts repl! 🚀");
        loop {
            io::stdout()
                .write_all(b"pts-repl> ")
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
