use libc::{c_int, c_char, c_void, size_t, off_t};

// Error codes
pub const ARCHIVE_EOF: c_int = 1;
pub const ARCHIVE_OK: c_int = 0;
pub const ARCHIVE_RETRY: c_int = -10;
pub const ARCHIVE_WARN: c_int = -20;
pub const ARCHIVE_FAILED: c_int = -25;
pub const ARCHIVE_FATAL: c_int = -30;

// Filter codes
pub const ARCHIVE_FILTER_NONE: c_int = 0;
pub const ARCHIVE_FILTER_GZIP: c_int = 1;
pub const ARCHIVE_FILTER_BZIP2: c_int = 2;
pub const ARCHIVE_FILTER_COMPRESS: c_int = 3;
pub const ARCHIVE_FILTER_PROGRAM: c_int = 4;
pub const ARCHIVE_FILTER_LZMA: c_int = 5;
pub const ARCHIVE_FILTER_XZ: c_int = 6;
pub const ARCHIVE_FILTER_UU: c_int = 7;
pub const ARCHIVE_FILTER_RPM: c_int = 8;
pub const ARCHIVE_FILTER_LZIP: c_int = 9;
pub const ARCHIVE_FILTER_LRZIP: c_int = 10;
pub const ARCHIVE_FILTER_LZOP: c_int = 11;
pub const ARCHIVE_FILTER_GRZIP: c_int = 12;
pub const ARCHIVE_FILTER_LZ4: c_int = 13;
pub const ARCHIVE_FILTER_ZSTD: c_int = 14;


pub enum ArchiveStruct {}
pub enum ArchiveEntryStruct {}

pub type ArchiveOpenCallBack = unsafe extern "C" fn (archive: ArchiveStruct,
                                                 _client_data: *mut c_void);
pub type ArchiveCloseCallBack = unsafe extern "C" fn (archive: ArchiveStruct,
                                                  _client_data: *mut c_void);


#[link(name = "archive")]
extern "C" {
    pub fn archive_version_number() -> c_int;
    pub fn archive_version_string() -> *const c_char;
    pub fn archive_errno(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_new() -> *mut ArchiveStruct;
    pub fn archive_read_support_filter_all(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_all(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_open_filename(archive: *mut ArchiveStruct, _filename: *const c_char, _block_size: size_t) -> c_int;
    pub fn archive_read_next_header(archive: *mut ArchiveStruct, entry: *mut *mut ArchiveEntryStruct) -> c_int;
    pub fn archive_entry_pathname(entry: *mut ArchiveEntryStruct) -> *const c_char;
    pub fn archive_entry_size(entry: *mut ArchiveEntryStruct) -> i64;
    pub fn archive_entry_new() -> *mut ArchiveEntryStruct;
    pub fn archive_entry_new2(archive: *mut ArchiveStruct) -> *mut ArchiveEntryStruct;
    pub fn archive_read_free(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_close(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_data_block(archive: *mut ArchiveStruct, buff: *mut *mut u8, len: *mut size_t, offset: *mut off_t) -> c_int;
}


// #[cfg(test)]
// mod tests {
//     use super::*;


//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
