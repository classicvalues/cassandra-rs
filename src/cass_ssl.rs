#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use libc::types::os::arch::c95::c_char;
use libc::types::os::arch::c95::c_uint;
use libc::types::os::arch::c95::c_int;

use cass_error::CassError;
use cass_string::CassString;

enum Struct_CassSsl_ { }
pub type CassSsl = Struct_CassSsl_;

type Enum_CassSslVerifyFlags = c_uint;
pub const CASS_SSL_VERIFY_NONE: c_uint = 0;
pub const CASS_SSL_VERIFY_PEER_CERT: c_uint = 1;
pub const CASS_SSL_VERIFY_PEER_IDENTITY: c_uint = 2;
pub type CassSslVerifyFlags = Enum_CassSslVerifyFlags;

extern "C" {
    pub fn cass_ssl_new() -> *mut CassSsl;
    pub fn cass_ssl_free(ssl: *mut CassSsl);
    pub fn cass_ssl_add_trusted_cert(ssl: *mut CassSsl, cert: CassString) -> CassError;
    pub fn cass_ssl_set_verify_flags(ssl: *mut CassSsl, flags: c_int);
    pub fn cass_ssl_set_cert(ssl: *mut CassSsl, cert: CassString) -> CassError;
    pub fn cass_ssl_set_private_key(ssl: *mut CassSsl, key: CassString, password: *const c_char) -> CassError;
}
