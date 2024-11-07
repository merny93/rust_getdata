use std::any::TypeId;


use crate::ffi;

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

impl From<TypeId> for GdTypes {
    fn from(value: TypeId) -> Self {
        if value == TypeId::of::<f32>() {
            GdTypes::Float32
        } else if value == TypeId::of::<f64>() {
            GdTypes::Float64
        } else if value == TypeId::of::<i32>() {
            GdTypes::Int32
        } else if value == TypeId::of::<i64>() {
            GdTypes::Int64
        } else if value == TypeId::of::<u32>() {
            GdTypes::Uint32
        } else if value == TypeId::of::<u64>() {
            GdTypes::Uint64
        } else {
            panic!("Unsupported type");
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
