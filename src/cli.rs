use clap::{self, ArgMatches};
use read_input::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
pub enum Mode {
    Compile,
    Watch,
}
use Mode::*;

impl Mode {
    pub fn get(matches: &ArgMatches) -> Self {
        if matches.is_present("watch") {
            Mode::Watch
        } else if matches.is_present("compile") {
            Mode::Compile
        } else {
            input()
                .repeat_msg("Do you want to run in compile or watch mode?: ")
                .err("Input the word compile or the word watch.")
                .default(Mode::Watch)
                .get()
        }
    }
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" | "c" | "-c" | "-C" | "--compile" | "--Compile" | "-compile" | "-Compile"
            | "--C" | "--c" | "compile" | "Compile" => Ok(Compile),
            "W" | "w" | "-w" | "-W" | "--watch" | "--Watch" | "-watch" | "-Watch" | "--W"
            | "--w" | "watch" | "Watch" => Ok(Watch),
            _ => Err(()),
        }
    }
}
