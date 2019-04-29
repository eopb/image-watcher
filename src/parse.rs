use super::error_change::ChangeError;

use std::{convert::TryFrom, fmt, fs::File, io::prelude::*, iter::Iterator, path::Path};

use image::FilterType::{self, *};
use yaml_rust::{yaml::Hash, Yaml, YamlLoader};

#[derive(Debug, Clone)]
pub struct Settings {
    pub files_list: Vec<FileWatch>,
    pub other: ShareSettings,
}

#[derive(Debug, Clone)]
pub struct FileWatch {
    pub path: String,
    pub output: String,
    pub other: ShareSettings,
}
#[derive(Debug, Clone)]
pub struct ShareSettings {
    pub jobs: Vec<ImgEditJobs>,
}
#[derive(Debug, Clone)]
pub enum ImgEditJobs {
    Resize(Resize),
}
#[derive(Clone)]
pub struct Resize {
    pub size: Size,
    pub filter: Option<FilterType>,
}
impl fmt::Debug for Resize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Resize {{ size: {:?} }}", self.size)
    }
}

#[derive(Debug, Clone)]
pub enum Size {
    Width(u32),
    Height(u32),
    WidthHeight(u32, u32),
}

pub fn parse_config() -> Result<Settings, String> {
    fn push_some<T>(vec: &mut Vec<T>, value: Option<T>) {
        match value {
            Some(x) => vec.push(x),
            None => (),
        }
    }

    fn get_jobs(yaml: &Hash) -> Result<Vec<ImgEditJobs>, String> {
        let mut jobs = Vec::new();

        push_some(
            &mut jobs,
            match get_size(yaml) {
                Some(x) => Some(ImgEditJobs::Resize(Resize {
                    size: x,
                    filter: resize_filter_getter(
                        yaml.get(&Yaml::String("resize_filter".to_string())),
                    )?,
                })),
                None => None,
            },
        );
        Ok(jobs)
    }
    fn get_size(yaml: &Hash) -> Option<Size> {
        let width = yaml.get(&Yaml::String("width".to_string()));
        let height = yaml.get(&Yaml::String("height".to_string()));
        Some(match (width, height) {
            (Some(width), Some(height)) => Size::WidthHeight(
                u32::try_from(width.clone().into_i64().expect("7")).unwrap(),
                u32::try_from(height.clone().into_i64().expect("7")).unwrap(),
            ),
            (Some(width), None) => {
                Size::Width(u32::try_from(width.clone().into_i64().expect("7")).unwrap())
            }
            (None, Some(height)) => {
                Size::Height(u32::try_from(height.clone().into_i64().expect("7")).unwrap())
            }
            (None, None) => return None,
        })
    }
    fn resize_filter_getter(
        yaml: Option<&yaml_rust::yaml::Yaml>,
    ) -> Result<Option<FilterType>, String> {
        Ok(match yaml {
            Some(x) => {
                let x = x
                    .clone()
                    .into_string()
                    .set_error("Resize_filter not a string.");
                match x.clone()?.as_ref() {
                    "Nearest" => Some(Nearest),
                    "Triangle" => Some(Triangle),
                    "CatmullRom" => Some(CatmullRom),
                    "Gaussian" => Some(Gaussian),
                    "Lanczos3" => Some(Lanczos3),
                    _ => return Err(format!("Unknown resize_filter {}", x?)),
                }
            }
            None => None,
        })
    }
    let open_file = {
        YamlLoader::load_from_str(&{
            let mut contents = String::new();

            File::open("image_watcher.yaml")
                .set_error("Failed to open config file.")?
                .read_to_string(&mut contents)
                .set_error("Failed to open read file.")?;
            contents
        })
        .set_error("Failed to parse config file.")?[0]
            .clone()
    }
    .into_hash()
    .set_error("Base of the file not a hash.")?;
    let files_list = open_file
        .get(&Yaml::String("files".to_string()))
        .set_error("No files section in config file.")?
        .clone()
        .into_vec()
        .set_error("Files section in config is not a list.")?
        .into_iter();
    let mut files_as_hash_list = Vec::new();
    for (index, file) in files_list.enumerate() {
        files_as_hash_list.push(
            file.clone()
                .into_hash()
                .set_error(&format!("file index {} is not a hash", index))?,
        )
    }
    let mut files_list = Vec::new();
    for (index, file) in files_as_hash_list.into_iter().enumerate() {
        files_list.push({
            let path = file
                .get(&Yaml::String("path".to_string()))
                .set_error(&format!("file index {} has no path", index))?
                .clone()
                .into_string()
                .set_error(&format!(
                    "file index {} has a path that is not a string",
                    index
                ))?;
            FileWatch {
                path: path.clone(),
                output: match file.get(&Yaml::String("output".to_string())) {
                    Some(x) => x.clone().into_string().set_error(&format!(
                        "file index {} has a output path that is not a string",
                        index
                    ))?,
                    None => format!(
                        "{}{}.min.{}",
                        {
                            let parent = Path::new(&path).parent().unwrap().to_str().unwrap();
                            if parent.is_empty() {
                                parent.to_string()
                            } else {
                                format!("{}/", parent)
                            }
                        },
                        Path::new(&path).file_stem().unwrap().to_str().unwrap(),
                        Path::new(&path).extension().unwrap().to_str().unwrap()
                    ),
                },
                other: ShareSettings {
                    jobs: get_jobs(&file)?,
                },
            }
        })
    }
    Ok(Settings {
        files_list: files_list,
        other: ShareSettings {
            jobs: get_jobs(&open_file)?,
        },
    })
}
