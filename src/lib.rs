use libc::{c_int, c_long, c_char, c_void, size_t, off_t, ssize_t};

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

// Format codes
pub const ARCHIVE_FORMAT_BASE_MASK: c_int = 0xff000;
pub const ARCHIVE_FORMAT_CPIO: c_int = 0x10000;
pub const ARCHIVE_FORMAT_CPIO_POSIX: c_int = ARCHIVE_FORMAT_CPIO | 1;
pub const ARCHIVE_FORMAT_CPIO_BIN_LE: c_int = ARCHIVE_FORMAT_CPIO | 2;
pub const ARCHIVE_FORMAT_CPIO_BIN_BE: c_int = ARCHIVE_FORMAT_CPIO | 3;
pub const ARCHIVE_FORMAT_CPIO_SVR4_NOCRC: c_int = ARCHIVE_FORMAT_CPIO | 4;
pub const ARCHIVE_FORMAT_CPIO_SVR4_CRC: c_int = ARCHIVE_FORMAT_CPIO | 5;
pub const ARCHIVE_FORMAT_CPIO_AFIO_LARGE: c_int = ARCHIVE_FORMAT_CPIO | 6;
pub const ARCHIVE_FORMAT_CPIO_PWB: c_int = ARCHIVE_FORMAT_CPIO | 7;
pub const ARCHIVE_FORMAT_SHAR: c_int = 0x20000;
pub const ARCHIVE_FORMAT_SHAR_BASE: c_int = ARCHIVE_FORMAT_SHAR | 1;
pub const ARCHIVE_FORMAT_SHAR_DUMP: c_int = ARCHIVE_FORMAT_SHAR | 2;
pub const ARCHIVE_FORMAT_TAR: c_int = 0x30000;
pub const ARCHIVE_FORMAT_TAR_USTAR: c_int = ARCHIVE_FORMAT_TAR | 1;
pub const ARCHIVE_FORMAT_TAR_PAX_INTERCHANGE: c_int = ARCHIVE_FORMAT_TAR | 2;
pub const ARCHIVE_FORMAT_TAR_PAX_RESTRICTED: c_int = ARCHIVE_FORMAT_TAR | 3;
pub const ARCHIVE_TAR_GNUTAR: c_int = ARCHIVE_FORMAT_TAR | 4;
pub const ARCHIVE_ISO9660: c_int = 0x40000;
pub const ARCHIVE_ISO9660_ROCKRIDGE: c_int = ARCHIVE_ISO9660 | 1;
pub const ARCHIVE_FORMAT_ZIP: c_int = 0x50000;
pub const ARCHIVE_FORMAT_EMPTY: c_int = 0x60000;
pub const ARCHIVE_FORMAT_AR: c_int = 0x70000;
pub const ARCHIVE_FORMAT_AR_GNU: c_int = ARCHIVE_FORMAT_AR | 1;
pub const ARCHIVE_FORMAT_AR_BSD: c_int = ARCHIVE_FORMAT_AR | 2;
pub const ARCHIVE_FORMAT_MTREE: c_int = 0x80000;
pub const ARCHIVE_FORMAT_RAW: c_int = 0x90000;
pub const ARCHIVE_FORAMT_XAR: c_int = 0xA0000;
pub const ARCHIVE_FORMAT_LHA: c_int = 0xB0000;
pub const ARCHIVE_FORMAT_CAB: c_int = 0xC0000;
pub const ARCHIVE_FORMAT_RAR: c_int = 0xD0000;
pub const ARCHIVE_FORMAT_7ZIP: c_int = 0xE0000;
pub const ARCHIVE_FORMAT_WARC: c_int = 0xF0000;
pub const ARCHIVE_FORMAT_RAR_V5: c_int = 0x100000;

pub const ARCHIVE_READ_FORMAT_CAPS_NONE: c_int = 0;
pub const ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_DATA: c_int = 1 << 0;
pub const ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_METADATA: c_int = 1 << 1;

pub const ARCHIVE_READ_FORMAT_ENCRYPTION_UNSUPPORTED: c_int = -2;
pub const ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW: c_int = -1;

pub enum ArchiveStruct {}
pub enum ArchiveEntryStruct {}

pub type ArchiveReadCallback = unsafe extern "C" fn (archive: *mut ArchiveStruct, _client_data: *mut c_void, _buffer: *mut *mut c_void) -> ssize_t;
pub type ArchiveSkipCallback = unsafe extern "C" fn (archive: *mut ArchiveStruct, _client_data: *mut c_void, request: c_long) -> c_long;
pub type ArchiveSeekCallback = unsafe extern "C" fn (archive: *mut ArchiveStruct, _client_data: *mut c_void, offset: c_long, whence: c_int) -> c_long;
pub type ArchiveWriteCallback = unsafe extern "C" fn (archive: *mut ArchiveStruct, _client_data: *mut c_void, _buffer: *const c_void, length: size_t) -> ssize_t;
pub type ArchiveOpenCallBack = unsafe extern "C" fn (archive: *mut ArchiveStruct,
                                                 _client_data: *mut c_void) -> c_int;
pub type ArchiveCloseCallBack = unsafe extern "C" fn (archive: *mut ArchiveStruct,
                                                      _client_data: *mut c_void) -> c_int;
pub type ArchiveFreeCallback = unsafe extern "C" fn (archive: *mut ArchiveStruct, _client_data: *mut c_void) -> c_int;

pub type ArchiveSwitchCallback = unsafe extern "C" fn (archive: *mut ArchiveStruct, _client_data1: *mut c_void, _client_data2: *mut c_void) -> c_int;
pub type ArchivePassphraseCallback = unsafe extern "C" fn(archive: *mut ArchiveStruct, _client_data: *mut c_void) -> *const c_char;


#[link(name = "archive")]
extern "C" {
    pub fn archive_version_number() -> c_int;
    pub fn archive_version_string() -> *const c_char;
    pub fn archive_version_details() -> *const c_char;
    pub fn archive_zlib_version() -> *const c_char;
    pub fn archive_liblzma_version() -> *const c_char;
    pub fn archive_bzlib_version() -> *const c_char;
    pub fn archive_liblz4_version() -> *const c_char;
    pub fn archive_libzstd_version() -> *const c_char;
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
