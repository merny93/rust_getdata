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
    let mut dirfile = super::Dirfile::open(file_name).unwrap();
    dirfile.close();
    //check for the existance of the folder
    let path = std::path::Path::new(file_name);
    assert!(path.exists());
    // //delete the folder
    std::fs::remove_dir_all(file_name).unwrap();
}

#[test]
fn test_highlevel_add_entry(){
    use super::*;
    let file_name = "__testdirfile4__";
    let path = std::path::Path::new(file_name);
    if path.exists() {
        std::fs::remove_dir_all(file_name).unwrap();
    }
    let mut dirfile = super::Dirfile::open(file_name).unwrap();
    let entry = Entry::new("testfield", 0, EntryType::new_raw(GdTypes::Float32, 10));
    dirfile.add(&entry).unwrap();
    dirfile.close();
    // //check for the existance of the folder
    let path = std::path::Path::new(file_name);
    assert!(path.exists());

    // there should be a format file inside the folder which contains 
    // /VERSION 10
    // /ENDIAN little
    // /PROTECT none
    // /ENCODING none
    // testfield RAW FLOAT32 10
    // /REFERENCE testfield
    // this will be somewhere in that file but not at the top
    let format_file = std::fs::read_to_string(format!("{}/format", file_name)).unwrap();
    assert!(format_file.contains("testfield RAW FLOAT32 10"));
    assert!(format_file.contains("/REFERENCE testfield")); 

    let mut dirfile = Dirfile::open(file_name).unwrap();
    let entry = dirfile.get_entry("testfield").unwrap();
    assert_eq!(entry.field_code, "testfield");

    //try to put data!
    let npoint = 33;
    let data: Vec<f32> = vec![42.0; npoint];
    dirfile.putdata(FieldOrEntry::Field("testfield".to_string()), &data).unwrap();
    dirfile.putdata(FieldOrEntry::Field("testfield".to_string()), &data).unwrap();
    dirfile.putdata(FieldOrEntry::Field("testfield".to_string()), &data).unwrap();

    dirfile.close();

    //open again to read and double check that it works
    let mut dirfile = Dirfile::open(file_name).unwrap();
    let mut data_read: Vec<f32> = vec![0.0; npoint];
    let fc = CString::new("testfield").unwrap();
    let n_pts = unsafe {ffi::gd_getdata(dirfile.dirfile.unwrap().as_ptr(), fc.as_ptr(), 0, 0, 3, 3, gd_type_t_GD_FLOAT32, data_read.as_mut_ptr() as *mut c_void) };
    assert!(n_pts == npoint);
    assert_eq!(data, data_read);
    dirfile.close();



    //delete the folder
    std::fs::remove_dir_all(file_name).unwrap();
}


#[test]
fn test_highlevel_error(){
    //lets try to read from a field that does not exist
    let file_name = "__fakedirfilenoexist_";
    let dirfile = super::Dirfile::open(file_name);
    //try to read from it
    let mut data = vec![0.0; 10];
    let dirfile = dirfile.unwrap();
    unsafe{
    let fieldcode = CString::new("testfield").unwrap();
    let n_pts = gd_getdata(dirfile.dirfile.unwrap().as_ptr(), fieldcode.as_ptr(), 0, 0, 10, 0, gd_type_t_GD_FLOAT32, data.as_mut_ptr() as *mut c_void);
    }
    let er = dirfile.get_error().unwrap();
    assert_eq!(er., Some("Field not found: testfield".to_string()));

    std::fs::remove_dir_all(file_name).unwrap();

}