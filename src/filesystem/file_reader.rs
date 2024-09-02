use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub fn append_file_content<P: AsRef<Path>>(file_path: P, output_content: &mut String) -> io::Result<()> {
    // Добавляем путь файла
    output_content.push_str(&format!("\n// {}\n", file_path.as_ref().display()));

    // Читаем содержимое файла
    let mut file = fs::File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Добавляем содержимое файла
    output_content.push_str(&content);
    output_content.push_str("\n");  // Добавляем новую строку после содержимого

    Ok(())
}
