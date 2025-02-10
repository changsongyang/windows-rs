#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    IEnumVARIANT,
    IEnumVARIANT_Vtbl,
    0x00020404_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IEnumVARIANT, windows_core::IUnknown);
impl IEnumVARIANT {
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Skip)(
                windows_core::Interface::as_raw(self),
                celt,
            )
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IEnumVARIANT_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IEnumVARIANT_Impl: windows_core::IUnknownImpl {
    fn Skip(&self, celt: u32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumVARIANT>;
}
impl IEnumVARIANT_Vtbl {
    pub const fn new<Identity: IEnumVARIANT_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Skip<Identity: IEnumVARIANT_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumVARIANT_Impl::Skip(this, core::mem::transmute_copy(&celt))
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumVARIANT_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumVARIANT_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumVARIANT_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumVARIANT_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: 0,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumVARIANT as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumVARIANT {}
