use std::{
    fs::{self, hard_link, DirEntry},
    path::PathBuf,
};

use clap::Args;

use crate::utils::file_type;
use crate::utils::FileType;

#[derive(Args)]
pub struct HardLinkArgs {
    base_path: String,
    dst_path: String,
    #[arg(
        long,
        help = "Weather copy all file instead of just video and subtitles."
    )]
    link_all: bool,
    #[arg(
        short,
        long,
        default_value_t = 10,
        help = "Link File Size Threshold,default is 10 MB."
    )]
    threshold: u64,
    #[arg(long, help = "Pattern used to filter file")]
    pattern: Option<String>,
}

fn filter_video(entry: &DirEntry, threshold: u64, pattern: &Option<String>) -> bool {
    let path = entry.path();
    let meta = entry.metadata().unwrap();
    if meta.len() <= (threshold * 1024 * 1024) {
        return false;
    }
    match pattern {
        Some(pt) => {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if !file_name.contains(pt) {
                return false;
            }
        }
        None => {}
    }

    return true;
}

pub fn hard_link_files(args: &HardLinkArgs) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = PathBuf::from(&args.base_path);
    if !base_path.exists() || !base_path.is_dir() {
        panic!("Wrong Base Path!");
    }
    let dst_path = PathBuf::from(&args.dst_path);
    if !dst_path.exists() || !dst_path.is_dir() {
        panic!("Wrong Dst Path!");
    }
    for entry in fs::read_dir(base_path)? {
        let entry = entry?;
        let path = entry.path();
        let dst_file_path = dst_path.join(entry.file_name());
        let ext = path.extension().unwrap();
        match file_type(ext.to_str().unwrap()) {
            FileType::Video => {
                if filter_video(&entry, args.threshold, &args.pattern) {
                    hard_link(path, dst_file_path)?
                }
            }
            FileType::Subtitle => {
                fs::copy(path, dst_file_path)?;
            }
            _ => {
                if args.link_all {
                    let meta = entry.metadata()?;
                    if meta.len() < args.threshold {
                        fs::copy(path, dst_file_path)?;
                    } else {
                        hard_link(path, dst_file_path)?;
                    }
                }
            }
        }
    }
    Ok(())
}
