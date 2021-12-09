#[cfg(unix)]
pub mod unix;
#[cfg(windows)]
pub mod windows;

#[derive(Debug)]
struct ProcessInfo {
    parent_id: Option<u32>,
    command: String,
}
