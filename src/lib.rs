use libc::{c_int, c_long, c_uint, c_char, wchar_t, c_void, size_t, ssize_t, time_t, FILE, stat};

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

pub const ARCHIVE_COMPRESSION_NONE: c_int = ARCHIVE_FILTER_NONE;
pub const ARCHIVE_COMPRESSION_GZIP: c_int = ARCHIVE_FILTER_GZIP;
pub const ARCHIVE_COMPRESSION_BZIP2: c_int = ARCHIVE_FILTER_BZIP2;
pub const ARCHIVE_COMPRESSION_COMPRESS: c_int = ARCHIVE_FILTER_COMPRESS;
pub const ARCHIVE_COMPRESSION_PRGORAM: c_int = ARCHIVE_FILTER_PROGRAM;
pub const ARCHIVE_COMPRESSION_LZMA: c_int = ARCHIVE_FILTER_LZMA;
pub const ARCHIVE_COMPRESSION_XZ: c_int = ARCHIVE_FILTER_XZ;
pub const ARCHIVE_COMPRESSION_UU: c_int = ARCHIVE_FILTER_UU;
pub const ARCHIVE_COMPRESSION_RPM: c_int = ARCHIVE_FILTER_RPM;
pub const ARCHIVE_COMPRESSION_LZIP: c_int = ARCHIVE_FILTER_LZIP;
pub const ARCHIVE_COMPRESSION_LRZIP: c_int = ARCHIVE_FILTER_LRZIP;

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

// extract codes
pub const ARCHIVE_EXTRACT_OWNER: c_int = 0x0001;
pub const ARCHIVE_EXTRACT_PERM: c_int = 0x0002;
pub const ARCHIVE_EXTRACT_TIME: c_int = 0x0004;
pub const ARCHIVE_EXTRACT_NO_OVERWRITE: c_int = 0x0008;
pub const ARCHIVE_EXTRACT_UNLINK: c_int = 0x0010;
pub const ARCHIVE_EXTRACT_ACL: c_int = 0x0020;
pub const ARCHIVE_EXTRACT_FFLAGS: c_int = 0x0040;
pub const ARCHIVE_EXTRACT_XATTR: c_int = 0x0080;
pub const ARCHIVE_EXTRACT_SECURE_SYMLINKS: c_int = 0x0100;
pub const ARCHIVE_EXTRACT_SECURE_NODOTDOT: c_int = 0x0200;
pub const ARCHIVE_EXTRACT_NO_AUTODIR: c_int = 0x0400;
pub const ARCHIVE_EXTRACT_NO_OVERWRITE_NEWER: c_int = 0x0800;
pub const ARCHIVE_EXTRACT_SPARSE: c_int = 0x1000;
pub const ARCHIVE_EXTRACT_MAC_METADATA: c_int = 0x2000;
pub const ARCHIVE_EXTRACT_NO_HFS_COMPRESSION: c_int = 0x4000;
pub const ARCHIVE_EXTRACT_HFS_COMPRESSION_FORCED: c_int = 0x8000;
pub const ARCHIVE_EXTRACT_SECURE_NOABSOLUTEPATHS: c_int = 0x10000;
pub const ARCHIVE_EXTRACT_CLEAR_NOCHANGE_FFLAGS: c_int = 0x20000;
pub const ARCHIVE_EXTRACT_SAFE_WRITES: c_int = 0x40000;

// Archive readdisk const values
pub const ARCHIVE_READDISK_RESTORE_ATIME: c_int = 0x0001;
pub const ARCHIVE_READDISK_HONOR_NODUMP: c_int = 0x0002;
pub const ARCHIVE_READDISK_MAY_COPYFILE: c_int = 0x0004;
pub const ARCHIVE_READDISK_NO_TRAVERSE_MOUNTS: c_int = 0x0008;
pub const ARCHIVE_READDISK_NO_XATTR: c_int = 0x0010;
pub const ARCHIVE_READDISK_NO_ACL: c_int = 0x0020;
pub const ARCHIVE_READDISK_NO_FFLAGS: c_int = 0x0040;
pub const ARCHIVE_READDISK_NO_SPARSE: c_int = 0x0080;

// Archive match const values
pub const ARCHIVE_MATCH_MTIME: c_int = 0x0100;
pub const ARCHIVE_MATCH_CTIME: c_int = 0x0200;
pub const ARCHIVE_MATCH_NEWER: c_int = 0x0001;
pub const ARCHIVE_MATCH_OLDER: c_int = 0x0002;
pub const ARCHIVE_MATCH_EQUAL: c_int = 0x0010;

// entry symlink types
pub const AE_SYMLINK_TYPE_UNDEFINED: c_int = 0;
pub const AE_SYMLINK_TYPE_FILE: c_int = 1;
pub const AE_SYMLINK_TYPE_DIRECTORY: c_int = 2;


pub enum ArchiveStruct {}
pub enum ArchiveEntryStruct {}

pub type ArchiveReadCallback = unsafe extern "C" fn (archive: *mut ArchiveStruct, _client_data: *mut c_void, _buffer: *mut *mut c_void) -> ssize_t;
pub type ArchiveSkipCallback = unsafe extern "C" fn (archive: *mut ArchiveStruct, _client_data: *mut c_void, request: i64) -> i64;
pub type ArchiveSeekCallback = unsafe extern "C" fn (archive: *mut ArchiveStruct, _client_data: *mut c_void, offset: i64, whence: c_int) -> i64;
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
    pub fn archive_read_new() -> *mut ArchiveStruct;

    // read
    pub fn archive_read_support_filter_all(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_filter_by_code(archive: *mut ArchiveStruct, code: c_int) -> c_int;
    pub fn archive_read_support_filter_bzip2(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_filter_compress(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_filter_gzip(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_filter_grzip(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_filter_lrzip(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_filter_lz4(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_filter_lzma(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_filter_lzop(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_filter_none(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_filter_program(archiuve: *mut ArchiveStruct, command: *const c_char) -> c_int;
    pub fn archive_read_support_filter_program_signature(archive: *mut ArchiveStruct, cmd: *const c_char, signature: *const c_void, signature_length: size_t) -> c_int;
    pub fn archive_read_support_filter_rpm(archive: *mut ArchiveStruct);
    pub fn archive_read_support_filter_uu(archive: *mut ArchiveStruct);
    pub fn archive_read_support_filter_xz(archive: *mut ArchiveStruct);
    pub fn archive_read_support_filter_zstd(archive: *mut ArchiveStruct);

    pub fn archive_read_support_format_all(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_7zip(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_ar(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_by_code(archive: *mut ArchiveStruct, code: c_int) -> c_int;
    pub fn archive_read_support_format_cab(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_cpio(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_empty(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_gnutar(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_iso9660(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_lha(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_mtree(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_rar(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_rar5(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_raw(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_tar(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_warc(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_xar(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_zip(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_zip_streamable(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_support_format_zip_seekable(archive: *mut ArchiveStruct) -> c_int;

    pub fn archive_read_set_format(archive: *mut ArchiveStruct, format: c_int) -> c_int;
    pub fn archive_read_append_filter(archive: *mut ArchiveStruct, filter: c_int) -> c_int;
    pub fn archive_read_append_filter_program(archive: *mut ArchiveStruct, command: *const c_char) -> c_int;
    pub fn archive_read_append_filter_program_signature(archive: *mut ArchiveStruct, cmd: *const c_char, signature: *const c_void, signature_length: size_t) -> c_int;

    pub fn archive_read_set_open_callback(archive: *mut ArchiveStruct, callback: *mut std::option::Option<ArchiveOpenCallBack>) -> c_int;
    pub fn archive_read_set_read_callback(archive: *mut ArchiveStruct, callback: *mut std::option::Option<ArchiveReadCallback>) -> c_int;
    pub fn archive_read_set_seek_callback(archive: *mut ArchiveStruct, callback: *mut std::option::Option<ArchiveSeekCallback>) -> c_int;
    pub fn archive_read_set_skip_callback(archive: *mut ArchiveStruct, callback: *mut std::option::Option<ArchiveSkipCallback>) -> c_int;
    pub fn archive_read_set_close_callback(archive: *mut ArchiveStruct, callback: *mut std::option::Option<ArchiveCloseCallBack>) -> c_int;
    pub fn archive_read_set_switch_callback(archive: *mut ArchiveStruct, callback: *mut std::option::Option<ArchiveSwitchCallback>) -> c_int;
    pub fn archive_read_set_callback_data(archive: *mut ArchiveStruct, data: *mut c_void) -> c_int;
    pub fn archive_read_set_callback_data2(archive: *mut ArchiveStruct, data: *mut c_void, index: c_uint) -> c_int;
    pub fn archive_read_add_callback_data(archive: *mut ArchiveStruct, data: *mut c_void, index: c_uint) -> c_int;
    pub fn archive_read_append_callback_data(archive: *mut ArchiveStruct, data: *mut c_void) -> c_int;
    pub fn archive_read_prepend_callback_data(archive: *mut ArchiveStruct, data: *mut c_void) -> c_int;

    pub fn archive_read_open1(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_open(archive: *mut ArchiveStruct,
                             _client_data: *mut c_void,
                             archive_open_callback: *mut std::option::Option<ArchiveOpenCallBack>,
                             archive_read_callback: *mut std::option::Option<ArchiveReadCallback>,
                             archive_close_callback: *mut std::option::Option<ArchiveCloseCallBack>) -> c_int;
    pub fn archive_read_open2(archive: *mut ArchiveStruct,
                              _client_data: *mut c_void,
                              archive_open_callback: *mut std::option::Option<ArchiveOpenCallBack>,
                              archive_read_callback: *mut std::option::Option<ArchiveReadCallback>,
                              archive_skip_callback: *mut std::option::Option<ArchiveSkipCallback>,
                              archive_close_callback: *mut std::option::Option<ArchiveCloseCallBack>) -> c_int;
    
    pub fn archive_read_open_filename(archive: *mut ArchiveStruct, _filename: *const c_char, _block_size: size_t) -> c_int;
    pub fn archive_read_open_filenames(archive: *mut ArchiveStruct, _filenames: *const *const c_char, _block_size: size_t) -> c_int;
    pub fn archive_read_open_filename_w(archive: *mut ArchiveStruct, _filename: *const wchar_t, _block_size: size_t) -> c_int;
    pub fn archive_read_open_memory(archive: *mut ArchiveStruct, buff: *const c_void, size: size_t) -> c_int;
    pub fn archive_read_open_memory2(archive: *mut ArchiveStruct, buff: *const c_void, size: size_t, read_size: size_t) -> c_int;
    pub fn archive_read_open_fd(archive: *mut ArchiveStruct, _fd: c_int, _block_size: size_t) -> c_int;
    pub fn archive_read_open_FILE(archive: *mut ArchiveStruct, _file: *mut FILE) -> c_int;
    
    pub fn archive_read_next_header(archive: *mut ArchiveStruct, entry: *mut *mut ArchiveEntryStruct) -> c_int;
    pub fn archive_read_ntext_header2(archive: *mut ArchiveStruct, entry: *mut ArchiveEntryStruct) -> c_int;
    pub fn archive_read_header_position(archive: *mut ArchiveStruct) -> i64;

    pub fn archive_read_has_encrypted_entries(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_format_capabilities(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_data(archive: *mut ArchiveStruct, buf: *mut c_void, count: size_t) -> ssize_t;
    pub fn archive_seek_data(archive: *mut ArchiveStruct, offset: i64, whence: c_int) -> i64;
    pub fn archive_read_data_block(archive: *mut ArchiveStruct, buff: *mut *const c_void, size: *mut size_t, offset: *mut i64) -> c_int;
    pub fn archive_read_data_skip(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_into_fd(archive: *mut ArchiveStruct, fd: c_int) -> c_int;

    pub fn archive_read_set_format_option(archive: *mut ArchiveStruct, m: *const c_char, o: *const c_char, v: *const c_char) -> c_int;
    pub fn archive_read_set_filter_option(archive: *mut ArchiveStruct, m: *const c_char, o: *const c_char, v: *const c_char) -> c_int;
    pub fn archive_read_set_option(archive: *mut ArchiveStruct, m: *const c_char, o: *const c_char, v: *const c_char) -> c_int;
    pub fn archive_read_set_options(archive: *mut ArchiveStruct, opts: *const c_char) -> c_int;

    pub fn archive_read_passphrase(archive: *mut ArchiveStruct, passphrase: *const c_char) -> c_int;
    pub fn archive_read_set_passphrase_callback(archive: *mut ArchiveStruct,
                                                _client_data: *mut c_void,
                                                callback: *mut std::option::Option<ArchivePassphraseCallback>) -> c_int;

    pub fn archive_read_extract(archive: *mut ArchiveStruct, entry: *mut ArchiveEntryStruct, flags: c_int) -> c_int;
    pub fn archive_read_extract2(archive: *mut ArchiveStruct, entry: *mut ArchiveEntryStruct, dst: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_extract_set_progress_callback(archive: *mut ArchiveStruct, callback: std::option::Option<unsafe extern "C" fn (*mut c_void)>, _user_data: *mut c_void) -> c_int;
    pub fn archive_read_extract_set_skip_file(archive: *mut ArchiveStruct, dev: i64, ino: i64) -> c_int;

    pub fn archive_read_close(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_free(archive: *mut ArchiveStruct) -> c_int;

    // write
    pub fn archive_write_new() -> *mut ArchiveStruct;
    pub fn archive_write_set_bytes_per_block(archive: *mut ArchiveStruct, bytes_per_block: c_int) -> c_int;
    pub fn archive_write_get_bytes_per_block(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_bytes_in_last_block(archive: *mut ArchiveStruct, bytes_in_last_block: c_int) -> c_int;
    pub fn archive_write_get_bytes_in_last_block(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_wirte_set_skip_file(archive: *mut ArchiveStruct, dev: i64, ino: i64) -> c_int;

    pub fn archive_write_add_filter(archive: *mut ArchiveStruct, filter_code: c_int) -> c_int;
    pub fn archive_write_add_filter_by_name(archive: *mut ArchiveStruct, name: *const c_char) -> c_int;
    pub fn archive_write_add_filter_b64encode(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_add_filter_bzip2(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_add_filter_compress(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_add_filter_grzip(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_add_filter_gzip(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_add_filter_lrzip(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_add_filter_lz4(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_add_filter_lzip(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_add_filter_lzma(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_add_filter_lzop(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_add_filter_none(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_add_filter_program(archive: *mut ArchiveStruct, cmd: *const c_char) -> c_int;
    pub fn archive_write_add_filter_uuencode(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_add_filter_xz(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_add_filter_zstd(archive: *mut ArchiveStruct) -> c_int;

    pub fn archive_write_set_format(archive: *mut ArchiveStruct, format_code: c_int) -> c_int;
    pub fn archive_write_set_format_by_name(archive: *mut ArchiveStruct, name: *const c_char) -> c_int;
    pub fn archive_write_set_format_7zip(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_ar_bsd(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_cpio(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_cpio_bin(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_cpio_newc(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_cpio_odc(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_cpio_pwb(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_gnutar(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_iso9660(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_mtree(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_mtree_classic(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_pax(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_pax_restricted(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_raw(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_shar(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_shar_dump(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_ustar(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_v7tar(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_warc(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_xar(archive: *mut ArchiveStruct) -> c_int;
    pub fn arcvhie_write_set_format_zip(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_set_format_filter_by_ext(archive: *mut ArchiveStruct, filename: *const c_char) -> c_int;
    pub fn archive_write_set_format_filter_by_ext_def(archive: *mut ArchiveStruct, filename: *const c_char, def_ext: *const c_char) -> c_int;
    pub fn archive_write_zip_set_compression_deflate(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_zip_set_compression_store(archive: *mut ArchiveStruct) -> c_int;

    pub fn archive_write_open2(archive: *mut ArchiveStruct, _client_data: *mut c_void,
                               archive_open_callback: *mut std::option::Option<ArchiveOpenCallBack>,
                               archive_write_callback: *mut std::option::Option<ArchiveWriteCallback>,
                               archive_close_callback: *mut std::option::Option<ArchiveCloseCallBack>,
                               archive_free_callback: *mut std::option::Option<ArchiveFreeCallback>) -> c_int;
    pub fn archive_write_open_fd(archive: *mut ArchiveStruct, _fd: c_int) -> c_int;
    pub fn archive_write_open_filename(archive: *mut ArchiveStruct, _file: *const c_char) -> c_int;
    pub fn archive_write_open_filename_w(archive: *mut ArchiveStruct, _file: *const wchar_t) -> c_int;
    pub fn archive_write_open_FILE(archive: *mut ArchiveStruct, _file: *mut FILE) -> c_int;
    pub fn archive_write_open_memory(archive: *mut ArchiveStruct, _buffer: *mut c_void, _buff_size: size_t, _used: *mut size_t) -> c_int;

    pub fn archive_write_header(archive: *mut ArchiveStruct, entry: *mut ArchiveEntryStruct) -> c_int;
    pub fn archive_write_data(archive: *mut ArchiveStruct, data: *mut c_void, size: size_t) -> ssize_t;
    pub fn archvie_write_data_blocK(archive: *mut ArchiveStruct, data: *mut c_void, size: size_t, offset: i64) -> c_int;

    pub fn archive_write_finish_entry(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_close(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_fall(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_free(archive: *mut ArchiveStruct) -> c_int;

    pub fn archive_write_set_format_option(_a: *mut ArchiveStruct, m: *const c_char, o: *const c_char, v: *const c_char) -> c_int;
    pub fn archive_write_set_filter_option(_a: *mut ArchiveStruct, m: *const c_char, o: *const c_char, v: *const c_char) -> c_int;
    pub fn archive_write_set_option(_a: *mut ArchiveStruct, m: *const c_char, o: *const c_char, v: *const c_char) -> c_int;
    pub fn archive_write_set_options(_a: *mut ArchiveStruct, opts: *const c_char) -> c_int;

    pub fn archive_write_set_passphrase(_a: *mut ArchiveStruct, p: *const c_char) -> c_int;
    pub fn archive_write_set_passphrase_callback(archive: *mut ArchiveStruct, client_data: *mut c_void, callback: *mut std::option::Option<ArchivePassphraseCallback>) -> c_int;

    // write disk
    pub fn archive_write_disk_new() -> *mut ArchiveStruct;
    pub fn archive_write_set_skip_file(archive: *mut ArchiveStruct, dev: i64, ino: i64) -> c_int;
    pub fn archive_write_disk_set_options(archive: *mut ArchiveStruct, flags: c_int) -> c_int;
    pub fn archive_write_disk_set_standard_lookup(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_write_disk_set_group_lookup(archive: *mut ArchiveStruct,
                                               private_data: *mut c_void,
                                               f: std::option::Option<unsafe extern "C" fn (*mut c_void, *const c_char, i64) -> i64>,
                                               cleanup: std::option::Option<unsafe extern "C" fn (*mut c_void)>) -> c_int;
    pub fn archive_write_disk_set_user_lookup(archive: *mut ArchiveStruct,
                                              private_data: *mut c_void,
                                              f: std::option::Option<unsafe extern "C" fn (*mut c_void, *const c_char, i64) -> i64>,
                                              cleanup: std::option::Option<unsafe extern "C" fn (*mut c_void)>) -> c_int;
    pub fn archive_write_disk_gid(archive: *mut ArchiveStruct, arg2: *const c_char, arg3: i64) -> i64;
    pub fn archive_write_disk_uid(archive: *mut ArchiveStruct, arg2: *const c_char, arg3: i64) -> i64;

    // read disk
    pub fn archive_read_disk_new() -> *mut ArchiveStruct;
    pub fn archive_read_disk_set_symlink_logical(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_disk_set_symlink_physical(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_disk_set_symlink_hybrid(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_disk_entry_from_file(archive: *mut ArchiveStruct, entry: *mut ArchiveEntryStruct, fd: c_int, stat: *mut stat) -> c_int;
    pub fn archive_read_disk_gname(archive: *mut ArchiveStruct, arg2: i64) -> *const c_char;
    pub fn archive_read_disk_uname(archive: *mut ArchiveStruct, arg2: i64) -> *const c_char;
    pub fn archive_read_disk_set_standard_lookup(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_disk_get_gname_lookup(archive: *mut ArchiveStruct,
                                              private_data: *mut c_void,
                                              lookup_fn: *mut std::option::Option<unsafe extern "C" fn(*mut c_void, i64)>,
                                              cleanup_fn: *mut std::option::Option<unsafe extern "C" fn(*mut c_void)>) -> c_int;
    pub fn archive_read_disk_open(archive: *mut ArchiveStruct, arg2: *const c_char) -> c_int;
    pub fn archive_read_disk_open_w(archive: *mut ArchiveStruct, arg2: *const wchar_t) -> c_int;
    pub fn archive_read_disk_descend(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_disk_can_descend(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_disk_current_filesystem(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_disk_current_filesystem_is_synthetic(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_disk_current_filesystem_is_remote(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_disk_set_atime_restored(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_read_disk_set_behavior(archive: *mut ArchiveStruct, flags: c_int) -> c_int;
    pub fn archive_read_disk_set_matching(archive: *mut ArchiveStruct,
                                          _matching: *mut ArchiveStruct,
                                          arg3: *mut std::option::Option<unsafe extern "C" fn(*mut ArchiveStruct, *mut c_void, *mut ArchiveEntryStruct)>,
                                          _client_data: *mut c_void) -> c_int;
    pub fn archive_read_disk_set_metadata_filter_callback(archive: *mut ArchiveStruct,
                                                          arg2: *mut std::option::Option<unsafe extern "C" fn(*mut ArchiveStruct, *mut c_void, *mut ArchiveEntryStruct) -> c_int>,
                                                          _client_data: *mut c_void) -> c_int;
    
    pub fn archive_free(archive: *mut ArchiveStruct) -> c_int;

    pub fn archive_filter_count(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_filter_bytes(archive: *mut ArchiveStruct, arg2: c_int) -> i64;
    pub fn archive_filter_code(archive: *mut ArchiveStruct, arg2: c_int) -> c_int;
    pub fn archive_filter_name(archive: *mut ArchiveStruct, arg2: c_int) -> *const c_char;

    pub fn archive_errno(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_error_string(archive: *mut ArchiveStruct) -> *const c_char;
    pub fn archive_format_name(archive: *mut ArchiveStruct) -> *const c_char;
    pub fn archive_format(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_clear_error(archive: *mut ArchiveStruct);
    pub fn archive_set_error(archive: *mut ArchiveStruct,
                             _err: c_int,
                             fmt: *const c_char,
                             ...);
    pub fn archive_copy_error(dest: *mut ArchiveStruct, src: *mut ArchiveStruct);
    pub fn archive_file_count(archive: *mut ArchiveStruct) -> c_int;

    // match api
    pub fn archive_match_new() -> *mut ArchiveStruct;
    pub fn archive_match_free(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_match_excluded(archive: *mut ArchiveStruct, entry: *mut ArchiveEntryStruct) -> c_int;
    pub fn archive_match_path_excluded(archive: *mut ArchiveStruct, entry: *mut ArchiveEntryStruct) -> c_int;
    pub fn archive_match_set_inclusion_recursion(archive: *mut ArchiveStruct, arg2: c_int) -> c_int;
    pub fn archive_match_exclude_pattern(archive: *mut ArchiveStruct, arg2: *const c_char) -> c_int;
    pub fn archive_match_exclude_pattern_w(archive: *mut ArchiveStruct, arg2: *const wchar_t) -> c_int;
    pub fn archive_match_exclude_pattern_from_file(archive: *mut ArchiveStruct, arg2: *const c_char, _null_separator: c_int) -> c_int;
    pub fn archive_match_exclude_pattern_from_file_w(archive: *mut ArchiveStruct, arg2: *const wchar_t, _null_separator: c_int) -> c_int;
    pub fn archive_match_include_pattern(archive: *mut ArchiveStruct, arg2: *const c_char) -> c_int;
    pub fn archive_match_include_pattern_w(archive: *mut ArchiveStruct, arg2: *const wchar_t) -> c_int;
    pub fn archive_match_include_pattern_from_file(archive: *mut ArchiveStruct, arg2: *const c_char, _null_separator: c_int) -> c_int;
    pub fn archive_match_include_pattern_from_file_w(archive: *mut ArchiveStruct, arg2: *const wchar_t, _null_separator: c_int) -> c_int;
    pub fn archive_match_path_unmatched_inclusions(archive: *mut ArchiveStruct) -> c_int;
    pub fn archive_match_path_unmatched_inclusions_next(archive: *mut ArchiveStruct, arg2: *const *const c_char) -> c_int;
    pub fn archive_match_path_unmatched_inclusions_next_w(archive: *mut ArchiveStruct, arg2: *const *const wchar_t) -> c_int;
    pub fn archive_match_time_excluded(archive: *mut ArchiveStruct, entry: *mut ArchiveEntryStruct) -> c_int;
    pub fn archive_match_include_time(archive: *mut ArchiveStruct, _flag: c_int, _sec: time_t, _nsec: c_long) ->c_int;
    pub fn archive_match_include_date(archive: *mut ArchiveStruct, _flag: c_int, _datestr: *const c_char) -> c_int;
    pub fn archive_match_include_date_w(archive: *mut ArchiveStruct, _flag: c_int, _datestr: *const wchar_t) -> c_int;
    pub fn archive_match_include_file_time(archive: *mut ArchiveStruct, _flag: c_int, _pathname: *const c_char) -> c_int;
    pub fn archive_match_include_file_time_w(archive: *mut ArchiveStruct, _flag: c_int, _pathname: *const wchar_t) -> c_int;
    pub fn archive_match_exclude_entry(archive: *mut ArchiveStruct, _flag: c_int, entry: *mut ArchiveEntryStruct) -> c_int;
    pub fn archive_match_owner_excluded(archive: *mut ArchiveStruct, entry: *mut ArchiveEntryStruct) -> c_int;
    pub fn archive_match_include_uid(archive: *mut ArchiveStruct, arg2: i64) -> c_int;
    pub fn archive_match_include_gid(archive: *mut ArchiveStruct, arg2: i64) -> c_int;
    pub fn archive_match_include_uname(archive: *mut ArchiveStruct, arg2: *const c_char) -> c_int;
    pub fn archive_match_include_uname_w(archive: *mut ArchiveStruct, arg2: *const wchar_t) -> c_int;
    pub fn archive_match_include_gname(archive: *mut ArchiveStruct, arg2: *const c_char) -> c_int;
    pub fn archive_match_include_gname_w(archive: *mut ArchiveStruct, arg2: *const wchar_t) -> c_int;

    pub fn archive_utility_string_sort(arg: *mut *mut c_char) -> c_int;
    

    // entry
    pub fn archive_entry_clear(entry: *mut ArchiveEntryStruct) -> *mut ArchiveEntryStruct;
    pub fn archive_entry_clone(entry: *mut ArchiveEntryStruct) -> *mut ArchiveEntryStruct;
    pub fn archive_entry_free(entry: *mut ArchiveEntryStruct);
    pub fn archive_entry_pathname(entry: *mut ArchiveEntryStruct) -> *const c_char;
    pub fn archive_entry_size(entry: *mut ArchiveEntryStruct) -> i64;
    pub fn archive_entry_new() -> *mut ArchiveEntryStruct;
    pub fn archive_entry_new2(archive: *mut ArchiveStruct) -> *mut ArchiveEntryStruct;
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
