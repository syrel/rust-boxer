use boxer::boxes::{ValueBox, ValueBoxPointer};
use boxer::string::BoxerString;

#[no_mangle]
pub fn boxer_string_create() -> *mut ValueBox<BoxerString> {
    ValueBox::new(BoxerString::new()).into_raw()
}

/// I copy the data (must *not* contain zero-byte).
/// length must not include the zero-byte
#[no_mangle]
pub fn boxer_string_from_byte_string(data: *const u8, length: usize) -> *mut ValueBox<BoxerString> {
    ValueBox::new(unsafe { BoxerString::from_byte_string_data(data, length) }).into_raw()
}

/// I copy the data (must *not* contain zero-byte).
/// length must not include the zero-byte
#[no_mangle]
pub fn boxer_string_from_wide_string(
    data: *const u32,
    length: usize,
) -> *mut ValueBox<BoxerString> {
    ValueBox::new(unsafe { BoxerString::from_wide_string_data(data, length) }).into_raw()
}

/// I copy the data (must contain zero-byte).
/// length must not include the zero-byte
#[no_mangle]
pub fn boxer_string_from_utf8_string(data: *const u8, length: usize) -> *mut ValueBox<BoxerString> {
    ValueBox::new(unsafe { BoxerString::from_utf8_string_data(data, length) }).into_raw()
}

#[no_mangle]
pub fn boxer_string_drop(ptr: *mut ValueBox<BoxerString>) {
    ptr.drop()
}

#[no_mangle]
pub fn boxer_string_get_len(ptr: *mut ValueBox<BoxerString>) -> usize {
    ptr.with_not_null_return(0, |string| string.len())
}

#[no_mangle]
pub fn boxer_string_get_char_count(ptr: *mut ValueBox<BoxerString>) -> usize {
    ptr.with_not_null_return(0, |string| string.char_count())
}

#[no_mangle]
pub fn boxer_string_get_ptr(ptr: *mut ValueBox<BoxerString>) -> *const u8 {
    ptr.with_not_null_return(std::ptr::null(), |string| string.as_ptr())
}
