use std::error;
use std::fmt;
use crate::ffi;
use std::ffi::CString;






#[derive(Debug)]
pub enum GdError {
    Alloc(String),           // GD_E_ALLOC
    Accmode(String),         // GD_E_ACCMODE
    Argument(String),        // GD_E_ARGUMENT
    BadCode(String),         // GD_E_BAD_CODE
    BadDirfile(String),      // GD_E_BAD_DIRFILE
    BadEntry(String),        // GD_E_BAD_ENTRY
    BadFieldType(String),    // GD_E_BAD_FIELD_TYPE
    BadIndex(String),        // GD_E_BAD_INDEX
    BadReference(String),    // GD_E_BAD_REFERENCE
    BadScalar(String),       // GD_E_BAD_SCALAR
    BadType(String),         // GD_E_BAD_TYPE
    Bounds(String),          // GD_E_BOUNDS
    Callback(String),        // GD_E_CALLBACK
    Creat(String),           // GD_E_CREAT
    Delete(String),          // GD_E_DELETE
    Dimension(String),       // GD_E_DIMENSION
    Domain(String),          // GD_E_DOMAIN
    Duplicate(String),       // GD_E_DUPLICATE
    Exists(String),          // GD_E_EXISTS
    Format(String),          // GD_E_FORMAT
    InternalError(String),   // GD_E_INTERNAL_ERROR
    Io(String),              // GD_E_IO
    LineTooLong(String),     // GD_E_LINE_TOO_LONG
    Lut(String),             // GD_E_LUT
    Protected(String),       // GD_E_PROTECTED
    Range(String),           // GD_E_RANGE
    RecurseLevel(String),    // GD_E_RECURSE_LEVEL
    UncleanDb(String),       // GD_E_UNCLEAN_DB
    UnknownEncoding(String), // GD_E_UNKNOWN_ENCODING
    Unsupported(String),     // GD_E_UNSUPPORTED
}
impl GdError {
    pub fn message(&self) -> &str {
        match self {
            GdError::Alloc(msg) => msg,
            GdError::Accmode(msg) => msg,
            GdError::Argument(msg) => msg,
            GdError::BadCode(msg) => msg,
            GdError::BadDirfile(msg) => msg,
            GdError::BadEntry(msg) => msg,
            GdError::BadFieldType(msg) => msg,
            GdError::BadIndex(msg) => msg,
            GdError::BadReference(msg) => msg,
            GdError::BadScalar(msg) => msg,
            GdError::BadType(msg) => msg,
            GdError::Bounds(msg) => msg,
            GdError::Callback(msg) => msg,
            GdError::Creat(msg) => msg,
            GdError::Delete(msg) => msg,
            GdError::Dimension(msg) => msg,
            GdError::Domain(msg) => msg,
            GdError::Duplicate(msg) => msg,
            GdError::Exists(msg) => msg,
            GdError::Format(msg) => msg,
            GdError::InternalError(msg) => msg,
            GdError::Io(msg) => msg,
            GdError::LineTooLong(msg) => msg,
            GdError::Lut(msg) => msg,
            GdError::Protected(msg) => msg,
            GdError::Range(msg) => msg,
            GdError::RecurseLevel(msg) => msg,
            GdError::UncleanDb(msg) => msg,
            GdError::UnknownEncoding(msg) => msg,
            GdError::Unsupported(msg) => msg,
        }
    }
}

impl fmt::Display for GdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GetData Error: {}", self.message())
    }
}

impl error::Error for GdError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // In this case, we have no underlying error, so we return None.
        None
    }
}

impl crate::Dirfile{
    pub fn get_error(&self) -> Option<GdError> {
        let error = unsafe { ffi::gd_error(self.dirfile.unwrap().as_ptr()) };
        if error == ffi::GD_E_OK as i32 {
            return None;
        }
        let error_string_ptr = unsafe {
            ffi::gd_error_string(self.dirfile.unwrap().as_ptr(), std::ptr::null_mut(), 0)
        };
        let error_string_c = unsafe { CString::from_raw(error_string_ptr as *mut i8) }; //takes ownership of the pointer
        let error_string = error_string_c.to_str().unwrap().to_string(); //error wants to own the string
        match error {
            ffi::GD_E_ALLOC => Some(GdError::Alloc(error_string)),
            ffi::GD_E_ACCMODE => Some(GdError::Accmode(error_string)),
            ffi::GD_E_ARGUMENT => Some(GdError::Argument(error_string)),
            ffi::GD_E_BAD_CODE => Some(GdError::BadCode(error_string)),
            ffi::GD_E_BAD_DIRFILE => Some(GdError::BadDirfile(error_string)),
            ffi::GD_E_BAD_ENTRY => Some(GdError::BadEntry(error_string)),
            ffi::GD_E_BAD_FIELD_TYPE => Some(GdError::BadFieldType(error_string)),
            ffi::GD_E_BAD_INDEX => Some(GdError::BadIndex(error_string)),
            ffi::GD_E_BAD_REFERENCE => Some(GdError::BadReference(error_string)),
            ffi::GD_E_BAD_SCALAR => Some(GdError::BadScalar(error_string)),
            ffi::GD_E_BAD_TYPE => Some(GdError::BadType(error_string)),
            ffi::GD_E_BOUNDS => Some(GdError::Bounds(error_string)),
            ffi::GD_E_CALLBACK => Some(GdError::Callback(error_string)),
            ffi::GD_E_CREAT => Some(GdError::Creat(error_string)),
            ffi::GD_E_DELETE => Some(GdError::Delete(error_string)),
            ffi::GD_E_DIMENSION => Some(GdError::Dimension(error_string)),
            ffi::GD_E_DOMAIN => Some(GdError::Domain(error_string)),
            ffi::GD_E_DUPLICATE => Some(GdError::Duplicate(error_string)),
            ffi::GD_E_EXISTS => Some(GdError::Exists(error_string)),
            ffi::GD_E_FORMAT => Some(GdError::Format(error_string)),
            ffi::GD_E_INTERNAL_ERROR => Some(GdError::InternalError(error_string)),
            ffi::GD_E_IO => Some(GdError::Io(error_string)),
            ffi::GD_E_LINE_TOO_LONG => Some(GdError::LineTooLong(error_string)),
            ffi::GD_E_LUT => Some(GdError::Lut(error_string)),
            ffi::GD_E_PROTECTED => Some(GdError::Protected(error_string)),
            ffi::GD_E_RANGE => Some(GdError::Range(error_string)),
            ffi::GD_E_RECURSE_LEVEL => Some(GdError::RecurseLevel(error_string)),
            ffi::GD_E_UNCLEAN_DB => Some(GdError::UncleanDb(error_string)),
            ffi::GD_E_UNKNOWN_ENCODING => Some(GdError::UnknownEncoding(error_string)),
            ffi::GD_E_UNSUPPORTED => Some(GdError::Unsupported(error_string)),
            _ => {
                panic!("Unsupported error");
            }
        }
    }
}