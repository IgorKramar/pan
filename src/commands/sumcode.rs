use crate::filesystem::directory_tree::{DirectoryTree, CollectOptions};
use crate::filesystem::file_writer::write_to_file;
use crate::cli::args::SumcodeArgs;

pub fn sumcode_command(args: &SumcodeArgs) {
    println!("Running project in directory: {}", args.project_dir);

    if let Some(ref output) = args.output {
        println!("Output will be saved to: {}", output);
    } else {
        println!("Output will be displayed in console.");
    }
    println!();

    let (exclude_paths, exclude_patterns) = split_paths_and_patterns(&args.exclude);

    if !exclude_paths.is_empty() {
        println!("Exclude paths:");
        for path in &exclude_paths {
            println!("- {}", path);
        }
    }

    if !exclude_patterns.is_empty() {
        println!("Exclude patterns:");
        for pattern in &exclude_patterns {
            println!("- {}", pattern);
        }
    }

    let mut output_content = String::new();
    let tree = DirectoryTree::new(
        args.project_dir.clone(),
        exclude_paths,
        exclude_patterns,
    );

    let options = CollectOptions::new(
        true, // Собираем структуру
        args.with_file_content.is_some(), // Включаем содержимое файлов, если указаны расширения
        args.with_file_content.clone(),
    );

    tree.collect(&mut output_content, options);

    if let Some(ref output_path) = args.output {
        if let Err(e) = write_to_file(output_path, &output_content) {
            eprintln!("Error writing to file: {}", e);
        }
    } else {
        println!("{}", output_content);
    }
}

fn split_paths_and_patterns(paths: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut exclude_paths = Vec::new();
    let mut exclude_patterns = Vec::new();

    for path in paths {
        let parts: Vec<&str> = path.split(';').collect();
        for part in parts {
            if part.contains('*') {
                exclude_patterns.push(part.to_string());
            } else {
                exclude_paths.push(part.to_string());
            }
        }
    }

    (exclude_paths, exclude_patterns)
}
