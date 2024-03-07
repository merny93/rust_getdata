pub mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
// basic tests to confirm that getdata is working as expected
#[cfg(test)]
mod tests;

use std::any::TypeId;
use std::ffi::CStr;
use std::ffi::CString; //use this to go from rust to c
use std::os::fd::RawFd; //use this to go from c to rust

//lets make a struct to hold the dirfile
pub struct Dirfile {
    pub dirfile: Option<std::ptr::NonNull<ffi::DIRFILE>>,
}

pub enum GdTypes {
    Float32,
    Float64,
    Int32,
    Int64,
}
pub trait FromRaw {
    unsafe fn from_raw(raw: *const std::ffi::c_void, len: usize) -> Self;
}

impl FromRaw for Vec<f32> {
    unsafe fn from_raw(raw: *const std::ffi::c_void, len: usize) -> Self {
        std::slice::from_raw_parts(raw as *const f32, len).to_vec()
    }
}
impl FromRaw for Vec<f64> {
    unsafe fn from_raw(raw: *const std::ffi::c_void, len: usize) -> Self {
        std::slice::from_raw_parts(raw as *const f64, len).to_vec()
    }
}

impl FromRaw for Vec<i32> {
    unsafe fn from_raw(raw: *const std::ffi::c_void, len: usize) -> Self {
        std::slice::from_raw_parts(raw as *const i32, len).to_vec()
    }
}

impl FromRaw for Vec<i64> {
    unsafe fn from_raw(raw: *const std::ffi::c_void, len: usize) -> Self {
        std::slice::from_raw_parts(raw as *const i64, len).to_vec()
    }
}


impl GdTypes {
    fn to_ffi(&self) -> ffi::gd_type_t {
        match self {
            GdTypes::Float32 => ffi::gd_type_t_GD_FLOAT32,
            GdTypes::Float64 => ffi::gd_type_t_GD_FLOAT64,
            GdTypes::Int32 => ffi::gd_type_t_GD_INT32,
            GdTypes::Int64 => ffi::gd_type_t_GD_INT64,
        }
    }
    fn type_id(&self) -> TypeId {
        match self {
            GdTypes::Float32 => TypeId::of::<f32>(),
            GdTypes::Float64 => TypeId::of::<f64>(),
            GdTypes::Int32 => TypeId::of::<i32>(),
            GdTypes::Int64 => TypeId::of::<i64>(),
        }
    }
}

pub enum EntryType {
    Raw { data_type: GdTypes, spf: u32 },
}
pub struct Entry {
    pub field_code: String,
    pub fragment_index: i32,
    pub entry_type: EntryType,
}

impl Dirfile {
    /// Open a dirfile in read/write mode, creating it if it does not exist
    pub fn open(dirfile_name: &str) -> Dirfile {
        let dirfile_name = CString::new(dirfile_name).unwrap();
        let dirfile =
            unsafe { ffi::gd_open(dirfile_name.as_ptr(), (ffi::GD_RDWR | ffi::GD_CREAT).into()) };
        Dirfile {
            dirfile: std::ptr::NonNull::new(dirfile),
        }
    }
    /// Close the dirfile
    pub fn close(&mut self) {
            unsafe { ffi::gd_close(self.dirfile.expect("open dirfile!").as_ptr()) };
            self.dirfile = None;
    }

    /// add entry
    pub fn add(&mut self, entry: Entry) {
        match entry.entry_type {
            //only raw data is supported for now
            EntryType::Raw { data_type, spf } => {
                //create a c string
                let field_code = CString::new(entry.field_code).unwrap();
                let ret_val = unsafe {
                    ffi::gd_add_raw(
                        self.dirfile.expect("open the dirfile!").as_ptr(),
                        field_code.as_ptr(),
                        data_type.to_ffi(),
                        spf,
                        0,
                    )
                };
                if ret_val != 0 {
                    panic!("gd_add_raw failed with error: {}", ret_val);
                }
            }
        }
    }
    /// puts data vectors, returns if the write was successful
    pub fn putdata<T: 'static>(&mut self, entry: Entry, data: &Vec<T>) -> Result<usize, &'static str> {
        match entry.entry_type {
            //only raw data is supported for now
            EntryType::Raw { data_type, spf } => {
                //check that the type is correct
                assert_eq!(data_type.type_id(), TypeId::of::<T>(), "Data type mismatch");
                //create a c string
                let field_code = CString::new(entry.field_code).unwrap();
                //write data, will need to update the offset and stuff...
                let write_n = unsafe {
                    ffi::gd_putdata(
                        self.dirfile.expect("Open the dirfile!").as_ptr(),
                        field_code.as_ptr(),
                        0,
                        0,
                        data.len() as usize,
                        0,
                        data_type.to_ffi(),
                        data.as_ptr() as *const std::ffi::c_void,
                    )
                };
                if write_n != data.len(){
                    return Err("gd_putdata failed");
                }
                Ok(write_n)
            }
        }
    }

    pub fn get_entry(&mut self, field_code: &str) -> Option<Entry> {
        let field_code = CString::new(field_code).unwrap();
        let mut c_entry: *mut ffi::gd_unified_entry_;
        let ret = unsafe {
            ffi::gd_entry(self.dirfile.unwrap().as_ptr(), field_code.as_ptr(),c_entry)
        };
        if (ret != 0) {
            return None;
        }

        //need to convert from the c entry to the rust entry
        let entry = unsafe { *c_entry };
        entry.data_type;


    }
        

    ///getdata
    pub fn getdata<T: FromRaw>(&mut self, field_code: &str, first_frame: usize, first_sample: usize, num_frames: usize, num_samples: usize) -> Result<T,&'static str>{
        Ok(Vec::new())
    }
}
