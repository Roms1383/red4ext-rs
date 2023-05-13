use std::pin::Pin;

use red4ext_sys::ffi;
use thiserror::Error;

use crate::invocable::RedFunction;
use crate::types::{CName, RefShared, VoidPtr};

#[derive(Debug, Error)]
#[error("failed to bind arguments: {}", .0.join(", "))]
pub struct FailedBindings(Vec<&'static str>);

pub struct Rtti<'a> {
    inner: Pin<&'a mut ffi::IRttiSystem>,
}

impl<'a> Rtti<'a> {
    #[inline]
    pub fn get() -> Self {
        Self {
            inner: unsafe { Pin::new_unchecked(&mut *ffi::get_rtti()) },
        }
    }

    #[inline]
    pub fn get_class(&mut self, name: CName) -> *const ffi::CClass {
        self.inner.as_mut().get_class(name)
    }

    #[inline]
    pub fn get_type(&mut self, name: CName) -> *mut ffi::CBaseRttiType {
        self.inner.as_mut().get_type(name)
    }

    #[inline]
    pub fn get_function(&mut self, fn_name: CName) -> *mut ffi::CBaseFunction {
        self.inner.as_mut().get_function(fn_name) as *mut _
    }

    pub fn get_method(
        this: RefShared<ffi::IScriptable>,
        fn_name: CName,
    ) -> *mut ffi::CBaseFunction {
        unsafe {
            let typ = Self::class_of(this);
            ffi::get_method(&*typ, &fn_name) as _
        }
    }

    pub fn get_static_method(&mut self, class: CName, fn_name: CName) -> *mut ffi::CBaseFunction {
        unsafe {
            let typ = self.get_class(class);
            ffi::get_method(&*typ, &fn_name) as _
        }
    }

    pub fn register_function(
        &mut self,
        name: &str,
        func: RedFunction,
        args: &[CName],
        ret: CName,
    ) -> Result<(), FailedBindings> {
        let mut failed = vec![];
        unsafe {
            let func = ffi::new_native_function(
                name,
                name,
                VoidPtr(func as *mut _),
                args,
                ret,
                &mut failed,
            );
            if !failed.is_empty() {
                let failed = failed
                    .into_iter()
                    .filter_map(|i| args.get(i))
                    .map(ffi::resolve_cname)
                    .collect();
                return Err(FailedBindings(failed));
            }
            self.inner.as_mut().register_function(func);
        }
        Ok(())
    }

    #[inline]
    pub fn class_of(this: RefShared<ffi::IScriptable>) -> *const ffi::CClass {
        unsafe { Pin::new_unchecked(&mut *this.as_ptr()).get_class() }
    }

    #[inline]
    pub fn type_name_of(typ: *const ffi::CBaseRttiType) -> CName {
        unsafe { (*typ).get_name() }
    }
}

#[macro_export]
macro_rules! register_function {
    ($name:literal,$fun:expr) => {{
        unsafe extern "C" fn native_impl(
            ctx: *mut $crate::ffi::IScriptable,
            frame: *mut $crate::ffi::CStackFrame,
            ret: *mut ::std::ffi::c_void,
            _unk: i64,
        ) {
            #[cfg(debug_assertions)]
            if let Some(err) = ::std::panic::catch_unwind(|| {
                $crate::invocable::Invocable::invoke($fun, ctx, frame, ret)
            })
            .err()
            .and_then(|err| err.downcast::<::std::string::String>().ok())
            {
                $crate::error!("Panic in {}: {err}", $name);
            }

            #[cfg(not(debug_assertions))]
            $crate::invocable::Invocable::invoke($fun, ctx, frame, ret);

            ::std::pin::Pin::new_unchecked(&mut *frame).step();
        }

        let (arg_types, ret_type) = $crate::invocable::get_invocable_types(&$fun);
        if let Err(err) =
            $crate::rtti::Rtti::get().register_function($name, native_impl, arg_types, ret_type)
        {
            $crate::error!("Failed to register function {}: {:?}", $name, err);
        }
    }};
}
