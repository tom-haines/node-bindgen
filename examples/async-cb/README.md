simple hello world callback

```rs
#[node_bindgen]
async fn hello<F: Fn(f64,String)>( seconds: i32, cb: F) {

    println!("sleeping");
    sleep(Duration::from_secs(seconds as u64)).await;
    println!("woke from time");

    cb(10.0,"hello world".to_string());

}
```

```
async fn hello<F: Fn(f64,String)>( seconds: i32, cb: F) {
```
becomes
```
extern "C" fn napi_hello(
    env: node_bindgen::sys::napi_env,
    cb_info: node_bindgen::sys::napi_callback_info,
) -> node_bindgen::sys::napi_value {
    ....
}
```

proc macro expansion to:

```rs
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use std::time::Duration;
use flv_future_aio::timer::sleep;
use node_bindgen::derive::node_bindgen;
extern "C" fn napi_hello(
    env: node_bindgen::sys::napi_env,
    cb_info: node_bindgen::sys::napi_callback_info,
) -> node_bindgen::sys::napi_value {
    use node_bindgen::core::TryIntoJs;
    use node_bindgen::core::IntoJs;
    use node_bindgen::core::val::JsCallbackFunction;
    async fn hello<F: Fn(f64, String)>(seconds: i32, cb: F) {
        sleep(Duration::from_secs(seconds as u64)).await;
        cb(10.0, "hello world".to_string());
    }
    struct Argcb {
        arg0: f64,
        arg1: String,
    }
    extern "C" fn thread_safe_cb_complete(
        env: node_bindgen::sys::napi_env,
        js_cb: node_bindgen::sys::napi_value,
        _context: *mut ::std::os::raw::c_void,
        data: *mut ::std::os::raw::c_void,
    ) {
        if env != std::ptr::null_mut() {
            let js_env = node_bindgen::core::val::JsEnv::new(env);
            let result: Result<(), node_bindgen::core::NjError> = (move || {
                let global = js_env.get_global()?;
                let my_val: Box<Argcb> = unsafe { Box::from_raw(data as *mut Argcb) };
                let js_arg0 = my_val.arg0.try_to_js(&js_env)?;
                let js_arg1 = my_val.arg1.try_to_js(&js_env)?;
                js_env.call_function(global, js_cb, <[_]>::into_vec(box [js_arg0, js_arg1]))?;
                Ok(())
            })();
            match result {
                Ok(val) => val,
                Err(err) => ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &["napi call failed: "],
                    &match (&err,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                )),
            }
        }
    }
    let js_env = node_bindgen::core::val::JsEnv::new(env);
    let result: Result<node_bindgen::sys::napi_value, node_bindgen::core::NjError> = (move || {
        let js_cb = js_env.get_cb_info(cb_info, 2)?;
        let rust_value_0 = js_cb.get_value::<i32>(0)?;
        let rust_value_1 =
            js_cb.create_thread_safe_function("hello_sf", 1usize, Some(thread_safe_cb_complete))?;
        node_bindgen::core::future::spawn(async move {
            hello(rust_value_0, move |cb_arg0: f64, cb_arg1: String| {
                let arg = Argcb {
                    arg0: cb_arg0,
                    arg1: cb_arg1,
                };
                let my_box = Box::new(arg);
                let ptr = Box::into_raw(my_box);
                rust_value_1
                    .call(Some(ptr as *mut core::ffi::c_void))
                    .expect("callback should work");
            })
            .await;
        });
        Ok(std::ptr::null_mut())
    })();
    // return node_bindgen::sys::napi_value (bindgen created struct)
    result.to_js(&js_env)
}

#[used]
#[allow(non_upper_case_globals)]
// regiter the constructor for dyld linker to call on library load (__mod_init_func)
#[link_section = "__DATA,__mod_init_func"] 
static register_napi_hello: extern "C" fn() = {
    extern "C" fn register_napi_hello() {
        let property = node_bindgen::core::Property::new("hello").method(napi_hello);
        node_bindgen::core::submit_property(property);
    }
    register_napi_hello
};
```
