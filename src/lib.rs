use std::fs;
use std::env;
use std::process;
use std::error::Error;
use std::io::stdin;


pub struct Config {
    pub filename: String,
    pub flags: Flags,
}

pub struct Flags {
    line_numbs: bool,
    line_by_line: bool,
}

impl Config {

    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };
        
        let flags = Flags::parse(args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

        Ok(Config {
            filename,
            flags,
        })
    }
}

impl Flags {

    fn parse(args: env::Args) -> Result<Flags, &'static str> {

        let mut line_numbs = false;
        let mut line_by_line = false;

        for arg in args {
            match arg.as_str() {
                "-ln" => line_numbs = true,
                "-lbl" => line_by_line = true,
                &_ => return Err("Invalid argument in parameters"),
            }
        }

        Ok(Flags {
            line_numbs,
            line_by_line,
        })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.filename)?;
    let mut ln = 1;
    let mut buffer = String::new();

    for line in contents.lines() {

        if config.flags.line_numbs {
            print!("{}: ", ln);
        }

        println!("{}", line);

        if config.flags.line_by_line {
            stdin().read_line(&mut buffer)?;
        }

        ln = ln + 1;
    }

    Ok(())
}