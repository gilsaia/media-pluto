use std::{
    fs::{self, hard_link, DirEntry},
    path::PathBuf,
};

use clap::Args;

#[derive(Args)]
pub struct HardLinkArgs {
    base_path: String,
    dst_path: String,
    #[arg(long, help = "Weather copy all file instead of just video.")]
    copy_all: bool,
    #[arg(
        short,
        long,
        default_value_t = 100,
        help = "Copy File Size Threshold,default is 100 MB."
    )]
    threshold: u64,
    #[arg(long, help = "Pattern used to filter file")]
    pattern: Option<String>,
}

fn filter_file(entry: &DirEntry, copy_all: bool, threshold: u64, pattern: &Option<String>) -> bool {
    let path = entry.path();
    let ext = path.extension().unwrap();
    if !copy_all && !(ext == "mp4" || ext == "mkv" || ext == "avi" || ext == "mov" || ext == "wmv")
    {
        return false;
    }
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
        if filter_file(&entry, args.copy_all, args.threshold, &args.pattern) {
            hard_link(entry.path(), dst_path.join(entry.file_name()))?;
        }
    }
    Ok(())
}
