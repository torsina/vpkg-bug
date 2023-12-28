#[cfg(target_os = "windows")]
fn main() {
    vcpkg::find_package("openssl").unwrap();
}