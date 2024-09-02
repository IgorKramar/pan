use walkdir::{DirEntry, WalkDir};
use glob::Pattern;
use std::path::Path;
use crate::filesystem::file_reader::append_file_content;

pub struct DirectoryTree {
    root: String,
    exclude_paths: Vec<String>,
    exclude_patterns: Vec<Pattern>,
}

impl DirectoryTree {
    pub fn new(root: String, exclude_paths: Vec<String>, exclude_patterns: Vec<String>) -> Self {
        let compiled_patterns = exclude_patterns
            .into_iter()
            .filter_map(|pattern| Pattern::new(&pattern).ok())
            .collect();
        Self {
            root,
            exclude_paths,
            exclude_patterns: compiled_patterns,
        }
    }

    pub fn print_structure(&self) {
        self.collect_directory_structure(&self.root, "", &mut String::new(), None);
    }

    pub fn collect_structure(&self, output: &mut String) {
        self.collect_directory_structure(&self.root, "", output, None);
    }

    pub fn collect_structure_with_content(&self, output: &mut String, extensions: &Vec<String>) {
        self.collect_directory_structure(&self.root, "", output, Some(extensions));
    }

    pub fn collect_file_contents(&self, output: &mut String, extensions: &Vec<String>) {
        self.collect_files_with_content(&self.root, output, extensions);
    }

    fn collect_directory_structure(&self, dir: &str, prefix: &str, output: &mut String, extensions: Option<&Vec<String>>) {
        let mut entries: Vec<DirEntry> = WalkDir::new(dir)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| !self.should_exclude(entry))
            .collect();

        entries.sort_by_key(|e| e.file_name().to_owned());

        for (i, entry) in entries.iter().enumerate() {
            let file_name = entry.file_name().to_string_lossy();
            let new_prefix = if i == entries.len() - 1 { "└── " } else { "├── " };
            output.push_str(&format!("{}{}{}\n", prefix, new_prefix, file_name));

            if entry.file_type().is_dir() {
                let sub_prefix = if i == entries.len() - 1 {
                    format!("{}    ", prefix)
                } else {
                    format!("{}│   ", prefix)
                };
                self.collect_directory_structure(&entry.path().display().to_string(), &sub_prefix, output, extensions);
            } else if entry.file_type().is_file() {
                // Если указаны расширения, добавляем содержимое файлов только с нужными расширениями
                if let Some(exts) = extensions {
                    if let Some(extension) = Path::new(entry.path()).extension() {
                        if exts.iter().any(|ext| ext == &extension.to_string_lossy()) {
                            if let Err(e) = append_file_content(entry.path(), output) {
                                eprintln!("Error reading file {}: {}", entry.path().display(), e);
                            }
                        }
                    }
                }
            }
        }
    }

    fn collect_files_with_content(&self, dir: &str, output: &mut String, extensions: &Vec<String>) {
        let entries: Vec<DirEntry> = WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| !self.should_exclude(entry))
            .collect();

        for entry in entries {
            if let Some(extension) = Path::new(entry.path()).extension() {
                if extensions.iter().any(|ext| ext == &extension.to_string_lossy()) {
                    if let Err(e) = append_file_content(entry.path(), output) {
                        eprintln!("Error reading file {}: {}", entry.path().display(), e);
                    }
                }
            }
        }
    }

    fn should_exclude(&self, entry: &DirEntry) -> bool {
        let entry_path = entry.path().to_string_lossy().to_string();

        // Проверка по списку путей исключений
        if self.exclude_paths.iter().any(|exclude| entry_path.contains(exclude)) {
            return true;
        }

        // Проверка по паттернам
        if self
            .exclude_patterns
            .iter()
            .any(|pattern| pattern.matches(&entry_path))
        {
            return true;
        }

        false
    }
}
