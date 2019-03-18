use std::convert::TryFrom;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use std::{collections::HashMap, fs::File, io::prelude::*, iter::Iterator, str::FromStr};
use std::{thread, time};
use yaml_rust::{yaml::Hash, Yaml, YamlLoader};
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
    let files_list = parse_config();

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
        thread::sleep(time::Duration::from_secs(10))
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

fn parse_config() -> Vec<FileWatch> {
    {
        YamlLoader::load_from_str(&{
            let mut contents = String::new();

            File::open("image_watcher.yaml")
                .unwrap()
                .read_to_string(&mut contents);
            contents
        })
        .expect("1")[0]
            .clone()
    }
    .into_hash()
    .expect("2")
    .get(&Yaml::String("files".to_string()))
    .expect("12")
    .clone()
    .into_vec()
    .expect("7")
    .into_iter()
    .map(|x| x.clone().into_hash().expect("3"))
    .map(|x| FileWatch {
        path: x
            .get(&Yaml::String("path".to_string()))
            .expect("4")
            .clone()
            .into_string()
            .expect("5"),
        output: x
            .get(&Yaml::String("output".to_string()))
            .expect("6")
            .clone()
            .into_string()
            .expect("7"),
        size: match x.get(&Yaml::String("width".to_string())) {
            Some(x) => Size::Width(u32::try_from(x.clone().into_i64().expect("7")).unwrap()),
            None => Size::Height(
                u32::try_from(
                    x.get(&Yaml::String("height".to_string()))
                        .unwrap()
                        .clone()
                        .into_i64()
                        .expect("7"),
                )
                .unwrap(),
            ),
        },
    })
    .collect()
}
