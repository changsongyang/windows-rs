#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    IMMDevice,
    IMMDevice_Vtbl,
    0xd666063f_1587_4e43_81f1_b948e807363f
);
windows_core::imp::interface_hierarchy!(IMMDevice, windows_core::IUnknown);
impl IMMDevice {
    pub unsafe fn GetId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IMMDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    Activate: usize,
    OpenPropertyStore: usize,
    pub GetId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::PWSTR,
    ) -> windows_core::HRESULT,
    GetState: usize,
}
pub trait IMMDevice_Impl: windows_core::IUnknownImpl {
    fn GetId(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IMMDevice_Vtbl {
    pub const fn new<Identity: IMMDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetId<Identity: IMMDevice_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppstrid: *mut windows_core::PWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDevice_Impl::GetId(this) {
                    Ok(ok__) => {
                        ppstrid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: 0,
            OpenPropertyStore: 0,
            GetId: GetId::<Identity, OFFSET>,
            GetState: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMDevice as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMMDevice {}
