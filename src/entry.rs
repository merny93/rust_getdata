use crate::ffi::{self, gd_entype_t_GD_RAW_ENTRY};
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
    Lincom,
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

struct LinterpData {
    in_field: CString,
    table: CString,
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
    pub fn from_c(field_code: &str, entry_c: ffi::gd_entry_t) -> Entry {
        let field_code_c: CString = CString::new(field_code).unwrap();

        let entry_type: EntryType = unsafe {
            match entry_c.field_type {
                ffi::gd_entype_t_GD_RAW_ENTRY => {
                    EntryType::Raw(RawData {
                        spf: entry_c.__bindgen_anon_1.__bindgen_anon_1.spf,
                        gd_type: entry_c.__bindgen_anon_1.__bindgen_anon_1.data_type,
                    })
                }
                ffi::gd_entype_t_GD_LINTERP_ENTRY => {
                    EntryType::Linterp(LinterpData {
                        in_field: CString::from_raw(entry_c.in_fields[0] as *mut i8),
                        table: CString::from_raw(entry_c.__bindgen_anon_1.__bindgen_anon_6.table as *mut i8),
                    })
                }
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
