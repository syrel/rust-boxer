use boxer::point::{BoxerPointF64};

#[no_mangle]
pub fn boxer_point_f64_create() -> *mut BoxerPointF64 {
    BoxerPointF64::boxer_point_create()
}

#[no_mangle]
pub fn boxer_point_f64_drop(_ptr: *mut BoxerPointF64) {
    BoxerPointF64::boxer_point_drop(_ptr);
}

#[no_mangle]
pub fn boxer_point_f64_get_x(_point_ptr: *mut BoxerPointF64) -> f64 {
    BoxerPointF64::boxer_point_get_x(_point_ptr)
}

#[no_mangle]
pub fn boxer_point_f64_set_x(_point_ptr: *mut BoxerPointF64, x: f64) {
    BoxerPointF64::boxer_point_set_x(_point_ptr, x);
}

#[no_mangle]
pub fn boxer_point_f64_get_y(_point_ptr: *mut BoxerPointF64) -> f64 {
    BoxerPointF64::boxer_point_get_y(_point_ptr)
}

#[no_mangle]
pub fn boxer_point_f64_set_y(_point_ptr: *mut BoxerPointF64, y: f64) {
    BoxerPointF64::boxer_point_set_y(_point_ptr, y);
}