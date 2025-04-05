use minijinja::Environment;

fn main() {
    // Check if current directory has a Cargo.toml with [workspace]
    let cargo_toml_path = std::env::current_dir().unwrap().join("Cargo.toml");
    let cargo_toml_content =
        std::fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");
    if !cargo_toml_content.contains("[workspace]") {
        panic!(
            "Cargo.toml does not contain [workspace] (you must run codegen from the workspace root)"
        );
    }

    // Initialize minijinja environment
    let mut env = Environment::empty();

    // Load the template from file
    let template_path = "shapely-codegen/src/tuples_impls.rs.j2";
    let template_content =
        std::fs::read_to_string(template_path).expect("Failed to read template file");

    // Add the template to the environment
    env.add_template("tuples_impls", &template_content)
        .expect("Failed to add template");

    // Get the template
    let template = env
        .get_template("tuples_impls")
        .expect("Failed to get template");

    // Render the template with context
    let output = template
        .render(minijinja::context! {
            max_tuple_size => 12
        })
        .expect("Failed to render template");

    let path = "shapely-trait/src/impls/tuples_impls.rs";
    std::fs::write(path, output).expect("Failed to write file");

    // Run rustfmt on the generated file
    let status = std::process::Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .arg(path)
        .status()
        .expect("Failed to execute rustfmt");

    if !status.success() {
        eprintln!("rustfmt failed with exit code: {}", status);
    }
}
