## sourcecode

```rs
use red4ext_rs::prelude::*;

define_plugin! {
    name: "example",
    author: "author",
    version: 0:1:0,
    on_register: {
        register_function!("CallNativeDemo", call_native_demo);
    }
}

/// define a binding for a native class type
#[derive(Clone, Default)]
#[repr(transparent)]
struct TimeSystem(Ref<IScriptable>);

unsafe impl RefRepr for TimeSystem {
    type Type = Strong;

    const CLASS_NAME: &'static str = "TimeSystem";
}

#[redscript_import]
impl TimeSystem {
    #[redscript(native)]
    fn get_game_time_stamp(&self) -> f32;

    #[redscript(native)]
    fn get_sim_time(&self) -> EngineTime;
}

fn call_native_demo(time: TimeSystem) {
    info!("current timestamp: {}", time.get_game_time_stamp());
    let sim = time.get_sim_time();
    info!("current engine time: {:#?}", sim);
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
struct EngineTime {
    pub unk00: [u8; 8],
}

unsafe impl NativeRepr for EngineTime {
    const NAME: &'static str = "EngineTime";
}
```

## cargo expand

```rs
  cargo expand -p red4ext-example
    Checking red4ext-example v0.3.3 (C:\Development\red4ext-rs\example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s

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
                                    call_native_demo,
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
                                        "CallNativeDemo",
                                        err,
                                    ),
                                )
                        });
                    }
                    ::std::pin::Pin::new_unchecked(&mut *frame).step();
                }
                let (arg_types, ret_type) = ::red4ext_rs::invocable::get_invocable_types(
                    &call_native_demo,
                );
                if let Err(err)
                    = ::red4ext_rs::rtti::Rtti::get()
                        .register_function(
                            "CallNativeDemo",
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
                                    "CallNativeDemo",
                                    err,
                                ),
                            )
                    });
                }
            };
        }
    }
}
/// define a binding for a native class type
#[repr(transparent)]
struct TimeSystem(Ref<IScriptable>);
#[automatically_derived]
impl ::core::clone::Clone for TimeSystem {
    #[inline]
    fn clone(&self) -> TimeSystem {
        TimeSystem(::core::clone::Clone::clone(&self.0))
    }
}
#[automatically_derived]
impl ::core::default::Default for TimeSystem {
    #[inline]
    fn default() -> TimeSystem {
        TimeSystem(::core::default::Default::default())
    }
}
unsafe impl RefRepr for TimeSystem {
    type Type = Strong;
    const CLASS_NAME: &'static str = "TimeSystem";
}
impl TimeSystem {
    fn get_game_time_stamp(&self) -> f32 {
        {
            let mut rtti = ::red4ext_rs::rtti::Rtti::get();
            let this = self.0.clone();
            match {
                let args = ();
                let args = ::red4ext_rs::invocable::Args::to_stack_args(&args);
                let res: ::std::result::Result<f32, _> = ::red4ext_rs::invocable::invoke(
                    this.clone().into_shared(),
                    ::red4ext_rs::rtti::Rtti::get_method(
                        this.into_shared(),
                        ::red4ext_rs::types::CName::new(unsafe {
                            ::std::str::from_utf8_unchecked(
                                &{
                                    let mut buf = [0u8; 0 + "GetGameTimeStamp".len()];
                                    let mut i = 0;
                                    buf[i..i + "GetGameTimeStamp".len()]
                                        .copy_from_slice("GetGameTimeStamp".as_bytes());
                                    i += "GetGameTimeStamp".len();
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
                                    let mut buf = [0u8; 0 + "GetGameTimeStamp".len()];
                                    let mut i = 0;
                                    buf[i..i + "GetGameTimeStamp".len()]
                                        .copy_from_slice("GetGameTimeStamp".as_bytes());
                                    i += "GetGameTimeStamp".len();
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
    fn get_sim_time(&self) -> EngineTime {
        {
            let mut rtti = ::red4ext_rs::rtti::Rtti::get();
            let this = self.0.clone();
            match {
                let args = ();
                let args = ::red4ext_rs::invocable::Args::to_stack_args(&args);
                let res: ::std::result::Result<EngineTime, _> = ::red4ext_rs::invocable::invoke(
                    this.clone().into_shared(),
                    ::red4ext_rs::rtti::Rtti::get_method(
                        this.into_shared(),
                        ::red4ext_rs::types::CName::new(unsafe {
                            ::std::str::from_utf8_unchecked(
                                &{
                                    let mut buf = [0u8; 0 + "GetSimTime".len()];
                                    let mut i = 0;
                                    buf[i..i + "GetSimTime".len()]
                                        .copy_from_slice("GetSimTime".as_bytes());
                                    i += "GetSimTime".len();
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
                                    let mut buf = [0u8; 0 + "GetSimTime".len()];
                                    let mut i = 0;
                                    buf[i..i + "GetSimTime".len()]
                                        .copy_from_slice("GetSimTime".as_bytes());
                                    i += "GetSimTime".len();
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
fn call_native_demo(time: TimeSystem) {
    ::red4ext_rs::logger::Logger::with_logger(|log| {
        log.info(format_args!("current timestamp: {0}", time.get_game_time_stamp()))
    });
    let sim = time.get_sim_time();
    ::red4ext_rs::logger::Logger::with_logger(|log| {
        log.info(format_args!("current engine time: {0:#?}", sim))
    });
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
```