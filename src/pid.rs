//! Contains the process sub-directory files within the proc file system.
//!
//! Each file has its own submodule with the same name.
//!
//! Tese submodules is re-expored.
//!

define_modules! {
    task "task";
    stat "pid_stat";
    comm "pid_comm";
}