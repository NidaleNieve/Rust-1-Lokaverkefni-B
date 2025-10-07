#[cfg(windows)]
fn main() {
    // Embed Windows icon if provided at assets/app_icon.ico
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/app_icon.ico");
    let _ = res.compile();
}

#[cfg(not(windows))]
fn main() {}
