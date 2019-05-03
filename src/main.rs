#![deny(clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::enum_glob_use,
    clippy::cast_possible_truncation
)]

mod cli;
mod parse;

use cli::Mode;
use file_watcher::{
    FileListBuilder, WatchedFile,
    WatchingFuncResult::{self, *},
};
use image::{DynamicImage, FilterType};
use set_error::ChangeError;
use std::{iter::Iterator, path::Path, time::SystemTime};

use parse::{parse_config, FileWatch, ImgEditJobs, Resize, SharedSettings, Size};

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
    for file in files_list.clone() {
        file_builder.add_file({
            let mut watched_file = {
                {
                    let temp_file = file.clone();
                    match WatchedFile::new(&file.path.clone(), move |img| {
                        save(&img, temp_file.clone().output.clone())
                    }) {
                        Ok(t) => t,
                        Err(s) => {
                            println!("{}", s);
                            return;
                        }
                    }
                }
            };
            let jobs = (&file.clone()).clone().other.jobs;
            if let Some(x) = jobs.resize {
                let resize_filter = file.other.resize_filter;
                watched_file.add_func(move |img| resize_image(&img, &x.clone(), resize_filter))
            }
            if let Some(x) = jobs.blur {
                watched_file.add_func(move |img| blur_image(&img, x))
            }
            //Sharpen does not work for some reason. Output does not look sharp
            if let Some(x) = jobs.sharpen {
                watched_file.add_func(move |img| {
                    println!("With sharpening level {}\n", x);
                    Success(img.unsharpen(100.0, x))
                })
            }
            if let Some(x) = jobs.adjust_contrast {
                watched_file.add_func(move |img| {
                    println!("With contrast level {}\n", x);
                    Success(img.adjust_contrast(x))
                })
            }
            if let Some(x) = jobs.brighten {
                watched_file.add_func(move |img| {
                    println!("With brightness level {}\n", x);
                    Success(img.brighten(x))
                })
            }
            if let Some(x) = jobs.huerotate {
                watched_file.add_func(move |img| {
                    println!("With hue rotation of {}\n", x);
                    Success(img.huerotate(x))
                })
            }
            if jobs.flipv {
                watched_file.add_func(|img| {
                    println!("And flipping vertically\n");
                    Success(img.flipv())
                })
            }
            if jobs.fliph {
                watched_file.add_func(|img| {
                    println!("And flipping horizontally\n");
                    Success(img.fliph())
                })
            }
            if jobs.rotate90 {
                watched_file.add_func(|img| {
                    println!("And rotating 90 degrees\n");
                    Success(img.rotate90())
                })
            }
            if jobs.rotate180 {
                watched_file.add_func(|img| {
                    println!("And rotating 180 degrees\n");
                    Success(img.rotate180())
                })
            }
            if jobs.rotate270 {
                watched_file.add_func(|img| {
                    println!("And rotating 270 degrees\n");
                    Success(img.rotate270())
                })
            }
            if jobs.grayscale {
                watched_file.add_func(|img| {
                    println!("And changing image to grayscale\n");
                    Success(img.grayscale())
                })
            }
            if jobs.invert {
                watched_file.add_func(|mut img| {
                    println!("And inverting image\n");
                    Success({
                        img.invert();
                        img
                    })
                })
            }
            watched_file
        })
    }
    match file_builder
        .run_only_once(match mode {
            Mode::Compile => true,
            Mode::Watch => false,
        })
        .launch()
    {
        Ok(_) => return,
        Err(s) => {
            println!("Error: {}", s);
            return;
        }
    }
}

fn file_open(path_str: &str) -> WatchingImageFuncResult {
    let path = Path::new(path_str);
    println!("Updating image file \"{}\"\n", path_str,);
    match image::open(path) {
        Ok(t) => Success(t),
        Err(_) => Retry(format!("failed to open file {}", path.display())),
    }
}

fn resize_image(
    img: &DynamicImage,
    resize: &Resize,
    filter: Option<FilterType>,
) -> WatchingImageFuncResult {
    let filter_type = filter.unwrap_or(FilterType::Gaussian);
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

fn blur_image(img: &DynamicImage, blur_amount: f32) -> WatchingImageFuncResult {
    println!("With a blur of {}\n", blur_amount);
    Success(img.blur(blur_amount))
}

fn save(img: &DynamicImage, output_path: String) -> Result<(), String> {
    println!("and saving to \"{}\"\n\n------------\n", output_path,);
    img.save(output_path).set_error("Failed to save.")
}

#[allow(clippy::similar_names)]
fn file_share_or_combine(
    settings_one: SharedSettings,
    settings_two: SharedSettings,
) -> SharedSettings {
    let resize = settings_one.jobs.resize.or(settings_two.jobs.resize);
    let blur = settings_one.jobs.blur.or(settings_two.jobs.blur);
    let sharpen = settings_one.jobs.sharpen.or(settings_two.jobs.sharpen);
    let adjust_contrast = settings_one
        .jobs
        .adjust_contrast
        .or(settings_two.jobs.adjust_contrast);
    let brighten = settings_one.jobs.brighten.or(settings_two.jobs.brighten);
    let huerotate = settings_one.jobs.huerotate.or(settings_two.jobs.huerotate);
    let flipv = settings_one.jobs.flipv || settings_two.jobs.flipv;
    let fliph = settings_one.jobs.fliph || settings_two.jobs.fliph;
    let rotate90 = settings_one.jobs.rotate90 || settings_two.jobs.rotate90;
    let rotate180 = settings_one.jobs.rotate180 || settings_two.jobs.rotate180;
    let rotate270 = settings_one.jobs.rotate270 || settings_two.jobs.rotate270;
    let grayscale = settings_one.jobs.grayscale || settings_two.jobs.grayscale;
    let invert = settings_one.jobs.invert || settings_two.jobs.invert;
    let resize_filter = settings_one.resize_filter.or(settings_two.resize_filter);
    SharedSettings {
        jobs: ImgEditJobs {
            resize,
            blur,
            sharpen,
            adjust_contrast,
            brighten,
            huerotate,
            flipv,
            fliph,
            rotate90,
            rotate180,
            rotate270,
            grayscale,
            invert,
        },
        resize_filter,
    }
}
