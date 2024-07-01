mod file_ops;

use file_ops::{generate_srt_files, combine_srt_files_to_markdown};


fn main() -> std::io::Result<()> {
    generate_srt_files()?;
    combine_srt_files_to_markdown()?;
    Ok(())
}
