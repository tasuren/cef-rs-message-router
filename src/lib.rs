#[cfg(any(
    not(target_os = "windows"),
    all(target_os = "windows", feature = "sandbox")
))]
pub mod shared;
pub mod tests_shared;

#[cfg(target_os = "macos")]
mod mac;
#[cfg(all(target_os = "windows", feature = "sandbox"))]
mod win;
