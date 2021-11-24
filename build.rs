extern crate embed_resource;
extern crate winres;

// Use to enabled compilation of the winrt-notification crate
// @link https://github.com/microsoft/windows-rs/issues/1294
fn main() {
    if cfg!(target_os = "windows") {
        embed_resource::compile("vsi-manifest.rc");
        let mut res = winres::WindowsResource::new();
        res.set_icon("plane-blue.ico");
        res.compile().unwrap();
    }
}