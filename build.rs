#[cfg(target_os = "windows")]
extern crate winres;

#[cfg(target_os = "windows")]
fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("resources/icon.ico");
        match res.compile() {
            Ok(_) => {}
            Err(_) => {
                println!("File \"resources/icon.ico\" not found. No icon set for app.");
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn main() {}
