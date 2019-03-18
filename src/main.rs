use std::env;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    // for entry in WalkDir::new(env::current_exe().unwrap().parent().unwrap()) {
    //     let entry = entry.unwrap();
    //     println!("{}", entry.path().display());
    //     println!(
    //         "{:#?}",
    //         entry.path().metadata().unwrap().modified().unwrap()
    //     );
    // }
    let image_path = Path::new("hand_and_book.JPG");
    println!("{:#?}", image_path.metadata().unwrap().modified().unwrap(),);
    let img = image::open(image_path).unwrap();
    let img = img.resize(60, u32::max_value(), image::FilterType::Gaussian);
    img.save("native/test.png").unwrap();
}
