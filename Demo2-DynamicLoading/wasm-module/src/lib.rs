

/// A simple function returning a number as this is the most simple and native data type supported by WASM
/// returns a number
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 { return a + b}

#[no_mangle]
pub extern "C" fn sub(a: i32, b: i32) -> i32 { return a - b}

#[no_mangle]
pub extern "C" fn mul(a: i32, b: i32) -> i32 { return a * b}

#[no_mangle]
pub extern "C" fn div(a: i32, b: i32) -> i32 { return a / b}
