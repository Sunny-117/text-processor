use colored::*;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

pub fn generate_srt_files(input_dir: &Path, output_dir: &Path) -> io::Result<()> {
    let srt_files: Vec<_> = fs::read_dir(input_dir)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("srt") {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect();

    fs::create_dir_all(output_dir)?;

    for (index, srt_file) in srt_files.iter().enumerate() {
        let mut contents = String::new();
        File::open(srt_file)?.read_to_string(&mut contents)?;

        let output_file_path = output_dir.join(format!("file_{}.srt", index + 1));
        let mut output_file = File::create(output_file_path)?;
        output_file.write_all(contents.as_bytes())?;
    }

    println!(
        "{}",
        "All .srt files have been processed and saved to the 'output' directory.".green()
    );

    Ok(())
}

pub fn combine_srt_files_to_markdown(output_dir: &Path) -> io::Result<()> {
    let combined_file_path = output_dir.join("combined.md");

    let mut combined_file = File::create(&combined_file_path)?;

    if output_dir.exists() && output_dir.is_dir() {
        for entry in fs::read_dir(output_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown");
                let mut contents = String::new();
                File::open(&path)?.read_to_string(&mut contents)?;
                writeln!(combined_file, "# {}\n", file_name)?;
                writeln!(combined_file, "{}", contents)?;
            }
        }
    }

    println!(
        "{}",
        "All files have been combined into 'combined.md'.".green()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;

    fn setup_test_env() -> (PathBuf, PathBuf) {
        let input_dir = PathBuf::from(".test/input");
        let output_dir = PathBuf::from(".test/output");
        fs::create_dir_all(&input_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();

        // Create some test .srt files
        for i in 1..=3 {
            let file_path = input_dir.join(format!("file_{}.srt", i));
            let mut file = File::create(file_path).unwrap();
            writeln!(
                file,
                "1\n00:00:00,000 --> 00:00:02,000\nSubtitle text {}",
                i
            )
            .unwrap();
        }

        (input_dir, output_dir)
    }

    fn cleanup_test_env(input_dir: &Path, output_dir: &Path) {
        fs::remove_dir_all(input_dir).unwrap();
        fs::remove_dir_all(output_dir).unwrap();
    }

    #[test]
    fn test_generate_srt_files() {
        let (input_dir, output_dir) = setup_test_env();

        let result = generate_srt_files(&input_dir, &output_dir);
        assert!(result.is_ok());

        // Check that the output files are created
        for i in 1..=3 {
            let output_file_path = output_dir.join(format!("file_{}.srt", i));
            assert!(output_file_path.exists());
        }

        cleanup_test_env(&input_dir, &output_dir);
    }

    #[test]
    fn test_combine_srt_files_to_markdown() {
        test_generate_srt_files();
        let (input_dir, output_dir) = setup_test_env();
        generate_srt_files(&input_dir, &output_dir).unwrap();

        let result = combine_srt_files_to_markdown(&output_dir);
        assert!(result.is_ok());

        // Check that the combined markdown file is created
        let combined_file_path = output_dir.join("combined.md");
        assert!(combined_file_path.exists());

        // Verify the contents of the combined file
        let mut combined_contents = String::new();
        File::open(combined_file_path)
            .unwrap()
            .read_to_string(&mut combined_contents)
            .unwrap();

        for i in 1..=3 {
            assert!(combined_contents.contains(&format!("# file_{}.srt\n", i)));
            assert!(combined_contents.contains(&format!("Subtitle text {}", i)));
        }

        cleanup_test_env(&input_dir, &output_dir);
    }
}
