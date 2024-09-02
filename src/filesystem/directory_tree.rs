use walkdir::{DirEntry, WalkDir};

pub struct DirectoryTree {
    root: String,
}

impl DirectoryTree {
    pub fn new(root: String) -> Self {
        Self { root }
    }

    pub fn print_structure(&self) {
        self.print_directory_structure(&self.root, "");
    }

    fn print_directory_structure(&self, dir: &str, prefix: &str) {
        let mut entries: Vec<DirEntry> = WalkDir::new(dir)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .collect();

        entries.sort_by_key(|e| e.file_name().to_owned());

        for (i, entry) in entries.iter().enumerate() {
            let file_name = entry.file_name().to_string_lossy();
            let new_prefix = if i == entries.len() - 1 { "└── " } else { "├── " };
            println!("{}{}{}", prefix, new_prefix, file_name);

            if entry.file_type().is_dir() {
                let sub_prefix = if i == entries.len() - 1 {
                    format!("{}    ", prefix)
                } else {
                    format!("{}│   ", prefix)
                };
                self.print_directory_structure(&entry.path().display().to_string(), &sub_prefix);
            }
        }
    }
}
