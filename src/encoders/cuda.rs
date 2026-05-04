use std::ffi::c_void;

use cust::sys::{CUcontext, CUstream};

#[repr(C)]
pub struct AVCUDADeviceContext {
    pub cuda_ctx: CUcontext,
    pub stream: CUstream,
    pub internal: *mut c_void,
}
