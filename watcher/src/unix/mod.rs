use std::ffi::CString;
use std::io::Error;
use std::mem;

pub const STRUCT_SIZE: usize = mem::size_of::<InotifyEvent>();

pub const IN_MODIFY: u32 = 0x0000_0002;
pub const IN_MOVED_FROM: u32 = 0x0000_0040;
pub const IN_MOVED_TO: u32 = 0x0000_0080;
pub const IN_MOVED: u32 = (IN_MOVED_FROM | IN_MOVED_TO);
pub const IN_CREATE: u32 = 0x0000_0100;
pub const IN_DELETE: u32 = 0x0000_0200;

pub fn observation_init() -> Result<i32, Error> {
    unsafe {
        let file_descriptor = inotify_init();
        if file_descriptor == -1 {
            return Err(Error::last_os_error());
        }
        Ok(file_descriptor)
    }
}

pub fn add_watch(file_descriptor: i32, directory: &str) -> Result<i32, Error> {
    unsafe {
        let watch_descriptor = inotify_add_watch(
            file_descriptor,
            CString::new(directory)?.as_ptr(),
            IN_MODIFY | IN_MOVED | IN_CREATE | IN_DELETE,
        );
        if watch_descriptor == -1 {
            return Err(Error::last_os_error());
        }
        Ok(watch_descriptor)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct InotifyEvent {
    pub watch_descriptor: i32,
    pub mask: u32,
    pub cookie: u32,
    pub length: u32,
}

extern "C" {
    fn inotify_init() -> i32;
    #[allow(improper_ctypes)]
    fn inotify_add_watch(file_descriptor: i32, pathname: *const i8, mask: u32) -> i32;
    pub fn inotify_rm_watch(file_descriptor: i32, watch_descriptor: i32) -> i32;
    pub fn close(file_descriptor: i32) -> i32;
    pub fn read(file_descriptor: i32, buffer: *mut u8, count: usize) -> i32;
}
