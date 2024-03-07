use super::ffi::*;
use std::ffi::CString;
use std::ffi::c_void;
#[test]
fn test_gd_open_close() {
    let file_name = "__testdirfile__";
    //check if the folder already exists and delete it if so
    let path = std::path::Path::new(file_name);
    if path.exists() {
        std::fs::remove_dir_all(file_name).unwrap();
    }
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
    let path = std::path::Path::new(file_name);
    if path.exists() {
        std::fs::remove_dir_all(file_name).unwrap();
    }
    unsafe {
        let filename= CString::new(file_name).unwrap();
        let fieldcode = CString::new("testfield").unwrap();

        //open dirfile
        let dirfile = gd_open(filename.as_ptr(), (GD_RDWR | GD_CREAT).into());
        
        //add the field and flush metadata
        gd_add_raw(dirfile, fieldcode.as_ptr(),  gd_type_t_GD_FLOAT32, 1, 0);
        gd_metaflush(dirfile);
        gd_flush(dirfile, std::ptr::null_mut());


        let npoint = 10;
        let mut data: Vec<f32> = vec![42.0; npoint];
        let write_n = gd_putdata(dirfile, fieldcode.as_ptr(), 0, 0,npoint, 0, gd_type_t_GD_FLOAT32, data.as_mut_ptr() as *mut c_void);
        assert_eq!(write_n, npoint, "gd_putdata failed with error: {}", write_n);

        gd_flush(dirfile, std::ptr::null_mut());
        gd_close(dirfile);

        //reopen the file
        let dirfile = gd_open(filename.as_ptr(), (GD_RDWR | GD_CREAT).into());

        //read the data back
        let mut data_read: Vec<f32> = vec![0.0; npoint];

        let n_pts = gd_getdata(dirfile, fieldcode.as_ptr(), 0, 0, npoint, 0, gd_type_t_GD_FLOAT32, data_read.as_mut_ptr() as *mut c_void);
        assert_eq!(n_pts, npoint, "gd_getdata failed with error: {}", n_pts);
        assert_eq!(data, data_read, "data read from file is not the same as data written to file");

        gd_close(dirfile);
    }
    //check for the existance of the folder
    let path = std::path::Path::new(file_name);
    assert!(path.exists());
    // //delete the folder
    std::fs::remove_dir_all(file_name).unwrap();
    
}


#[test]
fn test_highlevel_open_close() {
    let file_name = "__testdirfile3__";
    let path = std::path::Path::new(file_name);
    if path.exists() {
        std::fs::remove_dir_all(file_name).unwrap();
    }
    let mut dirfile = super::Dirfile::open(file_name);
    dirfile.close();
    //check for the existance of the folder
    let path = std::path::Path::new(file_name);
    assert!(path.exists());
    // //delete the folder
    std::fs::remove_dir_all(file_name).unwrap();
}
