mod cli;
mod error_change;
mod parse;

use cli::Mode;
use error_change::ChangeError;
use image::FilterType;
use std::{
    iter::Iterator,
    path::Path,
    thread,
    time::{self, SystemTime},
};

use parse::{parse_config, FileWatch, Size};

#[derive(Clone)]
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
            let filter_type = file.file.resize_filter.unwrap_or(FilterType::Gaussian);
            let mut resize_func = || files_list[index].time = Some(modified);
            resize_image(
                &file.file.path,
                &file.file.output,
                &file.file.size,
                filter_type,
            )
            .unwrap();
            match file.time {
                Some(last) if last != modified => resize_func(),
                None => resize_func(),
                _ => (),
            };
        }
        if let Mode::Compile = mode {
            return;
        }
        thread::sleep(time::Duration::from_millis(1000))
    }
}

fn resize_image(
    path: &str,
    output: &str,
    size: &Size,
    filter_type: FilterType,
) -> Result<(), String> {
    println!("updating {} to {}", path, output);
    let path = Path::new(path);
    let img = image::open(path).set_error(&format!("failed to open file {}", path.display()))?;
    let size = match size {
        Size::WidthHeight(x, y) => (*x, *y),
        Size::Width(x) => (*x, u32::max_value()),
        Size::Height(x) => (u32::max_value(), *x),
    };
    println!("{:?}", size);
    let img = img.resize(size.0, size.1, filter_type);
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
