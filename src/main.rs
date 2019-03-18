use std::env;
use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new(env::current_exe().unwrap().parent().unwrap()) {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
        println!(
            "{:#?}",
            entry.path().metadata().unwrap().modified().unwrap()
        );
    }
}
