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

use std::collections::HashMap;
use std::error::Error;
use std::ffi::CString;
use std::sync::Mutex;

//lets make a struct to hold the dirfile
#[derive(Default)]
pub struct Dirfile {
    dirfile: Option<std::ptr::NonNull<ffi::DIRFILE>>,
    write_mutex: Mutex<()>,
    entries: HashMap<String, Entry>,
}

unsafe impl Send for Dirfile {}

mod gd_types;

impl Dirfile {
    pub fn dirfile_get(&self) -> Result<*mut ffi::DIRFILE, GdError> {
        match self.dirfile {
            Some(dirfile) => Ok(dirfile.as_ptr()),
            None => Err(GdError::InternalError("Dirfile is not open".to_string())),
        }
    }
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
            None => Ok(df),
            Some(error) => {
                unsafe { ffi::gd_close(dirfile) };
                Err(error)
            }
        }
    }
    /// Close the dirfile
    pub fn close(&mut self) -> Result<(), Box<dyn Error + '_>> {
        let _guard = self.write_mutex.try_lock()?;
        //read the errors
        let error = self.get_error();

        unsafe { ffi::gd_close(self.dirfile_get()?) };
        self.dirfile = None;
        if let Some(error) = error {
            return Err(Box::new(error));
        }
        Ok(())
    }

    /// add entry
    pub fn add(&mut self, entry: &Entry) -> Result<(), Box<dyn Error + '_>> {
        let _guard = self.write_mutex.try_lock()?;
        let ret_val = unsafe { ffi::gd_add(self.dirfile_get()?, &entry.entry_c) };
        if ret_val == 0 {
            Ok(())
        } else {
            Err(self.get_error_boxed())
        }
    }

    /// Add alias directive into the format file
    /// This function does not check if the field actually exists
    pub fn add_alias(&mut self, alias_name: &str, field: &str) -> Result<(), Box<dyn Error + '_>> {
        let alias_name = CString::new(alias_name)?;

        let field_code_c = CString::new(field)?;

        let _guard = self.write_mutex.try_lock()?;

        let ret_val = unsafe {
            ffi::gd_add_alias(
                self.dirfile_get()?,
                alias_name.as_ptr(),
                field_code_c.as_ptr(),
                0,
            )
        };
        if ret_val == 0 {
            Ok(())
        } else {
            Err(self.get_error_boxed())
        }
    }
    pub fn pull_entry(&mut self, field_code: &str) -> Result<(), Box<dyn Error>> {
        //check if the entry is already in the hashmap
        if self.entries.contains_key(field_code) {
            return Ok(());
        }

        let field_code_c = CString::new(field_code).unwrap();
        let mut entry_c = unsafe { std::mem::zeroed::<ffi::gd_entry_t>() };
        let ret_val =
            unsafe { ffi::gd_entry(self.dirfile_get()?, field_code_c.as_ptr(), &mut entry_c) };
        if ret_val != 0 {
            return Err(self.get_error_boxed());
        }

        let entry = Entry::from_c(field_code, entry_c);

        //free the memory
        // turns out the strings are allocated on the caller's heap
        unsafe { ffi::gd_free_entry_strings(&mut entry_c) };

        //put the entry in the hashmap but keep a reference to it
        self.entries.insert(field_code.to_string(), entry);
        Ok(())
    }

    /// puts data vectors, returns if the write was successful
    pub fn putdata<T: 'static>(
        &mut self,
        field: &str,
        data: &Vec<T>,
    ) -> Result<usize, Box<dyn Error + '_>> {
        let entry = {
            self.pull_entry(field)?;
            self.entries.get(field).expect("imposi")
        };

        if let EntryType::Raw(raw_data) = &entry.field_type {
            // see if we can find a function to convert the data to the correct type
            let gd_rust = TypeId::from(gd_types::GdTypes::from(raw_data.gd_type));

            if gd_rust != TypeId::of::<T>() {
                return Err(Box::new(GdError::BadType(
                    "Data type does not match field type".to_string(),
                )));
            }
            // //figure out how much data to write
            let num_frames: usize = data.len() / raw_data.spf as usize;
            let num_samples: usize = data.len() % raw_data.spf as usize;

            //write data, will need to update the offset and stuff...
            let _guard = self.write_mutex.try_lock()?;
            let write_n = unsafe {
                ffi::gd_putdata(
                    self.dirfile_get()?,
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
                    Some(error) => return Err(Box::new(error)),
                    None => {}
                };
            }
            Ok(write_n)
        } else {
            Err(Box::new(GdError::BadType(
                "Field is not of type Raw".to_string(),
            )))
        }
    }
    pub fn getdata<T: 'static>(
        &self,
        field: &str,
        first_frame: usize,
        first_sample: usize,
        num_frames: usize,
        num_samples: usize,
    ) -> Result<Vec<T>, Box<dyn Error>> {
        //get the spf
        let field_code_c = CString::new(field)?;
        let spf = unsafe { ffi::gd_spf(self.dirfile_get()?, field_code_c.as_ptr()) };
        if spf == 0 {
            return Err(self.get_error_boxed());
        }
        let spf = spf as usize;

        let gd_type = gd_types::GdTypes::from(TypeId::of::<T>());

        let n_points_request = num_frames * spf + num_samples;
        let mut data: Vec<T> = Vec::with_capacity(n_points_request as usize);
        let n_pts = unsafe {
            ffi::gd_getdata(
                self.dirfile_get()?,
                field_code_c.as_ptr(),
                first_frame as i64,
                first_sample as i64,
                num_frames.into(),
                num_samples.into(),
                gd_type.into(),
                data.as_mut_ptr() as *mut std::ffi::c_void,
            )
        };
        if n_pts == 0 {
            return Err(self.get_error_boxed());
        }
        //otherwise set the length of the data vector to the number of points
        unsafe { data.set_len(n_pts as usize) };
        Ok(data)
    }

    pub fn flush(&mut self) -> Result<(), GdError> {
        let ret_val = unsafe { ffi::gd_flush(self.dirfile_get()?, std::ptr::null_mut()) };
        if ret_val != 0 {
            return Err(self.get_error().unwrap());
        }
        Ok(())
    }
    pub fn sync(&mut self) -> Result<(), GdError> {
        let ret_val = unsafe { ffi::gd_sync(self.dirfile_get()?, std::ptr::null_mut()) };
        if ret_val != 0 {
            return Err(self.get_error().unwrap());
        }
        Ok(())
    }
    pub fn metaflush(&mut self) -> Result<(), GdError> {
        let ret_val = unsafe { ffi::gd_metaflush(self.dirfile_get()?) };
        if ret_val != 0 {
            return Err(self.get_error().unwrap());
        }
        Ok(())
    }
}
