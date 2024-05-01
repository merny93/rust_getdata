use crate::ffi;
use std::ffi::CString;
pub struct Entry {
    pub field: CString,
    pub field_type: EntryType,
    // flags: None, //non implemented
    pub entry_c: ffi::gd_entry_t,
}

pub enum EntryType {
    No,
    Bit,
    Carry,
    Const,
    Divide,
    Lincom(LincomData),
    Linterp(LinterpData),
    Mplex,
    Multiply,
    Phase,
    Polynom,
    Raw(RawData),
    Recip,
    Sbit,
    String,
    Window,
    Index,
}

pub struct RawData {
    pub spf: u32,
    pub gd_type: ffi::gd_type_t,
}

pub struct LinterpData {
    in_field: CString,
    table: CString,
}

pub struct LincomData {
    in_fields: (CString, CString, CString),
    n_fields: i32,
    m: (f64, f64, f64),
    b: (f64, f64, f64),
}

impl Entry {
    pub fn get_field_code(&self) -> &str {
        self.field.to_str().unwrap()
    }
    fn _new(field_code: &str) -> Entry {
        let field = CString::new(field_code).unwrap();
        let entry_c: ffi::gd_entry_t;
        unsafe {
            entry_c = std::mem::zeroed();
        }
        let type_data = EntryType::No;
        let mut entry = Entry {
            field: field,
            field_type: type_data,
            entry_c: entry_c,
        };
        entry.entry_c.field = entry.field.as_ptr() as *mut i8;
        entry
    }
    pub fn new_raw(field_code: &str, spf: u32, gd_type: crate::GdTypes) -> Entry {
        let type_data = EntryType::Raw(RawData {
            spf: spf,
            gd_type: gd_type.into(),
        });
        let mut entry = Entry::_new(field_code); //assigns "field"
        entry.field_type = type_data; //asigns rust field type
        entry.entry_c.field_type = ffi::gd_entype_t_GD_RAW_ENTRY;
        entry.entry_c.__bindgen_anon_1.__bindgen_anon_1.data_type = gd_type.into();
        entry.entry_c.__bindgen_anon_1.__bindgen_anon_1.spf = spf;
        entry
    }
    pub fn new_linterp(field_code: &str, in_field: &str, table: &str) -> Entry {
        let mut entry = Entry::_new(field_code); //assigns "field"
        let linterp_data = LinterpData {
            in_field: CString::new(in_field).unwrap(),
            table: CString::new(table).unwrap(),
        };

        entry.entry_c.field_type = ffi::gd_entype_t_GD_LINTERP_ENTRY;
        entry.entry_c.in_fields[0] = linterp_data.in_field.as_ptr() as *mut i8;
        entry.entry_c.__bindgen_anon_1.__bindgen_anon_6.table =
            linterp_data.table.as_ptr() as *mut i8;

        let type_data = EntryType::Linterp(linterp_data);
        entry.field_type = type_data;

        entry
    }
    pub fn new_lincom(
        field_code: &str,
        in_fields: Vec<&str>,
        m: Vec<f64>,
        b: Vec<f64>,
    ) -> Entry {
        let mut entry = Entry::_new(field_code); //assigns "field"
        let n_fields = in_fields.len() as i32;
        let mut lincom_data = LincomData {
            in_fields: (
                CString::new("").unwrap(),
                CString::new("").unwrap(),
                CString::new("").unwrap(),
            ),
            n_fields: n_fields,
            m: (0.0, 0.0, 0.0),
            b: (0.0, 0.0, 0.0),
        };
        match n_fields {
            1 => {
                lincom_data.in_fields.0 = CString::new(in_fields[0]).unwrap();
                entry.entry_c.in_fields[0] = lincom_data.in_fields.0.as_ptr() as *mut i8;
                lincom_data.n_fields = 1;
                lincom_data.m.0 = m[0];
                lincom_data.b.0 = b[0];
            }
            2 => {
                lincom_data.in_fields.0 = CString::new(in_fields[0]).unwrap();
                lincom_data.in_fields.1 = CString::new(in_fields[1]).unwrap();
                entry.entry_c.in_fields[0] = lincom_data.in_fields.0.as_ptr() as *mut i8;
                entry.entry_c.in_fields[1] = lincom_data.in_fields.1.as_ptr() as *mut i8;
                lincom_data.n_fields = 2;
                lincom_data.m.0 = m[0];
                lincom_data.m.1 = m[1];
                lincom_data.b.0 = b[0];
                lincom_data.b.1 = b[1];
            }
            3 => {
                lincom_data.in_fields.0 = CString::new(in_fields[0]).unwrap();
                lincom_data.in_fields.1 = CString::new(in_fields[1]).unwrap();
                lincom_data.in_fields.2 = CString::new(in_fields[2]).unwrap();
                entry.entry_c.in_fields[0] = lincom_data.in_fields.0.as_ptr() as *mut i8;
                entry.entry_c.in_fields[1] = lincom_data.in_fields.1.as_ptr() as *mut i8;
                entry.entry_c.in_fields[2] = lincom_data.in_fields.2.as_ptr() as *mut i8;
                lincom_data.n_fields = 3;
                lincom_data.m.0 = m[0];
                lincom_data.m.1 = m[1];
                lincom_data.m.2 = m[2];
                lincom_data.b.0 = b[0];
                lincom_data.b.1 = b[1];
                lincom_data.b.2 = b[2];
            }
            _ => panic!("Invalid number of fields"),
        }

        entry.entry_c.field_type = ffi::gd_entype_t_GD_LINCOM_ENTRY;
        entry.entry_c.__bindgen_anon_1.__bindgen_anon_2.n_fields = n_fields;
        unsafe {
            entry.entry_c.__bindgen_anon_1.__bindgen_anon_2.m[0] = lincom_data.m.0;
            entry.entry_c.__bindgen_anon_1.__bindgen_anon_2.m[1] = lincom_data.m.1;
            entry.entry_c.__bindgen_anon_1.__bindgen_anon_2.m[2] = lincom_data.m.2;
            entry.entry_c.__bindgen_anon_1.__bindgen_anon_2.b[0] = lincom_data.b.0;
            entry.entry_c.__bindgen_anon_1.__bindgen_anon_2.b[1] = lincom_data.b.1;
            entry.entry_c.__bindgen_anon_1.__bindgen_anon_2.b[2] = lincom_data.b.2;
        }

        let type_data = EntryType::Lincom(lincom_data);
        entry.field_type = type_data;

        entry
    }
    pub fn from_c(field_code: &str, entry_c: ffi::gd_entry_t) -> Entry {
        let field_code_c: CString = CString::new(field_code).unwrap();

        let entry_type: EntryType = unsafe {
            match entry_c.field_type {
                ffi::gd_entype_t_GD_RAW_ENTRY => EntryType::Raw(RawData {
                    spf: entry_c.__bindgen_anon_1.__bindgen_anon_1.spf,
                    gd_type: entry_c.__bindgen_anon_1.__bindgen_anon_1.data_type,
                }),
                ffi::gd_entype_t_GD_LINTERP_ENTRY => EntryType::Linterp(LinterpData {
                    in_field: CString::from_raw(entry_c.in_fields[0] as *mut i8),
                    table: CString::from_raw(
                        entry_c.__bindgen_anon_1.__bindgen_anon_6.table as *mut i8,
                    ),
                }),
                _ => panic!("Unknown entry type, memory leak!"),
            }
        };
        Entry {
            field: field_code_c,
            field_type: entry_type,
            entry_c: entry_c,
        }
    }
}
