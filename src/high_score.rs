#[cfg(target_arch = "wasm32")]
pub fn load() -> u32 {
    unsafe { flappy_high_score_load() }
}

#[cfg(target_arch = "wasm32")]
pub fn save(score: u32) {
    unsafe {
        flappy_high_score_save(score);
    }
}

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn flappy_high_score_load() -> u32;
    fn flappy_high_score_save(score: u32);
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn flappy_storage_crate_version() -> u32 {
    1
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load() -> u32 {
    score_path()
        .and_then(|path| std::fs::read_to_string(path).ok())
        .and_then(|value| value.trim().parse::<u32>().ok())
        .unwrap_or(0)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save(score: u32) {
    if let Some(path) = score_path() {
        let _ = std::fs::write(path, score.to_string());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn score_path() -> Option<std::path::PathBuf> {
    std::env::var_os("HOME")
        .map(|home| std::path::PathBuf::from(home).join(".flappy_rust_high_score"))
        .or_else(|| {
            std::env::current_dir()
                .ok()
                .map(|dir| dir.join(".flappy_rust_high_score"))
        })
}
