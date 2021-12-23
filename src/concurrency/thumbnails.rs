use error_chain::error_chain;
use error_chain::ChainedError;
use glob::{glob_with, MatchOptions};
use image::imageops::FilterType;
use rayon::prelude::*;
use std::env;
use std::fs::create_dir_all;
use std::path::Path;

error_chain! {
    foreign_links{
        Image(image::ImageError);
        Io(std::io::Error);
        Glob(glob::PatternError);
    }
}

pub fn jpg_thumbnails() -> Result<()> {
    let options: MatchOptions = Default::default();

    let path = Path::new("/sharedfs/myspace/pics");
    env::set_current_dir(&path)?;

    let files: Vec<_> = glob_with("*.jpg", options)?
        .filter_map(|x| x.ok())
        .collect();

    if files.is_empty() {
        error_chain::bail!("No jpg files found!");
    }

    println!("{:?}", files);

    let thumb_dir = "thumbnails";
    create_dir_all(thumb_dir)?;

    println!("Saving {} thumbnails into '{}'...", files.len(), thumb_dir);

    let image_failures: Vec<_> = files
        .par_iter()
        .map(|path| {
            make_thumbnail(path, thumb_dir, 300)
                .map_err(|e| e.chain_err(|| path.display().to_string()))
        })
        .filter_map(|e| e.err())
        .collect();

    image_failures
        .iter()
        .for_each(|x| println!("{}", x.display_chain()));

    println!(
        "{} thumbnails is saved successfully",
        files.len() - image_failures.len()
    );

    Ok(())
}

fn make_thumbnail<PA, PB>(original: PA, thumb_dir: PB, longest_edge: u32) -> Result<()>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
{
    let image = image::open(original.as_ref())?;
    let file_path = thumb_dir.as_ref().join(original);

    Ok(image
        .resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(file_path)?)
}
