pub mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
// basic tests to confirm that getdata is working as expected
#[cfg(test)]
mod tests;

mod entry;

pub use entry::{Entry, EntryType};

mod gd_error;

pub use gd_error::GdError;

use std::any::TypeId;

use std::ffi::CString;

//lets make a struct to hold the dirfile
#[derive(Default)]
pub struct Dirfile{
    dirfile: Option<std::ptr::NonNull<ffi::DIRFILE>>,
}

#[derive(Clone, Copy)]
pub enum GdTypes {
    Float32,
    Float64,
    Int32,
    Int64,
    Uint32,
    Uint64,
}

impl From<GdTypes> for ffi::gd_type_t {
    fn from(gd_type: GdTypes) -> Self {
        match gd_type {
            GdTypes::Float32 => ffi::gd_type_t_GD_FLOAT32,
            GdTypes::Float64 => ffi::gd_type_t_GD_FLOAT64,
            GdTypes::Int32 => ffi::gd_type_t_GD_INT32,
            GdTypes::Int64 => ffi::gd_type_t_GD_INT64,
            GdTypes::Uint32 => ffi::gd_type_t_GD_UINT32,
            GdTypes::Uint64 => ffi::gd_type_t_GD_UINT64,
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
            ffi::gd_type_t_GD_UINT32 => GdTypes::Uint32,
            ffi::gd_type_t_GD_UINT64 => GdTypes::Uint64,
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
            GdTypes::Uint32 => TypeId::of::<u32>(),
            GdTypes::Uint64 => TypeId::of::<u64>(),
        }
    }
}

pub enum FieldOrEntry {
    Field(String),
    Entry(Entry),
}

impl Dirfile {
    /// Open a dirfile in read/write mode, creating it if it does not exist
    pub fn open(dirfile_name: &str) -> Result<Dirfile, GdError> {
        let dirfile_name = CString::new(dirfile_name).unwrap();
        let dirfile =
            unsafe { ffi::gd_open(dirfile_name.as_ptr(), (ffi::GD_RDWR | ffi::GD_CREAT).into()) };
        let df = Dirfile {
            dirfile: std::ptr::NonNull::new(dirfile),
            ..Default::default()
        };
        match df.get_error() {
            None => Ok(Dirfile {
                dirfile: std::ptr::NonNull::new(dirfile),
                ..Default::default()
            }),
            Some(error) => {
                unsafe { ffi::gd_close(dirfile) };
                Err(error)
            }
        }
    }
    /// Close the dirfile
    pub fn close(&mut self) {
        unsafe { ffi::gd_close(self.dirfile.expect("open dirfile!").as_ptr()) };
        self.dirfile = None;
    }

    /// add entry
    pub fn add(&mut self, entry: &Entry) -> Result<(), GdError> {
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

    pub fn add_alias(
        &mut self,
        alias_name: &str,
        field_or_entry: FieldOrEntry,
    ) -> Result<(), GdError> {
        let alias_name = CString::new(alias_name).unwrap();

        let field_code_c = match field_or_entry {
            FieldOrEntry::Field(field_code) => CString::new(field_code).unwrap(),
            FieldOrEntry::Entry(entry) => entry.field.clone(),
        };

        let ret_val = unsafe {
            ffi::gd_add_alias(
                self.dirfile.expect("Open the dirfile!").as_ptr(),
                alias_name.as_ptr(),
                field_code_c.as_ptr(),
                0,
            )
        };
        if ret_val == 0 {
            Ok(())
        } else {
            Err(self.get_error().unwrap())
        }
    }
    pub fn get_entry(&self, field_code: &str) -> Result<Entry, GdError> {
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

        let entry = Entry::from_c(field_code.to_str().unwrap(), entry_c);
        Ok(entry)
    }

    pub fn putdata_field<T: 'static>(
        &self,
        field_code: &str,
        data: &Vec<T>,
    ) -> Result<usize, GdError> {
        let entry = self.get_entry(field_code)?;
        self.putdata(&entry, data)
    }

    /// puts data vectors, returns if the write was successful
    pub fn putdata<T: 'static>(
        &self,
        entry: &Entry,
        data: &Vec<T>,
    ) -> Result<usize, GdError> {
        match &entry.field_type {
            // only raw data is supported for now
            EntryType::Raw(raw_data) => {
                // check that the type is correct
                let gd_type = GdTypes::from(raw_data.gd_type);
                assert_eq!(
                    TypeId::from(gd_type),
                    TypeId::of::<T>(),
                    "Data type mismatch"
                );
                //figure out how much data to write
                let num_frames: usize = data.len() / raw_data.spf as usize;
                let num_samples: usize = data.len() % raw_data.spf as usize;

                //write data, will need to update the offset and stuff...
                let write_n = unsafe {
                    ffi::gd_putdata(
                        self.dirfile.expect("Open the dirfile!").as_ptr(),
                        entry.field.as_ptr() as *const i8,
                        ffi::GD_HERE.into(),
                        0,
                        num_frames,
                        num_samples,
                        raw_data.gd_type,
                        data.as_ptr() as *const std::ffi::c_void,
                    )
                };
                if write_n != data.len() {
                    match self.get_error() {
                        Some(error) => return Err(error),
                        None => {}
                    };
                }
                Ok(write_n)
            }
            _ => {
                panic!("only put data for RAW");
            }
        }
    }

    pub fn flush(&mut self) -> Result<(), GdError> {
        let ret_val =
            unsafe { ffi::gd_flush(self.dirfile.unwrap().as_ptr(), std::ptr::null_mut()) };
        if ret_val != 0 {
            return Err(self.get_error().unwrap());
        }
        Ok(())
    }
    pub fn sync(&mut self) -> Result<(), GdError> {
        let ret_val = unsafe { ffi::gd_sync(self.dirfile.unwrap().as_ptr(), std::ptr::null_mut()) };
        if ret_val != 0 {
            return Err(self.get_error().unwrap());
        }
        Ok(())
    }
    pub fn metaflush(&mut self) -> Result<(), GdError> {
        let ret_val = unsafe { ffi::gd_metaflush(self.dirfile.unwrap().as_ptr()) };
        if ret_val != 0 {
            return Err(self.get_error().unwrap());
        }
        Ok(())
    }
}
