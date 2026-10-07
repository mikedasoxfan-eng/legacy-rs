//! Windows file properties for beta.exe: icon, product name and version.
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("../../assets/icon.ico")
            .set("ProductName", "legacy-rs")
            .set("FileDescription", "Minecraft Beta 1.7.3 (Rust port)")
            .set("LegalCopyright", "Unofficial fan project. Minecraft is a trademark of Mojang Synergies AB.");
        if let Err(e) = res.compile() {
            println!("cargo:warning=no Windows resources: {e}");
        }
    }
    println!("cargo:rerun-if-changed=../../assets/icon.ico");
}
