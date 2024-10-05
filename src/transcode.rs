use std::{fs, io, path::PathBuf, process::Command};

use clap::Args;

use crate::utils::{file_type, FileType};

#[derive(Args)]
pub struct TranscodeArgs {
    base_path: String,
    dst_path: String,
}

pub fn transcode_files(args: &TranscodeArgs) -> Result<(), Box<dyn std::error::Error>> {
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
                let output = Command::new("ffmpeg")
                    .args([
                        "-i",
                        path.to_str().expect("src path wrong"),
                        "-c:v libx265",
                        "-map 0:v",
                        "-c:a copy",
                        "-map 0:a",
                        "-c:s copy",
                        "-map 0:s",
                    ])
                    .output()
                    .expect("Failed to execute command");
                println!("For File {} Transcode Done!", path.display());
                println!(
                    "Output Status {} Out {:?} Err {:?}",
                    &output.status, &output.stdout, &output.stderr
                );
                // io::stdout().write_all(&output.stdout).unwrap();
                // io::stderr().write_all(&output.stderr).unwrap();
            }
            _ => {
                fs::copy(path, dst_file_path)?;
            }
        }
    }
    Ok(())
}
