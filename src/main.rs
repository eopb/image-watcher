use std::env;
use std::path::Path;
use std::{collections::HashMap, fs::File, io::prelude::*, iter::Iterator, str::FromStr};
use yaml_rust::{yaml::Hash, Yaml, YamlLoader};

#[derive(Debug)]
struct FileWatch {
    path: String,
    output: String,
}

fn main() {
    let mut files_list = Vec::new();
    for file in {
        YamlLoader::load_from_str(&{
            let mut contents = String::new();

            File::open("image_watcher.yaml")
                .unwrap()
                .read_to_string(&mut contents);
            contents
        })
        .unwrap()[0]
            .clone()
    }
    .into_hash()
    .unwrap()
    .get(&Yaml::String("files".to_string()))
    .map(|x| x.clone().into_hash().unwrap())
    {
        files_list.push(FileWatch {
            path: file
                .get(&Yaml::String("path".to_string()))
                .unwrap()
                .into_string()
                .unwrap(),

            output: file
                .get(&Yaml::String("output".to_string()))
                .unwrap()
                .into_string()
                .unwrap(),
        })
    }
    println!("{:#?}", files_list);
    // resize_image("hand_and_book.JPG")
}

fn resize_image(path: &str) {
    let image_path = Path::new(path);
    println!("{:#?}", image_path.metadata().unwrap().modified().unwrap());
    let img = image::open(image_path).unwrap();
    let img = img.resize(60, u32::max_value(), image::FilterType::Gaussian);
    img.save("native/test.png").unwrap();
}
