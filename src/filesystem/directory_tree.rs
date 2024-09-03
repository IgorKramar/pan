use walkdir::{DirEntry, WalkDir};
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::Path;
use crate::filesystem::file_reader::append_file_content;

pub struct DirectoryTree {
    root: String,
    exclude_paths: Vec<String>,
    exclude_patterns: GlobSet,
}

impl DirectoryTree {
    pub fn new(root: String, exclude_paths: Vec<String>, exclude_patterns: Vec<String>) -> Self {
        let mut builder = GlobSetBuilder::new();
        for pattern in exclude_patterns {
            if let Ok(glob) = Glob::new(&pattern) {
                builder.add(glob);
            }
        }
        let compiled_patterns = builder.build().expect("Failed to build glob set");
        
        Self {
            root,
            exclude_paths,
            exclude_patterns: compiled_patterns,
        }
    }

    pub fn collect(&self, output: &mut String, options: CollectOptions) {
        self.collect_directory_structure(&self.root, "", output, &options);
    }

    fn collect_directory_structure(&self, dir: &str, prefix: &str, output: &mut String, options: &CollectOptions) {
        let entries = self.get_sorted_entries(dir);

        for (i, entry) in entries.iter().enumerate() {
            let file_name = entry.file_name().to_string_lossy();
            let new_prefix = if i == entries.len() - 1 { "└── " } else { "├── " };
            output.push_str(&format!("{}{}{}\n", prefix, new_prefix, file_name));

            if entry.file_type().is_dir() {
                self.process_directory(entry, prefix, output, options, i == entries.len() - 1);
            } else if entry.file_type().is_file() {
                self.process_file(entry, output, options);
            }
        }
    }

    fn process_directory(&self, entry: &DirEntry, prefix: &str, output: &mut String, options: &CollectOptions, is_last: bool) {
        let sub_prefix = if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };
        self.collect_directory_structure(&entry.path().display().to_string(), &sub_prefix, output, options);
    }

    fn process_file(&self, entry: &DirEntry, output: &mut String, options: &CollectOptions) {
        if let Some(exts) = &options.extensions {
            if let Some(extension) = entry.path().extension() {
                if exts.iter().any(|ext| ext == &extension.to_string_lossy()) {
                    if options.include_content {
                        if let Err(e) = append_file_content(entry.path(), output) {
                            eprintln!("Error reading file {}: {}", entry.path().display(), e);
                        }
                    }
                }
            }
        }
    }

    fn get_sorted_entries(&self, dir: &str) -> Vec<DirEntry> {
        let mut entries: Vec<DirEntry> = WalkDir::new(dir)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| !self.should_exclude(entry))
            .collect();

        entries.sort_by_key(|e| e.file_name().to_owned());
        entries
    }

    fn should_exclude(&self, entry: &DirEntry) -> bool {
        let entry_path = entry.path().to_string_lossy().to_string();

        if self.exclude_paths.iter().any(|exclude| entry_path.contains(exclude)) {
            return true;
        }

        if self.exclude_patterns.is_match(&entry_path) {
            return true;
        }

        false
    }
}

pub struct CollectOptions {
    pub include_structure: bool,
    pub include_content: bool,
    pub extensions: Option<Vec<String>>,
}

impl CollectOptions {
    pub fn new(include_structure: bool, include_content: bool, extensions: Option<Vec<String>>) -> Self {
        Self {
            include_structure,
            include_content,
            extensions,
        }
    }
}
