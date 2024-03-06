#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests{
    use super::*;
    use std::ffi::CString;
    use std::ffi::c_void;
    #[test]
    fn test_gd_open_close() {
        let file_name = "__testdirfile__";
        unsafe {
            let filename = CString::new(file_name).unwrap();
            let file = gd_open(filename.as_ptr(), (GD_RDWR | GD_CREAT).into());
            gd_sync(file, std::ptr::null_mut());
            gd_close(file);
        }
        // //check for the existance of the folder
        let path = std::path::Path::new(file_name);
        assert!(path.exists());
        // //delete the folder
        std::fs::remove_dir_all(file_name).unwrap();
    }

    #[test]
    fn test_gd_putgetdata(){
        let file_name = "__testdirfile2__";
        unsafe {
            let filename= CString::new(file_name).unwrap();
            let fieldcode = CString::new("testfield").unwrap();

            //open dirfile
            let dirfile = gd_open(filename.as_ptr(), (GD_RDWR | GD_CREAT).into());
            
            //add the field and flush metadata
            gd_add_raw(dirfile, fieldcode.as_ptr(),  gd_type_t_GD_FLOAT32, 1, 0);
            gd_metaflush(dirfile);
            let sync_result = gd_sync(dirfile, std::ptr::null_mut());
            assert_eq!(sync_result, 0, "gd_sync failed with error: {}", sync_result);

            let npoint = 10;
            let mut data: Vec<f32> = vec![3.0; npoint];
            gd_putdata(dirfile, fieldcode.as_ptr(), 0, 0,npoint, 0, gd_type_t_GD_FLOAT32, data.as_mut_ptr() as *mut c_void);
            // assert_eq!(putdata_result, npoint, "gd_putdata failed with error: {}", putdata_result);
  
            gd_sync(dirfile, std::ptr::null_mut());

            //read the data back
            let mut data_read: Vec<f32> = vec![0.0; npoint];

            let n_pts = gd_getdata(dirfile, fieldcode.as_ptr(), 0, 0, npoint, 0, gd_type_t_GD_FLOAT32, data_read.as_mut_ptr() as *mut c_void);
            // assert_eq!(n_pts, npoint, "gd_getdata failed with error: {}", n_pts);
            assert_eq!(data, data_read, "data read from file is not the same as data written to file");

            gd_close(dirfile);
        }
        //check for the existance of the folder
        let path = std::path::Path::new(file_name);
        assert!(path.exists());
        // //delete the folder
        std::fs::remove_dir_all(file_name).unwrap();
        
    }
}