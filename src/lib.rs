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
use std::error;
use std::ffi::CStr;
use std::ffi::CString;


//lets make a struct to hold the dirfile
pub struct Dirfile {
    pub dirfile: Option<std::ptr::NonNull<ffi::DIRFILE>>,
}
#[derive(Clone, Copy)]
pub enum GdTypes {
    Float32,
    Float64,
    Int32,
    Int64,
}

impl From<GdTypes> for ffi::gd_type_t {
    fn from(gd_type: GdTypes) -> Self {
        match gd_type {
            GdTypes::Float32 => ffi::gd_type_t_GD_FLOAT32,
            GdTypes::Float64 => ffi::gd_type_t_GD_FLOAT64,
            GdTypes::Int32 => ffi::gd_type_t_GD_INT32,
            GdTypes::Int64 => ffi::gd_type_t_GD_INT64,
        }
    }
}

impl From<ffi::gd_type_t> for GdTypes {
    fn from(gd_type: ffi::gd_type_t) -> Self {
        match gd_type {
            ffi::gd_type_t_GD_FLOAT32 => GdTypes::Float32,
            ffi::gd_type_t_GD_FLOAT64 => GdTypes::Float64,
            ffi::gd_type_t_GD_INT32 => GdTypes::Int32,
            ffi::gd_type_t_GD_INT64 => GdTypes::Int64,
            _ => {
                panic!("Unsupported type");
            }
        }
    }
}

impl From<GdTypes> for TypeId {
    fn from(gd_type: GdTypes) -> Self {
        match gd_type {
            GdTypes::Float32 => TypeId::of::<f32>(),
            GdTypes::Float64 => TypeId::of::<f64>(),
            GdTypes::Int32 => TypeId::of::<i32>(),
            GdTypes::Int64 => TypeId::of::<i64>(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum EntryType {
    Raw {
        data_type: Option<GdTypes>,
        spf: Option<u32>,
    },
    // Add other variants here
}

impl From<EntryType> for ffi::gd_entype_t {
    fn from(entry_type: EntryType) -> Self {
        match entry_type {
            EntryType::Raw { .. } => ffi::gd_entype_t_GD_RAW_ENTRY,
            // Add other variants here
        }
    }
}

impl From<ffi::gd_entype_t> for EntryType {
    fn from(entry_type: ffi::gd_entype_t) -> Self {
        match entry_type {
            ffi::gd_entype_t_GD_RAW_ENTRY => EntryType::Raw {
                data_type: None,
                spf: None,
            },
            _ => {
                panic!("Unsupported entry type");
            }
        }
    }
}
impl EntryType {
    pub fn new_raw(data_type: GdTypes, spf: u32) -> Self {
        EntryType::Raw {
            data_type: Some(data_type),
            spf: Some(spf),
        }
    }
}

pub struct Entry {
    pub field_code: String,
    pub fragment_index: i32,
    pub entry_type: EntryType,
    entry_c: ffi::gd_entry_t,
    _field_code_c: CString,
}

impl Entry {
    pub fn new(field_code: &str, fragment_index: i32, entry_type: EntryType) -> Entry {
        let field_code_c = CString::new(field_code).unwrap();
        let mut entry_c: ffi::gd_entry_t;
        unsafe {
            entry_c = std::mem::zeroed();
        }
        //assign the field name
        entry_c.field = field_code_c.as_ptr() as *mut i8;
        match entry_type {
            EntryType::Raw { data_type, spf } => {
                entry_c.field_type = entry_type.into();
                entry_c.__bindgen_anon_1.__bindgen_anon_1.data_type = data_type.unwrap().into();
                entry_c.__bindgen_anon_1.__bindgen_anon_1.spf = spf.unwrap();
            }
        }
        Entry {
            field_code: field_code.to_string(),
            fragment_index: fragment_index,
            entry_type: entry_type,
            entry_c: entry_c,
            _field_code_c: field_code_c,
        }
    }
    pub fn from_c(field_code: &str, fragment_index: i32, entry_c: ffi::gd_entry_t) -> Entry {
        let field_code_c: CString = CString::new(field_code).unwrap();

        let entry_type: EntryType = unsafe {
            match EntryType::from(entry_c.field_type) {
                EntryType::Raw { .. } => EntryType::Raw {
                    data_type: Some(GdTypes::from(
                        entry_c.__bindgen_anon_1.__bindgen_anon_1.data_type,
                    )),
                    spf: Some(entry_c.__bindgen_anon_1.__bindgen_anon_1.spf),
                },
            }
        };
        Entry {
            field_code: field_code.to_string(),
            fragment_index: fragment_index,
            entry_type: entry_type,
            entry_c: entry_c,
            _field_code_c: field_code_c,
        }
    }
}

pub enum FieldOrEntry {
    Field(String),
    Entry(Entry),
}

impl Dirfile {
    /// Open a dirfile in read/write mode, creating it if it does not exist
    pub fn open(dirfile_name: &str) -> Result<Dirfile, String> {
        let dirfile_name = CString::new(dirfile_name).unwrap();
        let dirfile =
            unsafe { ffi::gd_open(dirfile_name.as_ptr(), (ffi::GD_RDWR | ffi::GD_CREAT).into()) };
        let df = Dirfile {
            dirfile: std::ptr::NonNull::new(dirfile),
        };
        match df.get_error() {
            None => Ok(Dirfile {
                dirfile: std::ptr::NonNull::new(dirfile),
            }),
            Some(error) => { 
                unsafe { ffi::gd_close(dirfile) };
                Err(error)
            },
        }
        
    }
    /// Close the dirfile
    pub fn close(&mut self) {
        unsafe { ffi::gd_close(self.dirfile.expect("open dirfile!").as_ptr()) };
        self.dirfile = None;
    }

    /// add entry
    pub fn add(&mut self, entry: &Entry) -> Result<(), String> {
        let ret_val = unsafe {
            ffi::gd_add(
                self.dirfile.expect("Open the dirfile!").as_ptr(),
                &entry.entry_c,
            )
        };
        if ret_val == 0 {
            Ok(())
        } else {
            Err(self.get_error().unwrap())
        }
    }

    pub fn get_entry(&self, field_code: &str) -> Result<Entry, String> {
        let field_code = CString::new(field_code).unwrap();
        let mut entry_c: ffi::gd_entry_t;
        unsafe {
            entry_c = std::mem::zeroed();
        }
        let ret_val = unsafe {
            ffi::gd_entry(
                self.dirfile.expect("Open the dirfile!").as_ptr(),
                field_code.as_ptr(),
                &mut entry_c,
            )
        };
        if ret_val != 0 {
            return Err(self.get_error().unwrap());
        }

        let entry = Entry::from_c(field_code.to_str().unwrap(), 0, entry_c);
        Ok(entry)
    }

    /// puts data vectors, returns if the write was successful
    pub fn putdata<T: 'static>(
        &mut self,
        field_or_entry: FieldOrEntry,
        data: &Vec<T>,
    ) -> Result<usize, String> {
        let entry = match field_or_entry {
            FieldOrEntry::Field(field_code) => self.get_entry(&field_code).unwrap(),
            FieldOrEntry::Entry(entry) => entry,
        };
        match entry.entry_type {
            //only raw data is supported for now
            EntryType::Raw { data_type, spf } => {
                //check that the type is correct
                assert_eq!(
                    TypeId::from(data_type.unwrap()),
                    TypeId::of::<T>(),
                    "Data type mismatch"
                );
                //figure out how much data to write
                let num_frames: usize = data.len()/spf.unwrap() as usize;
                let num_samples: usize = data.len()%spf.unwrap() as usize;
                //create a c string
                let field_code = CString::new(entry.field_code).unwrap();
                //write data, will need to update the offset and stuff...
                let write_n = unsafe {
                    ffi::gd_putdata(
                        self.dirfile.expect("Open the dirfile!").as_ptr(),
                        field_code.as_ptr(),
                        ffi::GD_HERE.into(),
                        0,
                        num_frames,
                        num_samples,
                        data_type.unwrap().into(),
                        data.as_ptr() as *const std::ffi::c_void,
                    )
                };
                if write_n != data.len() {
                    match self.get_error() {
                        Some(error) => return Err(error),
                        None => {},
                    };
                }
                Ok(write_n)
            }
        }
    }

    pub fn flush(&mut self) -> Result<(), String> {
        let ret_val = unsafe { ffi::gd_flush(self.dirfile.unwrap().as_ptr(), std::ptr::null_mut()) };
        if ret_val != 0 {
            return Err(self.get_error().unwrap());
        }
        Ok(())
    }
    pub fn sync(&mut self) -> Result<(), String> {
        let ret_val = unsafe { ffi::gd_sync(self.dirfile.unwrap().as_ptr(), std::ptr::null_mut()) };
        if ret_val != 0 {
            return Err(self.get_error().unwrap());
        }
        Ok(())
    }
    pub fn metaflush(&mut self) -> Result<(), String> {
        let ret_val = unsafe { ffi::gd_metaflush(self.dirfile.unwrap().as_ptr()) };
        if ret_val != 0 {
            return Err(self.get_error().unwrap());
        }
        Ok(())
    }

    pub fn get_error(&self) -> Option<String> {
        let error = unsafe { ffi::gd_error(self.dirfile.unwrap().as_ptr()) };
        if error == ffi::GD_E_OK as i32 {
            return None;
        }
        let error_string_ptr = unsafe { ffi::gd_error_string(self.dirfile.unwrap().as_ptr(), std::ptr::null_mut(), 0) };
    let error_string_cstr: &CStr = unsafe { CStr::from_ptr(error_string_ptr) };
    let error_string = error_string_cstr.to_string_lossy().into_owned();
    unsafe { libc::free(error_string_ptr as *mut libc::c_void) };  // Free the error string
    Some(error_string)
    }
}
