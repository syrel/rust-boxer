use boxer::CBox;
use boxer::size::{BoxerSizeU64};

#[no_mangle]
pub fn boxer_size_u64_create() -> *mut BoxerSizeU64 {
    CBox::into_raw(BoxerSizeU64::default())
}

#[no_mangle]
pub fn boxer_size_u64_drop(_ptr: *mut BoxerSizeU64) {
    CBox::drop(_ptr)
}

#[no_mangle]
pub fn boxer_size_u64_get_width(_size_ptr: *mut BoxerSizeU64) -> u64 {
    CBox::with_optional_raw(_size_ptr, |option| match option {
        None => 0,
        Some(size) => { size.width },
    } )
}

#[no_mangle]
pub fn boxer_size_u64_set_width(_size_ptr: *mut BoxerSizeU64, width: u64) {
    CBox::with_optional_raw(_size_ptr, |option| match option {
        None => {},
        Some(size) => { size.width = width },
    } )
}

#[no_mangle]
pub fn boxer_size_u64_get_height(_size_ptr: *mut BoxerSizeU64) -> u64 {
    CBox::with_optional_raw(_size_ptr, |option| match option {
        None => 0,
        Some(size) => { size.height },
    } )
}

#[no_mangle]
pub fn boxer_size_u64_set_height(_size_ptr: *mut BoxerSizeU64, height: u64) {
    CBox::with_optional_raw(_size_ptr, |option| match option {
        None => {},
        Some(size) => { size.height = height },
    } )
}