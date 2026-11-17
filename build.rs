fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|target_os| target_os == "windows") {
        winresource::WindowsResource::new()
            .set_icon("assets/icon.ico")
            .compile()
            .expect("failed to embed the executable icon");
    }
}
