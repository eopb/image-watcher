use set_error::ChangeError;

use std::{
    convert::TryFrom, fmt, fs::File, io::prelude::*, iter::Iterator, path::Path, string::ToString,
};

use image::FilterType::{self, *};
use yaml_rust::{yaml::Hash, Yaml, YamlLoader};

#[derive(Debug, Clone)]
pub struct Settings {
    pub files_list: Vec<FileWatch>,
    pub other: SharedSettings,
}

#[derive(Debug, Clone)]
pub struct FileWatch {
    pub path: String,
    pub output: String,
    pub other: SharedSettings,
}
#[derive(Clone)]
pub struct SharedSettings {
    pub jobs: ImgEditJobs,
    pub resize_filter: Option<FilterType>,
}
impl fmt::Debug for SharedSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SharedSettings {{ jobs: {:?} }}", self.jobs)
    }
}

#[derive(Debug, Clone)]
pub struct ImgEditJobs {
    pub resize: Option<Resize>,
    pub blur: Option<f32>,
    pub sharpen: Option<i32>,
    pub adjust_contrast: Option<f32>,
    pub brighten: Option<i32>,
    pub huerotate: Option<i32>,
    pub flipv: bool,
    pub fliph: bool,
    pub rotate90: bool,
    pub rotate180: bool,
    pub rotate270: bool,
    pub grayscale: bool,
    pub invert: bool,
}
#[derive(Debug, Clone)]
pub struct Resize {
    pub size: Size,
}

#[derive(Debug, Clone)]
pub enum Size {
    Width(u32),
    Height(u32),
    WidthHeight(u32, u32),
}

pub fn parse_config() -> Result<Settings, String> {
    fn get_jobs(yaml: &Hash) -> Result<ImgEditJobs, String> {
        fn get_i32(yaml: &Hash, field: &str) -> Result<Option<i32>, String> {
            Ok(match yaml.get(&Yaml::String(field.to_string())) {
                Some(x) => Some({
                    x.clone()
                        .as_i64()
                        .and_then(|x| i32::try_from(x).ok())
                        .set_error(&format!("{} value is valid: Not a valid number", field))?
                }),
                None => None,
            })
        }
        fn get_bool(yaml: &Hash, field: &str) -> Result<bool, String> {
            Ok(match yaml.get(&Yaml::String(field.to_string())) {
                Some(x) => x
                    .clone()
                    .into_bool()
                    .set_error(&format!("{} value is valid: Not true or false.", field))?,
                None => false,
            })
        }
        fn get_float(yaml: &Hash, field: &str) -> Result<Option<f32>, String> {
            Ok(match yaml.get(&Yaml::String(field.to_string())) {
                Some(x) => Some({
                    let f = x
                        .clone()
                        .into_f64()
                        .set_error(&format!("{} value is valid: Not Float", field))?;
                    if f < f64::from(core::f32::MAX) {
                        f as f32
                    } else {
                        core::f32::MAX
                    }
                }),
                None => None,
            })
        }

        Ok(ImgEditJobs {
            resize: {
                match get_size(yaml)? {
                    Some(x) => Some(Resize { size: x }),
                    None => None,
                }
            },
            blur: get_float(yaml, "blur")?,
            sharpen: get_i32(yaml, "sharpen")?,
            adjust_contrast: get_float(yaml, "contrast")?,
            brighten: get_i32(yaml, "brighten")?,
            huerotate: get_i32(yaml, "huerotate")?,
            flipv: get_bool(yaml, "flipv")?,
            fliph: get_bool(yaml, "fliph")?,
            rotate90: get_bool(yaml, "rotate90")?,
            rotate180: get_bool(yaml, "rotate180")?,
            rotate270: get_bool(yaml, "rotate270")?,
            grayscale: get_bool(yaml, "grayscale")?,
            invert: get_bool(yaml, "invert")?,
        })
    }

    fn get_size(yaml: &Hash) -> Result<Option<Size>, String> {
        fn get_u32(yaml: &Hash, field: &str) -> Result<Option<u32>, String> {
            Ok(match yaml.get(&Yaml::String(field.to_string())) {
                Some(x) => Some({
                    x.clone()
                        .as_i64()
                        .and_then(|x| u32::try_from(x).ok())
                        .set_error(&format!("{} value is valid: Not a valid number", field))?
                }),
                None => None,
            })
        }
        let width = get_u32(yaml, "width")?;
        let height = get_u32(yaml, "height")?;
        Ok(Some(match (width, height) {
            (Some(width), Some(height)) => Size::WidthHeight(width, height),
            (Some(width), None) => Size::Width(width),
            (None, Some(height)) => Size::Height(height),
            (None, None) => return Ok(None),
        }))
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
                            let parent = Path::new(&path)
                                .parent()
                                .and_then(|x| x.to_str())
                                .set_error(&format!(
                                    "file index {} has a output path with invalid parent.",
                                    index
                                ))?;
                            if parent.is_empty() {
                                parent.to_string()
                            } else {
                                format!("{}/", parent)
                            }
                        },
                        Path::new(&path)
                            .file_stem()
                            .and_then(|x| x.to_str())
                            .set_error(&format!(
                                "file index {} has a output path with invalid file stem.",
                                index
                            ))?,
                        Path::new(&path)
                            .extension()
                            .and_then(|x| x.to_str())
                            .set_error(&format!(
                                "file index {} has a output path with invalid extension.",
                                index
                            ))?
                    ),
                },
                other: SharedSettings {
                    jobs: get_jobs(&file)?,
                    resize_filter: resize_filter_getter(
                        file.get(&Yaml::String("resize_filter".to_string())),
                    )?,
                },
            }
        })
    }
    Ok(Settings {
        files_list,
        other: SharedSettings {
            jobs: get_jobs(&open_file)?,
            resize_filter: resize_filter_getter(
                open_file.get(&Yaml::String("resize_filter".to_string())),
            )?,
        },
    })
}
