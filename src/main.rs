use std::{
    convert::TryFrom,
    fs::{self, File},
    io::prelude::*,
    iter::Iterator,
    path::Path,
    thread, time,
    time::SystemTime,
};
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug, Clone)]
struct FileWatched {
    file: FileWatch,
    time: SystemTime,
}
#[derive(Debug, Clone)]
struct FileWatch {
    path: String,
    output: String,
    size: Size,
}
#[derive(Debug, Clone)]
enum Size {
    Width(u32),
    Height(u32),
}

fn main() {
    let files_list = parse_config().unwrap();

    let mut files_list: Vec<FileWatched> = files_list
        .into_iter()
        .map(|x| FileWatched {
            file: x.clone(),
            time: Path::new(&x.path).metadata().unwrap().modified().unwrap(),
        })
        .collect();
    loop {
        for (index, file) in files_list.clone().iter().enumerate() {
            let modified = Path::new(&file.file.path)
                .metadata()
                .unwrap()
                .modified()
                .unwrap();
            if file.time != modified {
                files_list[index].time = modified;
                println!("updating {}", file.file.path);
                resize_image(&file.file.path, &file.file.output, &file.file.size)
            }
        }
        thread::sleep(time::Duration::from_millis(1000))
    }
    println!("{:#?}", files_list);
    // resize_image("hand_and_book.JPG")
}

fn resize_image(path: &str, output: &str, size: &Size) {
    if fs::remove_file(output).is_err() {
        println!("Failed to remove old file")
    }
    let image_path = Path::new(path);
    let img = image::open(image_path).unwrap();
    let img = match size {
        Size::Width(x) => img.resize(*x, u32::max_value(), image::FilterType::Gaussian),
        Size::Height(x) => img.resize(u32::max_value(), *x, image::FilterType::Gaussian),
    };
    img.save(output).unwrap();
}

fn parse_config() -> Result<Vec<FileWatch>, String> {
    let files_list = {
        YamlLoader::load_from_str(&{
            let mut contents = String::new();

            File::open("image_watcher.yaml")
                .wrap("Failed to open config file.")?
                .read_to_string(&mut contents)
                .wrap("Failed to open read file.")?;
            contents
        })
        .wrap("Failed to parse config file.")?[0]
            .clone()
    }
    .into_hash()
    .wrap("Base of the file not a hash.")?
    .get(&Yaml::String("files".to_string()))
    .wrap("No files section in config file.")?
    .clone()
    .into_vec()
    .wrap("Files section in config is not a list.")?
    .into_iter();
    let mut files_as_hash_list = Vec::new();
    for (index, file) in files_list.enumerate() {
        files_as_hash_list.push(
            file.clone()
                .into_hash()
                .wrap(&format!("file index {} is not a hash", index))?,
        )
    }
    let mut files_list = Vec::new();
    for (index, file) in files_as_hash_list.into_iter().enumerate() {
        files_list.push({
            let path = file
                .get(&Yaml::String("path".to_string()))
                .expect("4")
                .clone()
                .into_string()
                .expect("5");
            FileWatch {
                path: path.clone(),
                output: match file.get(&Yaml::String("output".to_string())) {
                    Some(x) => x.clone().into_string().expect("7"),
                    None => format!(
                        "{}.min.{}",
                        Path::new(&path).file_stem().unwrap().to_str().unwrap(),
                        Path::new(&path).extension().unwrap().to_str().unwrap()
                    ),
                },
                size: match file.get(&Yaml::String("width".to_string())) {
                    Some(x) => {
                        Size::Width(u32::try_from(x.clone().into_i64().expect("7")).unwrap())
                    }
                    None => Size::Height(
                        u32::try_from(
                            file.get(&Yaml::String("height".to_string()))
                                .unwrap()
                                .clone()
                                .into_i64()
                                .expect("7"),
                        )
                        .unwrap(),
                    ),
                },
            }
        })
    }
    Ok(files_list)
}

pub trait WrapError<T> {
    fn wrap(self, s: &str) -> Result<T, String>;
}

impl<T, E> WrapError<T> for Result<T, E> {
    fn wrap(self, s: &str) -> Result<T, String> {
        match self {
            Ok(t) => Ok(t),
            Err(_) => Err(s.to_string()),
        }
    }
}
impl<T> WrapError<T> for Option<T> {
    fn wrap(self, s: &str) -> Result<T, String> {
        match self {
            Some(t) => Ok(t),
            None => Err(s.to_string()),
        }
    }
}
