#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::enum_glob_use)]

mod cli;
mod parse;

use cli::Mode;
use file_watcher::{
    FileListBuilder, WatchedFile,
    WatchingFuncResult::{self, *},
};
use image::{DynamicImage, FilterType};
use set_error::ChangeError;
use std::{
    iter::Iterator,
    path::Path,
    thread,
    time::{self, SystemTime},
};

use parse::{parse_config, FileWatch, ImgEditJobs, Resize, Settings, SharedSettings, Size};

type WatchingImageFuncResult = WatchingFuncResult<DynamicImage>;

#[derive(Clone)]
struct FileWatched {
    file: FileWatch,
    time: Option<SystemTime>,
}

fn main() {
    let mode = Mode::get();
    println!("Parsing config file image_watcher.yaml\n");
    let config = match parse_config() {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let files_list: Vec<FileWatch> = config
        .files_list
        .clone()
        .into_iter()
        .map(|x| FileWatch {
            other: file_share_or_combine(x.other.clone(), config.other.clone()),
            ..x.clone()
        })
        .collect();
    let mut file_builder = FileListBuilder::new(file_open);
    for (index, file) in files_list.clone().into_iter().enumerate() {
        file_builder.add_file({
            let mut watched_file = {
                {
                    let temp_file = file.clone();
                    match WatchedFile::new(&file.path.clone(), move |img| {
                        save(img, temp_file.clone().output.clone())
                    }) {
                        Ok(t) => t,
                        Err(s) => {
                            println!("{}", s);
                            return;
                        }
                    }
                }
            };
            match (&file.clone()).clone().other.jobs.resize.clone() {
                Some(x) => {
                    let temp_file = file.clone();
                    watched_file.add_func(move |img| resize_image(img, x.clone()))
                }
                None => (),
            }
            watched_file
        })
    }
    file_builder
        .run_only_once(match mode {
            Mode::Compile => true,
            Mode::Watch => false,
        })
        .launch()
        .unwrap()
}

fn file_open(path_str: &str) -> WatchingImageFuncResult {
    let path = Path::new(path_str);
    println!("Updating image file \"{}\"\n", path_str,);
    match image::open(path) {
        Ok(t) => Success(t),
        Err(_) => Retry(format!("failed to open file {}", path.display())),
    }
}

fn resize_image(img: DynamicImage, resize: Resize) -> WatchingImageFuncResult {
    let filter_type = resize.filter.unwrap_or(FilterType::Gaussian);
    let size = &resize.size;
    println!(
        "With {}\n",
        match size {
            Size::WidthHeight(x, y) => format!(
                "as close as possible to width {}px and height {}px while keeping aspect ratio",
                x, y
            ),
            Size::Width(x) => format!("new width {}px", x),
            Size::Height(x) => format!("new height {}px", x),
        }
    );
    let size = match size {
        Size::WidthHeight(x, y) => (x, y),
        Size::Width(x) => (x, &u32::max_value()),
        Size::Height(x) => (&u32::max_value(), x),
    };
    let img = img.resize(*size.0, *size.1, filter_type);
    Success(img)
}

fn save(img: DynamicImage, output_path: String) -> Result<(), String> {
    println!("and saving to \"{}\"\n\n------------\n", output_path,);
    img.save(output_path).set_error("Failed to save.")
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
