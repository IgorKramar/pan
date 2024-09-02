use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Функция для записи данных в файл
pub fn write_to_file<P: AsRef<Path>>(output_path: P, content: &str) -> io::Result<()> {
    let mut file = File::create(output_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}