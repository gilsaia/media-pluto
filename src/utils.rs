pub enum FileType {
    Video,
    Subtitle,
    None,
}

pub fn file_type(ext: &str) -> FileType {
    return match ext {
        "mp4" | "mkv" | "avi" | "mov" | "wmv" => FileType::Video,
        "srt" => FileType::Subtitle,
        _ => FileType::None,
    };
}
