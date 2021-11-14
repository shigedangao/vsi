extern crate embed_resource;

// Use to enabled compilation of the winrt-notification crate
// @link https://github.com/microsoft/windows-rs/issues/1294
fn main() {
    embed_resource::compile("vsi-manifest.rc");
}