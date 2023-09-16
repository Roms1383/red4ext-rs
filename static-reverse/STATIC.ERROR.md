## sourcecode

```rs
use red4ext_rs::prelude::*;

define_plugin! {
    name: "example",
    author: "author",
    version: 0:1:0,
    on_register: {
        register_function!("CompareEngineTime", compare_engine_time);
    }
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
struct EngineTime {
    pub unk00: [u8; 8],
}

unsafe impl NativeRepr for EngineTime {
    const NAME: &'static str = "EngineTime";
}

#[redscript_import]
impl EngineTime {
    #[redscript(native)]
    fn to_float(time: Self) -> f32;

    #[redscript(native)]
    fn from_float(time: f32) -> Self;
}

fn compare_engine_time(sim: EngineTime, float: f32) {
    info!("compare engine time: {:#?}\n{}", sim, float);
}
```

## cargo expand

```rs
$ cargo expand -p red4ext-example
    Checking red4ext-example v0.3.3 (C:\Development\red4ext-rs\example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use red4ext_rs::prelude::*;
mod __raw_api {
    use super::*;
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe extern "C" fn Main(
        handle: ::red4ext_rs::plugin::PluginHandle,
        reason: ::red4ext_rs::plugin::MainReason,
        sdk: &'static ::red4ext_rs::plugin::Sdk,
    ) {
        match reason {
            ::red4ext_rs::plugin::MainReason::Load => {
                ::red4ext_rs::logger::Logger::init(sdk, handle).ok();
                ::red4ext_rs::ffi::add_rtti_callback(
                    ::red4ext_rs::types::VoidPtr(Register as *mut _),
                    ::red4ext_rs::types::VoidPtr(PostRegister as *mut _),
                    true,
                )
            }
            _ => {}
        }
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe extern "C" fn Query(info: *mut ::red4ext_rs::ffi::PluginInfo) {
        ::red4ext_rs::ffi::define_plugin(
            info as _,
            (&[101u16, 120u16, 97u16, 109u16, 112u16, 108u16, 101u16, 0u16]).as_ptr(),
            (&[97u16, 117u16, 116u16, 104u16, 111u16, 114u16, 0u16]).as_ptr(),
            0,
            1,
            0,
        );
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    extern "C" fn Supports() -> u32 {
        ::red4ext_rs::ffi::get_sdk_version()
    }
    #[allow(non_snake_case)]
    extern "C" fn Register() {}
    #[allow(non_snake_case)]
    extern "C" fn PostRegister() {
        {
            {
                unsafe extern "C" fn native_impl(
                    ctx: *mut ::red4ext_rs::ffi::IScriptable,
                    frame: *mut ::red4ext_rs::ffi::CStackFrame,
                    ret: *mut ::std::ffi::c_void,
                    _unk: i64,
                ) {
                    #[cfg(debug_assertions)]
                    if let Some(err)
                        = ::std::panic::catch_unwind(|| {
                                ::red4ext_rs::invocable::Invocable::invoke(
                                    compare_engine_time,
                                    ctx,
                                    frame,
                                    ret,
                                )
                            })
                            .err()
                            .and_then(|err| err.downcast::<::std::string::String>().ok())
                    {
                        ::red4ext_rs::logger::Logger::with_logger(|log| {
                            log
                                .error(
                                    format_args!(
                                        "Function \'{0}\' has panicked: {1}",
                                        "CompareEngineTime",
                                        err,
                                    ),
                                )
                        });
                    }
                    ::std::pin::Pin::new_unchecked(&mut *frame).step();
                }
                let (arg_types, ret_type) = ::red4ext_rs::invocable::get_invocable_types(
                    &compare_engine_time,
                );
                if let Err(err)
                    = ::red4ext_rs::rtti::Rtti::get()
                        .register_function(
                            "CompareEngineTime",
                            native_impl,
                            arg_types,
                            ret_type,
                        )
                {
                    ::red4ext_rs::logger::Logger::with_logger(|log| {
                        log
                            .warn(
                                format_args!(
                                    "Registering \'{0}\' has partially failed: {1}",
                                    "CompareEngineTime",
                                    err,
                                ),
                            )
                    });
                }
            };
        }
    }
}
#[repr(C)]
struct EngineTime {
    pub unk00: [u8; 8],
}
#[automatically_derived]
impl ::core::fmt::Debug for EngineTime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "EngineTime",
            "unk00",
            &&self.unk00,
        )
    }
}
#[automatically_derived]
impl ::core::default::Default for EngineTime {
    #[inline]
    fn default() -> EngineTime {
        EngineTime {
            unk00: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for EngineTime {
    #[inline]
    fn clone(&self) -> EngineTime {
        EngineTime {
            unk00: ::core::clone::Clone::clone(&self.unk00),
        }
    }
}
unsafe impl NativeRepr for EngineTime {
    const NAME: &'static str = "EngineTime";
}
impl EngineTime {
    fn to_float(time: Self) -> f32 {
        {
            let mut rtti = ::red4ext_rs::rtti::Rtti::get();
            match {
                let args = (::red4ext_rs::invocable::into_type_and_repr(
                    &mut rtti,
                    time,
                ));
                let args = ::red4ext_rs::invocable::Args::to_stack_args(&args);
                let res: ::std::result::Result<f32, _> = ::red4ext_rs::invocable::invoke(
                    ::red4ext_rs::types::RefShared::null(),
                    rtti
                        .get_function(
                            ::red4ext_rs::types::CName::new(unsafe {
                                ::std::str::from_utf8_unchecked(
                                    &{
                                        let mut buf = [0u8; 0 + "EngineTime".len() + "::".len()
                                            + "ToFloat".len()];
                                        let mut i = 0;
                                        buf[i..i + "EngineTime".len()]
                                            .copy_from_slice("EngineTime".as_bytes());
                                        i += "EngineTime".len();
                                        buf[i..i + "::".len()].copy_from_slice("::".as_bytes());
                                        i += "::".len();
                                        buf[i..i + "ToFloat".len()]
                                            .copy_from_slice("ToFloat".as_bytes());
                                        i += "ToFloat".len();
                                        buf
                                    },
                                )
                            }),
                        ),
                    &args,
                );
                res
            } {
                Ok(res) => res,
                Err(err) => {
                    ::red4ext_rs::invocable::raise_invoke_error(
                        unsafe {
                            ::std::str::from_utf8_unchecked(
                                &{
                                    let mut buf = [0u8; 0 + "EngineTime".len() + "::".len()
                                        + "ToFloat".len()];
                                    let mut i = 0;
                                    buf[i..i + "EngineTime".len()]
                                        .copy_from_slice("EngineTime".as_bytes());
                                    i += "EngineTime".len();
                                    buf[i..i + "::".len()].copy_from_slice("::".as_bytes());
                                    i += "::".len();
                                    buf[i..i + "ToFloat".len()]
                                        .copy_from_slice("ToFloat".as_bytes());
                                    i += "ToFloat".len();
                                    buf
                                },
                            )
                        },
                        err,
                    )
                }
            }
        }
    }
    fn from_float(time: f32) -> Self {
        {
            let mut rtti = ::red4ext_rs::rtti::Rtti::get();
            match {
                let args = (::red4ext_rs::invocable::into_type_and_repr(
                    &mut rtti,
                    time,
                ));
                let args = ::red4ext_rs::invocable::Args::to_stack_args(&args);
                let res: ::std::result::Result<Self, _> = ::red4ext_rs::invocable::invoke(
                    ::red4ext_rs::types::RefShared::null(),
                    rtti
                        .get_function(
                            ::red4ext_rs::types::CName::new(unsafe {
                                ::std::str::from_utf8_unchecked(
                                    &{
                                        let mut buf = [0u8; 0 + "EngineTime".len() + "::".len()
                                            + "FromFloat".len()];
                                        let mut i = 0;
                                        buf[i..i + "EngineTime".len()]
                                            .copy_from_slice("EngineTime".as_bytes());
                                        i += "EngineTime".len();
                                        buf[i..i + "::".len()].copy_from_slice("::".as_bytes());
                                        i += "::".len();
                                        buf[i..i + "FromFloat".len()]
                                            .copy_from_slice("FromFloat".as_bytes());
                                        i += "FromFloat".len();
                                        buf
                                    },
                                )
                            }),
                        ),
                    &args,
                );
                res
            } {
                Ok(res) => res,
                Err(err) => {
                    ::red4ext_rs::invocable::raise_invoke_error(
                        unsafe {
                            ::std::str::from_utf8_unchecked(
                                &{
                                    let mut buf = [0u8; 0 + "EngineTime".len() + "::".len()
                                        + "FromFloat".len()];
                                    let mut i = 0;
                                    buf[i..i + "EngineTime".len()]
                                        .copy_from_slice("EngineTime".as_bytes());
                                    i += "EngineTime".len();
                                    buf[i..i + "::".len()].copy_from_slice("::".as_bytes());
                                    i += "::".len();
                                    buf[i..i + "FromFloat".len()]
                                        .copy_from_slice("FromFloat".as_bytes());
                                    i += "FromFloat".len();
                                    buf
                                },
                            )
                        },
                        err,
                    )
                }
            }
        }
    }
}
fn compare_engine_time(sim: EngineTime, float: f32) {
    ::red4ext_rs::logger::Logger::with_logger(|log| {
        log.info(format_args!("compare engine time: {0:#?}\n{1}", sim, float))
    });
}
```