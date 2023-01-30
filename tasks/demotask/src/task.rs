#[allow(clippy::all)]
pub mod host {
    #[allow(clippy::all)]
    pub fn test() -> () {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "host")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "test")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "host_test")]
                fn wit_import();
            }
            wit_import();
        }
    }
}

pub trait Task {
    fn init() -> i32;
    fn run() -> i32;
    fn dispose() -> i32;
}

#[doc(hidden)]
pub unsafe fn call_init<T: Task>() -> i32 {
    let result0 = T::init();
    wit_bindgen_guest_rust::rt::as_i32(result0)
}

#[doc(hidden)]
pub unsafe fn call_run<T: Task>() -> i32 {
    let result0 = T::run();
    wit_bindgen_guest_rust::rt::as_i32(result0)
}

#[doc(hidden)]
pub unsafe fn call_dispose<T: Task>() -> i32 {
    let result0 = T::dispose();
    wit_bindgen_guest_rust::rt::as_i32(result0)
}

/// Declares the export of the component's world for the
/// given type.
#[macro_export]
macro_rules! export_task(($t:ident) => {
  const _: () = {

    #[doc(hidden)]
    #[export_name = "init"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __export_task_init() -> i32 {
      call_init::<$t>()
    }

    #[doc(hidden)]
    #[export_name = "run"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __export_task_run() -> i32 {
      call_run::<$t>()
    }

    #[doc(hidden)]
    #[export_name = "dispose"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __export_task_dispose() -> i32 {
      call_dispose::<$t>()
    }

  };

  #[used]
  #[doc(hidden)]
  #[cfg(target_arch = "wasm32")]
  static __FORCE_SECTION_REF: fn() = __force_section_ref;
  #[doc(hidden)]
  #[cfg(target_arch = "wasm32")]
  fn __force_section_ref() {
    __link_section()
  }
});

// #[cfg(target_arch = "wasm32")]
#[link_section = "component-type:task"]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 209] = [
    2, 0, 4, 116, 97, 115, 107, 4, 116, 97, 115, 107, 4, 116, 97, 115, 107, 0, 97, 115, 109, 11, 0,
    1, 0, 7, 161, 1, 1, 65, 4, 1, 66, 2, 1, 64, 0, 1, 0, 4, 4, 116, 101, 115, 116, 0, 1, 0, 4, 10,
    104, 111, 115, 116, 45, 102, 117, 110, 99, 115, 20, 112, 107, 103, 58, 47, 116, 97, 115, 107,
    47, 104, 111, 115, 116, 45, 102, 117, 110, 99, 115, 5, 0, 1, 65, 6, 1, 66, 2, 1, 64, 0, 1, 0,
    4, 4, 116, 101, 115, 116, 0, 1, 0, 3, 4, 104, 111, 115, 116, 20, 112, 107, 103, 58, 47, 116,
    97, 115, 107, 47, 104, 111, 115, 116, 45, 102, 117, 110, 99, 115, 5, 0, 1, 64, 0, 0, 122, 4, 4,
    105, 110, 105, 116, 0, 1, 1, 4, 3, 114, 117, 110, 0, 1, 1, 4, 7, 100, 105, 115, 112, 111, 115,
    101, 0, 1, 1, 4, 4, 116, 97, 115, 107, 14, 112, 107, 103, 58, 47, 116, 97, 115, 107, 47, 116,
    97, 115, 107, 4, 1, 11, 18, 1, 4, 116, 97, 115, 107, 9, 112, 107, 103, 58, 47, 116, 97, 115,
    107, 3, 0,
];

#[inline(never)]
#[doc(hidden)]
// #[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
