#![allow(non_snake_case)]

use std::env;
use dlopen2::wrapper::{Container, WrapperApi};

static CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR: i32 = 75;
static CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR: i32 = 76;


#[derive(WrapperApi)]
struct Api {
    cuInit: unsafe extern "C" fn(options: i32) -> i32,
    cuDriverGetVersion: unsafe extern "C" fn(version: *mut i32) -> i32,
    cuDeviceGetCount: unsafe extern "C" fn(count: *mut i32) -> i32,
    cuDeviceGetAttribute: unsafe extern "C" fn(value: *mut i32, attr: i32, device: i32) -> i32,
}

pub fn main(){
    let names: Vec<String> = match env::consts::OS {
        "linux" => vec![
            "libcuda.so".to_string(),  // check library path first
            "/usr/lib64/nvidia/libcuda.so".to_string(),  // RHEL/Centos/Fedora
            "/usr/lib/x86_64-linux-gnu/libcuda.so".to_string(),  // Ubuntu
            "/usr/lib/wsl/lib/libcuda.so".to_string(),  // WSL
        ],
        "windows" => vec![
            "nvcuda64.dll".to_string(),
            "nvcuda.dll".to_string(),
        ],
        _ => vec![],
    };
    unsafe  {
        if let Some(cont) = names.into_iter().find_map(|name| Container::<Api>::load(name).ok()) {
            cont.cuInit(0);
            let mut version = 0;
            cont.cuDriverGetVersion(&mut version);
            let version_major = version / 1000;
            let version_minor = (version % 1000) / 10;
            println!("CUDA Driver Version: {}.{}", version_major, version_minor);
            let mut count = 0;
            let return_code = cont.cuDeviceGetCount(&mut count);
            if return_code == 0 {
                println!("CUDA Device Count: {}", count);
            } else {
                // println!("Error getting CUDA Device Count. Return code: {}", CStr::from_ptr(cont.cudaGetErrorString(return_code)).to_str().unwrap());
            }
            for device in 0..count {
                let mut major = 0;
                let mut minor = 0;
                cont.cuDeviceGetAttribute(&mut major, CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR, device);
                cont.cuDeviceGetAttribute(&mut minor, CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR, device);
                println!("CUDA Device {} Compute Capability: {}.{}", device, major, minor);
            }
        } else { println!("CUDA library not found"); }
    }
}