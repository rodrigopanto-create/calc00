#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    calc00::run()
}

#[cfg(target_arch = "wasm32")]
fn main() {}
