use crate::filesystem::directory_tree::DirectoryTree;
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

    // Печатаем исключаемые пути и паттерны
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

    // Сначала собираем структуру проекта
    tree.collect_structure(&mut output_content);

    // Если указаны расширения, добавляем содержимое файлов после структуры
    if let Some(ref extensions) = args.with_file_content {
        tree.collect_file_contents(&mut output_content, extensions);
    }

    // Если указан output, записываем в файл, иначе выводим в консоль
    if let Some(ref output_path) = args.output {
        if let Err(e) = write_to_file(output_path, &output_content) {
            eprintln!("Error writing to file: {}", e);
        }
    } else {
        // Выводим в консоль
        println!("{}", output_content);
    }
}

/// Разделяет строку на пути и паттерны
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
