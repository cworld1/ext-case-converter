fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_bundled_translations("lang");
        
    slint_build::compile_with_config("ui/main.slint", config).expect("Slint build failed");
}
