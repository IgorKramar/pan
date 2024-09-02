use crate::filesystem::directory_tree::DirectoryTree;

pub fn sumcode_command(project_dir: String, output: String) {
    println!("Running project in directory: {}", project_dir);
    println!("Output will be saved to: {}", output);
    println!();

    let tree = DirectoryTree::new(project_dir);
    tree.print_structure();
}
