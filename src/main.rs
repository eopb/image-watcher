#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::enum_glob_use)]

mod cli;
mod parse;

use cli::Mode;
use image::{DynamicImage, FilterType};
use set_error::ChangeError;
use std::{
    iter::Iterator,
    path::Path,
    thread,
    time::{self, SystemTime},
};

use parse::{parse_config, FileWatch, ImgEditJobs, Resize, Settings, SharedSettings, Size};

#[derive(Clone)]
struct FileWatched {
    file: FileWatch,
    time: Option<SystemTime>,
}

fn main() {
    let mode = Mode::get();
    println!("Parsing config file image_watcher.yaml");
    let config = match parse_config() {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    dbg!(config.clone());

    let mut files_list: Vec<FileWatched> = config
        .files_list
        .clone()
        .into_iter()
        .map(|x| FileWatched {
            file: FileWatch {
                other: file_share_or_combine(x.other.clone(), config.other.clone()),
                ..x.clone()
            },
            time: None,
        })
        .collect();
    loop {
        for (
            index,
            FileWatched {
                file: file,
                time: time,
            },
        ) in files_list.clone().iter().enumerate()
        {
            let modified = match when_modified(Path::new(&file.path)) {
                Ok(s) => s,
                Err(_) => return,
            };

            let img_edit_job = || match &file.other.jobs.resize {
                Some(resize) => {
                    match resize_image(file, resize) {
                        Ok(k) => k,
                        Err(e) => {
                            println!("{}", e);
                            return None;
                        }
                    };
                    Some(modified)
                }
                None => Some(modified),
            };
            files_list[index].time = match time {
                Some(last) if last != &modified => img_edit_job(),
                None => img_edit_job(),
                _ => files_list[index].time,
            };
        }
        if let Mode::Compile = mode {
            return;
        }
        thread::sleep(time::Duration::from_millis(1000))
    }
}

fn file_open(path_str: String) -> Result<DynamicImage, String> {
    let path = Path::new(&path_str);
    image::open(path).set_error(&format!("failed to open file {}", path.display()))
}

fn resize_image(
    img: DynamicImage,
    resize: &Resize,
) -> Result<DynamicImage, String> {
    let filter_type = resize.filter.unwrap_or(FilterType::Gaussian);
    let size = &resize.size;
    // println!(
    //     "updating image file\n{}\nto\n{}\nWith {}\n\n\n",
    //     path_str,
    //     output,
    //     match size {
    //         Size::WidthHeight(x, y) => format!("With as close as possible to width {}px and height {}px while keeping aspect ratio", x, y),
    //         Size::Width(x) => format!("new width {}px", x),
    //         Size::Height(x) => format!("new height {}px", x),
    //     }
    // );
    let size = match size {
        Size::WidthHeight(x, y) => (x, y),
        Size::Width(x) => (x, &u32::max_value()),
        Size::Height(x) => (&u32::max_value(), x),
    };
    let img = img.resize(*size.0, *size.1, filter_type);
    Ok(img)
}

fn save(img: DynamicImage, path: String) {
    img.save(path).unwrap();
}

fn file_share_or_combine(
    settings_one: SharedSettings,
    settings_two: SharedSettings,
) -> SharedSettings {
    let resize = settings_one.jobs.resize.or(settings_two.jobs.resize);
    SharedSettings {
        jobs: ImgEditJobs { resize: resize },
    }
}
