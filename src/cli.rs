use read_input::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
pub enum Mode {
    Compile,
    Watch,
}
use Mode::*;

impl Mode {
    pub fn get() -> Self {
        for a in std::env::args() {
            match Self::from_str(a.as_ref()) {
                Ok(x) => return x,
                Err(_) => continue,
            }
        }
        input()
            .repeat_msg("Do you want to run in compile or watch mode?: ")
            .err("Input the word compile or the word watch.")
            .default(Watch)
            .get()
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
