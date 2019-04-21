mod cli;
mod error_change;
mod parse;

use cli::Mode;
use error_change::ChangeError;
use parse::{parse_config, FileWatch, Size};
use std::{
    convert::TryFrom,
    fs::File,
    io::prelude::*,
    iter::Iterator,
    path::Path,
    str::FromStr,
    thread,
    time::{self, SystemTime},
};

#[derive(Debug, Clone)]
struct FileWatched {
    file: FileWatch,
    time: Option<SystemTime>,
}

fn main() {
    let mode = Mode::get();
    println!("Parsing config file image_watcher.yaml");
    let files_list = match parse_config() {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let mut files_list: Vec<FileWatched> = files_list
        .into_iter()
        .map(|x| FileWatched {
            file: x.clone(),
            time: None,
        })
        .collect();
    loop {
        for (index, file) in files_list.clone().iter().enumerate() {
            let modified = match when_modified(Path::new(&file.file.path)) {
                Ok(s) => s,
                Err(_) => return,
            };
            match file.time {
                Some(last) => {
                    if last != modified {
                        files_list[index].time = Some(modified);
                        resize_image(&file.file.path, &file.file.output, &file.file.size).unwrap()
                    };
                }
                None => {
                    files_list[index].time = Some(modified);
                    resize_image(&file.file.path, &file.file.output, &file.file.size).unwrap()
                }
            };
        }
        if let Mode::Compile = mode {
            return;
        }
        thread::sleep(time::Duration::from_millis(1000))
    }
}

fn resize_image(path: &str, output: &str, size: &Size) -> Result<(), String> {
    println!("updating {} to {}", path, output);
    let path = Path::new(path);
    let img = image::open(path).set_error(&format!("failed to open file {}", path.display()))?;
    let size = match size {
        Size::WidthHeight(x, y) => (*x, *y),
        Size::Width(x) => (*x, u32::max_value()),
        Size::Height(x) => (u32::max_value(), *x),
    };
    println!("{:?}", size);
    let img = img.resize(size.0, size.1, image::FilterType::Gaussian);
    img.save(output).unwrap();
    Ok(())
}

fn when_modified(path: &Path) -> Result<SystemTime, String> {
    Ok::<_, String>(
        Path::new(path)
            .metadata()
            .set_error(&format!("failed to open file {} metadata", path.display()))?
            .modified()
            .set_error(&format!(
                "failed to find files date modifide {}",
                path.display()
            )),
    )?
}
