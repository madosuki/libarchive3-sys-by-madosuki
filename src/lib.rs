 use libc::{c_int, c_char, c_void, size_t, off_t};

pub enum ArchiveError {
    ArchiveEof = 1,
    ArchiveOk = 0,
    ArchiveRetry = -10,
    ArchiveWarn = -20,
    ArchiveFailed = -25,
    ArchiveFatal = -30,
}

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


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
