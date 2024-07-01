use std::fs::{self, File};
use std::io::{self, Read, Write};

pub fn generate_srt_files() -> io::Result<()> {
    let c = std::env::current_dir()?;
    let current_dir = c.join("input");
    let srt_files: Vec<_> = fs::read_dir(&current_dir)?
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

    let output_dir = c.join("output");
    fs::create_dir_all(&output_dir)?;

    for (index, srt_file) in srt_files.iter().enumerate() {
        let mut contents = String::new();
        File::open(srt_file)?.read_to_string(&mut contents)?;

        let output_file_path = output_dir.join(format!("file_{}.srt", index + 1));
        let mut output_file = File::create(output_file_path)?;
        output_file.write_all(contents.as_bytes())?;
    }

    println!("All .srt files have been processed and saved to the 'output' directory.");

    Ok(())
}

pub fn combine_srt_files_to_markdown() -> io::Result<()> {
    let c = std::env::current_dir()?;
    let output_dir = c.join("output");
    let combined_file_path = output_dir.join("combined.md");

    let mut combined_file = File::create(&combined_file_path)?;

    if output_dir.exists() && output_dir.is_dir() {
        for entry in fs::read_dir(output_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("Unknown");
                let mut contents = String::new();
                File::open(&path)?.read_to_string(&mut contents)?;
                writeln!(combined_file, "# {}\n", file_name)?;
                writeln!(combined_file, "{}", contents)?;
            }
        }
    }

    println!("All files have been combined into 'combined.md'.");

    Ok(())
}
