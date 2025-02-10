#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADVANCED_FEATURE_FLAGS(pub u16);
impl ADVANCED_FEATURE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ADVANCED_FEATURE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ADVANCED_FEATURE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ADVANCED_FEATURE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ADVANCED_FEATURE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ADVANCED_FEATURE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: *mut u8,
}
impl Default for BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BSTRBLOB {
    pub cbSize: u32,
    pub pData: *mut u8,
}
impl Default for BSTRBLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CABOOL {
    pub cElems: u32,
    pub pElems: *mut VARIANT_BOOL,
}
impl Default for CABOOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CABSTR {
    pub cElems: u32,
    pub pElems: *mut windows_core::BSTR,
}
impl Default for CABSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CABSTRBLOB {
    pub cElems: u32,
    pub pElems: *mut BSTRBLOB,
}
impl Default for CABSTRBLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAC {
    pub cElems: u32,
    pub pElems: windows_core::PSTR,
}
impl Default for CAC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CACLIPDATA {
    pub cElems: u32,
    pub pElems: *mut CLIPDATA,
}
impl Default for CACLIPDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CACLSID {
    pub cElems: u32,
    pub pElems: *mut windows_core::GUID,
}
impl Default for CACLSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CACY {
    pub cElems: u32,
    pub pElems: *mut CY,
}
impl Default for CACY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CADATE {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl Default for CADATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CADBL {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl Default for CADBL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFILETIME {
    pub cElems: u32,
    pub pElems: *mut FILETIME,
}
impl Default for CAFILETIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFLT {
    pub cElems: u32,
    pub pElems: *mut f32,
}
impl Default for CAFLT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAH {
    pub cElems: u32,
    pub pElems: *mut i64,
}
impl Default for CAH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAI {
    pub cElems: u32,
    pub pElems: *mut i16,
}
impl Default for CAI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAL {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl Default for CAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut windows_core::PSTR,
}
impl Default for CALPSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut windows_core::PWSTR,
}
impl Default for CALPWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAPROPVARIANT {
    pub cElems: u32,
    pub pElems: *mut PROPVARIANT,
}
impl Default for CAPROPVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CASCODE {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl Default for CASCODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAUB {
    pub cElems: u32,
    pub pElems: *mut u8,
}
impl Default for CAUB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAUH {
    pub cElems: u32,
    pub pElems: *mut u64,
}
impl Default for CAUH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAUI {
    pub cElems: u32,
    pub pElems: *mut u16,
}
impl Default for CAUI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAUL {
    pub cElems: u32,
    pub pElems: *mut u32,
}
impl Default for CAUL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLIPDATA {
    pub cbSize: u32,
    pub ulClipFmt: i32,
    pub pClipData: *mut u8,
}
impl Default for CLIPDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLSCTX(pub u32);
impl CLSCTX {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CLSCTX {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CLSCTX {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CLSCTX {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CLSCTX {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CLSCTX {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl Default for CY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
impl Default for CY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DECIMAL {
    pub wReserved: u16,
    pub Anonymous1: DECIMAL_0,
    pub Hi32: u32,
    pub Anonymous2: DECIMAL_1,
}
impl Default for DECIMAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DECIMAL_0 {
    pub Anonymous: DECIMAL_0_0,
    pub signscale: u16,
}
impl Default for DECIMAL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DECIMAL_0_0 {
    pub scale: u8,
    pub sign: u8,
}
impl Default for DECIMAL_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DECIMAL_1 {
    pub Anonymous: DECIMAL_1_0,
    pub Lo64: u64,
}
impl Default for DECIMAL_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DECIMAL_1_0 {
    pub Lo32: u32,
    pub Mid32: u32,
}
impl Default for DECIMAL_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_STATE(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
impl Default for FILETIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(
    IDispatch,
    IDispatch_Vtbl,
    0x00020400_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IDispatch, windows_core::IUnknown);
impl IDispatch {
    pub unsafe fn GetTypeInfoCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfoCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetIDsOfNames(
        &self,
        riid: *const windows_core::GUID,
        rgsznames: *const windows_core::PCWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetIDsOfNames)(
                windows_core::Interface::as_raw(self),
                riid,
                rgsznames,
                cnames,
                lcid,
                rgdispid as _,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IDispatch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTypeInfoCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    GetTypeInfo: usize,
    pub GetIDsOfNames: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const windows_core::PCWSTR,
        u32,
        u32,
        *mut i32,
    ) -> windows_core::HRESULT,
    Invoke: usize,
}
pub trait IDispatch_Impl: windows_core::IUnknownImpl {
    fn GetTypeInfoCount(&self) -> windows_core::Result<u32>;
    fn GetIDsOfNames(
        &self,
        riid: *const windows_core::GUID,
        rgsznames: *const windows_core::PCWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> windows_core::Result<()>;
}
impl IDispatch_Vtbl {
    pub const fn new<Identity: IDispatch_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTypeInfoCount<
            Identity: IDispatch_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pctinfo: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispatch_Impl::GetTypeInfoCount(this) {
                    Ok(ok__) => {
                        pctinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIDsOfNames<Identity: IDispatch_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            rgsznames: *const windows_core::PCWSTR,
            cnames: u32,
            lcid: u32,
            rgdispid: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispatch_Impl::GetIDsOfNames(
                    this,
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&rgsznames),
                    core::mem::transmute_copy(&cnames),
                    core::mem::transmute_copy(&lcid),
                    core::mem::transmute_copy(&rgdispid),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTypeInfoCount: GetTypeInfoCount::<Identity, OFFSET>,
            GetTypeInfo: 0,
            GetIDsOfNames: GetIDsOfNames::<Identity, OFFSET>,
            Invoke: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispatch as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDispatch {}
windows_core::imp::define_interface!(
    IMMDevice,
    IMMDevice_Vtbl,
    0xd666063f_1587_4e43_81f1_b948e807363f
);
windows_core::imp::interface_hierarchy!(IMMDevice, windows_core::IUnknown);
impl IMMDevice {
    pub unsafe fn Activate<T>(
        &self,
        dwclsctx: CLSCTX,
        pactivationparams: Option<*const PROPVARIANT>,
    ) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).Activate)(
                windows_core::Interface::as_raw(self),
                &T::IID,
                dwclsctx,
                pactivationparams.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OpenPropertyStore(
        &self,
        stgmaccess: STGM,
    ) -> windows_core::Result<IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenPropertyStore)(
                windows_core::Interface::as_raw(self),
                stgmaccess,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
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
    pub unsafe fn GetState(&self) -> windows_core::Result<DEVICE_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(
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
    pub Activate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        CLSCTX,
        *const PROPVARIANT,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OpenPropertyStore: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        STGM,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::PWSTR,
    ) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DEVICE_STATE,
    ) -> windows_core::HRESULT,
}
pub trait IMMDevice_Impl: windows_core::IUnknownImpl {
    fn Activate(
        &self,
        iid: *const windows_core::GUID,
        dwclsctx: CLSCTX,
        pactivationparams: *const PROPVARIANT,
        ppinterface: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn OpenPropertyStore(&self, stgmaccess: STGM) -> windows_core::Result<IPropertyStore>;
    fn GetId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetState(&self) -> windows_core::Result<DEVICE_STATE>;
}
impl IMMDevice_Vtbl {
    pub const fn new<Identity: IMMDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Activate<Identity: IMMDevice_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            iid: *const windows_core::GUID,
            dwclsctx: CLSCTX,
            pactivationparams: *const PROPVARIANT,
            ppinterface: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMDevice_Impl::Activate(
                    this,
                    core::mem::transmute_copy(&iid),
                    core::mem::transmute_copy(&dwclsctx),
                    core::mem::transmute_copy(&pactivationparams),
                    core::mem::transmute_copy(&ppinterface),
                )
                .into()
            }
        }
        unsafe extern "system" fn OpenPropertyStore<
            Identity: IMMDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            stgmaccess: STGM,
            ppproperties: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDevice_Impl::OpenPropertyStore(
                    this,
                    core::mem::transmute_copy(&stgmaccess),
                ) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
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
        unsafe extern "system" fn GetState<Identity: IMMDevice_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pdwstate: *mut DEVICE_STATE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDevice_Impl::GetState(this) {
                    Ok(ok__) => {
                        pdwstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, OFFSET>,
            OpenPropertyStore: OpenPropertyStore::<Identity, OFFSET>,
            GetId: GetId::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMDevice as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMMDevice {}
windows_core::imp::define_interface!(
    IPropertyStore,
    IPropertyStore_Vtbl,
    0x886d8eeb_8cf2_4446_8d02_cdba1dbdcf99
);
windows_core::imp::interface_hierarchy!(IPropertyStore, windows_core::IUnknown);
impl IPropertyStore {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
#[repr(C)]
pub struct IPropertyStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    GetAt: usize,
    GetValue: usize,
    SetValue: usize,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPropertyStore_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Commit(&self) -> windows_core::Result<()>;
}
impl IPropertyStore_Vtbl {
    pub const fn new<Identity: IPropertyStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPropertyStore_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            cprops: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyStore_Impl::GetCount(this) {
                    Ok(ok__) => {
                        cprops.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Commit<Identity: IPropertyStore_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStore_Impl::Commit(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: 0,
            GetValue: 0,
            SetValue: 0,
            Commit: Commit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyStore as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPropertyStore {}
windows_core::imp::define_interface!(
    ISequentialStream,
    ISequentialStream_Vtbl,
    0x0c733a30_2a1c_11ce_ade5_00aa0044773d
);
windows_core::imp::interface_hierarchy!(ISequentialStream, windows_core::IUnknown);
impl ISequentialStream {
    pub unsafe fn Read(
        &self,
        pv: *mut core::ffi::c_void,
        cb: u32,
        pcbread: Option<*mut u32>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Read)(
                windows_core::Interface::as_raw(self),
                pv as _,
                cb,
                pcbread.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn Write(
        &self,
        pv: *const core::ffi::c_void,
        cb: u32,
        pcbwritten: Option<*mut u32>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Write)(
                windows_core::Interface::as_raw(self),
                pv,
                cb,
                pcbwritten.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
}
#[repr(C)]
pub struct ISequentialStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Read: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
}
pub trait ISequentialStream_Impl: windows_core::IUnknownImpl {
    fn Read(&self, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32)
        -> windows_core::HRESULT;
    fn Write(
        &self,
        pv: *const core::ffi::c_void,
        cb: u32,
        pcbwritten: *mut u32,
    ) -> windows_core::HRESULT;
}
impl ISequentialStream_Vtbl {
    pub const fn new<Identity: ISequentialStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: ISequentialStream_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pv: *mut core::ffi::c_void,
            cb: u32,
            pcbread: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISequentialStream_Impl::Read(
                    this,
                    core::mem::transmute_copy(&pv),
                    core::mem::transmute_copy(&cb),
                    core::mem::transmute_copy(&pcbread),
                )
            }
        }
        unsafe extern "system" fn Write<Identity: ISequentialStream_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pv: *const core::ffi::c_void,
            cb: u32,
            pcbwritten: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISequentialStream_Impl::Write(
                    this,
                    core::mem::transmute_copy(&pv),
                    core::mem::transmute_copy(&cb),
                    core::mem::transmute_copy(&pcbwritten),
                )
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Read: Read::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISequentialStream as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISequentialStream {}
windows_core::imp::define_interface!(
    IStorage,
    IStorage_Vtbl,
    0x0000000b_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IStorage, windows_core::IUnknown);
impl IStorage {
    pub unsafe fn CreateStream<P0>(
        &self,
        pwcsname: P0,
        grfmode: STGM,
        reserved1: u32,
        reserved2: u32,
    ) -> windows_core::Result<IStream>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStream)(
                windows_core::Interface::as_raw(self),
                pwcsname.param().abi(),
                grfmode,
                reserved1,
                reserved2,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OpenStream<P0>(
        &self,
        pwcsname: P0,
        reserved1: Option<*const core::ffi::c_void>,
        grfmode: STGM,
        reserved2: u32,
    ) -> windows_core::Result<IStream>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenStream)(
                windows_core::Interface::as_raw(self),
                pwcsname.param().abi(),
                reserved1.unwrap_or(core::mem::zeroed()) as _,
                grfmode,
                reserved2,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateStorage<P0>(
        &self,
        pwcsname: P0,
        grfmode: STGM,
        reserved1: u32,
        reserved2: u32,
    ) -> windows_core::Result<IStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStorage)(
                windows_core::Interface::as_raw(self),
                pwcsname.param().abi(),
                grfmode,
                reserved1,
                reserved2,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OpenStorage<P0, P1>(
        &self,
        pwcsname: P0,
        pstgpriority: P1,
        grfmode: STGM,
        snbexclude: *const *const u16,
        reserved: u32,
    ) -> windows_core::Result<IStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IStorage>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenStorage)(
                windows_core::Interface::as_raw(self),
                pwcsname.param().abi(),
                pstgpriority.param().abi(),
                grfmode,
                snbexclude,
                reserved,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CopyTo<P3>(
        &self,
        rgiidexclude: Option<&[windows_core::GUID]>,
        snbexclude: Option<*const *const u16>,
        pstgdest: P3,
    ) -> windows_core::Result<()>
    where
        P3: windows_core::Param<IStorage>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopyTo)(
                windows_core::Interface::as_raw(self),
                rgiidexclude
                    .as_deref()
                    .map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    rgiidexclude
                        .as_deref()
                        .map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
                snbexclude.unwrap_or(core::mem::zeroed()) as _,
                pstgdest.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Commit)(
                windows_core::Interface::as_raw(self),
                grfcommitflags,
            )
            .ok()
        }
    }
    pub unsafe fn Revert(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Revert)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub unsafe fn DestroyElement<P0>(&self, pwcsname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DestroyElement)(
                windows_core::Interface::as_raw(self),
                pwcsname.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn RenameElement<P0, P1>(
        &self,
        pwcsoldname: P0,
        pwcsnewname: P1,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RenameElement)(
                windows_core::Interface::as_raw(self),
                pwcsoldname.param().abi(),
                pwcsnewname.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn SetElementTimes<P0>(
        &self,
        pwcsname: P0,
        pctime: *const FILETIME,
        patime: *const FILETIME,
        pmtime: *const FILETIME,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetElementTimes)(
                windows_core::Interface::as_raw(self),
                pwcsname.param().abi(),
                pctime,
                patime,
                pmtime,
            )
            .ok()
        }
    }
    pub unsafe fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetClass)(
                windows_core::Interface::as_raw(self),
                clsid,
            )
            .ok()
        }
    }
    pub unsafe fn SetStateBits(&self, grfstatebits: u32, grfmask: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetStateBits)(
                windows_core::Interface::as_raw(self),
                grfstatebits,
                grfmask,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateStream: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        STGM,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OpenStream: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *const core::ffi::c_void,
        STGM,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateStorage: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        STGM,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OpenStorage: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut core::ffi::c_void,
        STGM,
        *const *const u16,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const windows_core::GUID,
        *const *const u16,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    MoveElementTo: usize,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    EnumElements: usize,
    pub DestroyElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub RenameElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub SetElementTimes: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *const FILETIME,
        *const FILETIME,
        *const FILETIME,
    ) -> windows_core::HRESULT,
    pub SetClass: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub SetStateBits:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    Stat: usize,
}
pub trait IStorage_Impl: windows_core::IUnknownImpl {
    fn CreateStream(
        &self,
        pwcsname: &windows_core::PCWSTR,
        grfmode: STGM,
        reserved1: u32,
        reserved2: u32,
    ) -> windows_core::Result<IStream>;
    fn OpenStream(
        &self,
        pwcsname: &windows_core::PCWSTR,
        reserved1: *const core::ffi::c_void,
        grfmode: STGM,
        reserved2: u32,
    ) -> windows_core::Result<IStream>;
    fn CreateStorage(
        &self,
        pwcsname: &windows_core::PCWSTR,
        grfmode: STGM,
        reserved1: u32,
        reserved2: u32,
    ) -> windows_core::Result<IStorage>;
    fn OpenStorage(
        &self,
        pwcsname: &windows_core::PCWSTR,
        pstgpriority: windows_core::Ref<'_, IStorage>,
        grfmode: STGM,
        snbexclude: *const *const u16,
        reserved: u32,
    ) -> windows_core::Result<IStorage>;
    fn CopyTo(
        &self,
        ciidexclude: u32,
        rgiidexclude: *const windows_core::GUID,
        snbexclude: *const *const u16,
        pstgdest: windows_core::Ref<'_, IStorage>,
    ) -> windows_core::Result<()>;
    fn Commit(&self, grfcommitflags: u32) -> windows_core::Result<()>;
    fn Revert(&self) -> windows_core::Result<()>;
    fn DestroyElement(&self, pwcsname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RenameElement(
        &self,
        pwcsoldname: &windows_core::PCWSTR,
        pwcsnewname: &windows_core::PCWSTR,
    ) -> windows_core::Result<()>;
    fn SetElementTimes(
        &self,
        pwcsname: &windows_core::PCWSTR,
        pctime: *const FILETIME,
        patime: *const FILETIME,
        pmtime: *const FILETIME,
    ) -> windows_core::Result<()>;
    fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetStateBits(&self, grfstatebits: u32, grfmask: u32) -> windows_core::Result<()>;
}
impl IStorage_Vtbl {
    pub const fn new<Identity: IStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateStream<Identity: IStorage_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pwcsname: windows_core::PCWSTR,
            grfmode: STGM,
            reserved1: u32,
            reserved2: u32,
            ppstm: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorage_Impl::CreateStream(
                    this,
                    core::mem::transmute(&pwcsname),
                    core::mem::transmute_copy(&grfmode),
                    core::mem::transmute_copy(&reserved1),
                    core::mem::transmute_copy(&reserved2),
                ) {
                    Ok(ok__) => {
                        ppstm.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenStream<Identity: IStorage_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pwcsname: windows_core::PCWSTR,
            reserved1: *const core::ffi::c_void,
            grfmode: STGM,
            reserved2: u32,
            ppstm: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorage_Impl::OpenStream(
                    this,
                    core::mem::transmute(&pwcsname),
                    core::mem::transmute_copy(&reserved1),
                    core::mem::transmute_copy(&grfmode),
                    core::mem::transmute_copy(&reserved2),
                ) {
                    Ok(ok__) => {
                        ppstm.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateStorage<Identity: IStorage_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pwcsname: windows_core::PCWSTR,
            grfmode: STGM,
            reserved1: u32,
            reserved2: u32,
            ppstg: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorage_Impl::CreateStorage(
                    this,
                    core::mem::transmute(&pwcsname),
                    core::mem::transmute_copy(&grfmode),
                    core::mem::transmute_copy(&reserved1),
                    core::mem::transmute_copy(&reserved2),
                ) {
                    Ok(ok__) => {
                        ppstg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenStorage<Identity: IStorage_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pwcsname: windows_core::PCWSTR,
            pstgpriority: *mut core::ffi::c_void,
            grfmode: STGM,
            snbexclude: *const *const u16,
            reserved: u32,
            ppstg: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorage_Impl::OpenStorage(
                    this,
                    core::mem::transmute(&pwcsname),
                    core::mem::transmute_copy(&pstgpriority),
                    core::mem::transmute_copy(&grfmode),
                    core::mem::transmute_copy(&snbexclude),
                    core::mem::transmute_copy(&reserved),
                ) {
                    Ok(ok__) => {
                        ppstg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CopyTo<Identity: IStorage_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ciidexclude: u32,
            rgiidexclude: *const windows_core::GUID,
            snbexclude: *const *const u16,
            pstgdest: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::CopyTo(
                    this,
                    core::mem::transmute_copy(&ciidexclude),
                    core::mem::transmute_copy(&rgiidexclude),
                    core::mem::transmute_copy(&snbexclude),
                    core::mem::transmute_copy(&pstgdest),
                )
                .into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IStorage_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            grfcommitflags: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::Commit(this, core::mem::transmute_copy(&grfcommitflags)).into()
            }
        }
        unsafe extern "system" fn Revert<Identity: IStorage_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::Revert(this).into()
            }
        }
        unsafe extern "system" fn DestroyElement<Identity: IStorage_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pwcsname: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::DestroyElement(this, core::mem::transmute(&pwcsname)).into()
            }
        }
        unsafe extern "system" fn RenameElement<Identity: IStorage_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pwcsoldname: windows_core::PCWSTR,
            pwcsnewname: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::RenameElement(
                    this,
                    core::mem::transmute(&pwcsoldname),
                    core::mem::transmute(&pwcsnewname),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetElementTimes<Identity: IStorage_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pwcsname: windows_core::PCWSTR,
            pctime: *const FILETIME,
            patime: *const FILETIME,
            pmtime: *const FILETIME,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::SetElementTimes(
                    this,
                    core::mem::transmute(&pwcsname),
                    core::mem::transmute_copy(&pctime),
                    core::mem::transmute_copy(&patime),
                    core::mem::transmute_copy(&pmtime),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetClass<Identity: IStorage_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            clsid: *const windows_core::GUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::SetClass(this, core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn SetStateBits<Identity: IStorage_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            grfstatebits: u32,
            grfmask: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::SetStateBits(
                    this,
                    core::mem::transmute_copy(&grfstatebits),
                    core::mem::transmute_copy(&grfmask),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateStream: CreateStream::<Identity, OFFSET>,
            OpenStream: OpenStream::<Identity, OFFSET>,
            CreateStorage: CreateStorage::<Identity, OFFSET>,
            OpenStorage: OpenStorage::<Identity, OFFSET>,
            CopyTo: CopyTo::<Identity, OFFSET>,
            MoveElementTo: 0,
            Commit: Commit::<Identity, OFFSET>,
            Revert: Revert::<Identity, OFFSET>,
            EnumElements: 0,
            DestroyElement: DestroyElement::<Identity, OFFSET>,
            RenameElement: RenameElement::<Identity, OFFSET>,
            SetElementTimes: SetElementTimes::<Identity, OFFSET>,
            SetClass: SetClass::<Identity, OFFSET>,
            SetStateBits: SetStateBits::<Identity, OFFSET>,
            Stat: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IStorage {}
windows_core::imp::define_interface!(
    IStream,
    IStream_Vtbl,
    0x0000000c_0000_0000_c000_000000000046
);
impl core::ops::Deref for IStream {
    type Target = ISequentialStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStream, windows_core::IUnknown, ISequentialStream);
impl IStream {
    pub unsafe fn SetSize(&self, libnewsize: u64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSize)(
                windows_core::Interface::as_raw(self),
                libnewsize,
            )
            .ok()
        }
    }
    pub unsafe fn CopyTo<P0>(
        &self,
        pstm: P0,
        cb: u64,
        pcbread: Option<*mut u64>,
        pcbwritten: Option<*mut u64>,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStream>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopyTo)(
                windows_core::Interface::as_raw(self),
                pstm.param().abi(),
                cb,
                pcbread.unwrap_or(core::mem::zeroed()) as _,
                pcbwritten.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn Revert(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Revert)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub unsafe fn UnlockRegion(
        &self,
        liboffset: u64,
        cb: u64,
        dwlocktype: u32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).UnlockRegion)(
                windows_core::Interface::as_raw(self),
                liboffset,
                cb,
                dwlocktype,
            )
            .ok()
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IStream> {
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
pub struct IStream_Vtbl {
    pub base__: ISequentialStream_Vtbl,
    Seek: usize,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u64,
        *mut u64,
        *mut u64,
    ) -> windows_core::HRESULT,
    Commit: usize,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    LockRegion: usize,
    pub UnlockRegion:
        unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32) -> windows_core::HRESULT,
    Stat: usize,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IStream_Impl: ISequentialStream_Impl {
    fn SetSize(&self, libnewsize: u64) -> windows_core::Result<()>;
    fn CopyTo(
        &self,
        pstm: windows_core::Ref<'_, IStream>,
        cb: u64,
        pcbread: *mut u64,
        pcbwritten: *mut u64,
    ) -> windows_core::Result<()>;
    fn Revert(&self) -> windows_core::Result<()>;
    fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IStream>;
}
impl IStream_Vtbl {
    pub const fn new<Identity: IStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSize<Identity: IStream_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            libnewsize: u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::SetSize(this, core::mem::transmute_copy(&libnewsize)).into()
            }
        }
        unsafe extern "system" fn CopyTo<Identity: IStream_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pstm: *mut core::ffi::c_void,
            cb: u64,
            pcbread: *mut u64,
            pcbwritten: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::CopyTo(
                    this,
                    core::mem::transmute_copy(&pstm),
                    core::mem::transmute_copy(&cb),
                    core::mem::transmute_copy(&pcbread),
                    core::mem::transmute_copy(&pcbwritten),
                )
                .into()
            }
        }
        unsafe extern "system" fn Revert<Identity: IStream_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::Revert(this).into()
            }
        }
        unsafe extern "system" fn UnlockRegion<Identity: IStream_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            liboffset: u64,
            cb: u64,
            dwlocktype: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::UnlockRegion(
                    this,
                    core::mem::transmute_copy(&liboffset),
                    core::mem::transmute_copy(&cb),
                    core::mem::transmute_copy(&dwlocktype),
                )
                .into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IStream_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppstm: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStream_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppstm.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ISequentialStream_Vtbl::new::<Identity, OFFSET>(),
            Seek: 0,
            SetSize: SetSize::<Identity, OFFSET>,
            CopyTo: CopyTo::<Identity, OFFSET>,
            Commit: 0,
            Revert: Revert::<Identity, OFFSET>,
            LockRegion: 0,
            UnlockRegion: UnlockRegion::<Identity, OFFSET>,
            Stat: 0,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStream as windows_core::Interface>::IID
            || iid == &<ISequentialStream as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IStream {}
#[repr(C)]
pub struct PROPVARIANT {
    pub Anonymous: PROPVARIANT_0,
}
impl Default for PROPVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union PROPVARIANT_0 {
    pub Anonymous: core::mem::ManuallyDrop<PROPVARIANT_0_0>,
    pub decVal: DECIMAL,
}
impl Clone for PROPVARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for PROPVARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PROPVARIANT_0_0 {
    pub vt: VARENUM,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: PROPVARIANT_0_0_0,
}
impl Clone for PROPVARIANT_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for PROPVARIANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union PROPVARIANT_0_0_0 {
    pub cVal: i8,
    pub bVal: u8,
    pub iVal: i16,
    pub uiVal: u16,
    pub lVal: i32,
    pub ulVal: u32,
    pub intVal: i32,
    pub uintVal: u32,
    pub hVal: i64,
    pub uhVal: u64,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: VARIANT_BOOL,
    pub scode: i32,
    pub cyVal: CY,
    pub date: f64,
    pub filetime: FILETIME,
    pub puuid: *mut windows_core::GUID,
    pub pclipdata: *mut CLIPDATA,
    pub bstrVal: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrblobVal: BSTRBLOB,
    pub blob: BLOB,
    pub pszVal: windows_core::PSTR,
    pub pwszVal: windows_core::PWSTR,
    pub punkVal: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub pdispVal: core::mem::ManuallyDrop<Option<IDispatch>>,
    pub pStream: core::mem::ManuallyDrop<Option<IStream>>,
    pub pStorage: core::mem::ManuallyDrop<Option<IStorage>>,
    pub pVersionedStream: *mut VERSIONEDSTREAM,
    pub parray: *mut SAFEARRAY,
    pub cac: CAC,
    pub caub: CAUB,
    pub cai: CAI,
    pub caui: CAUI,
    pub cal: CAL,
    pub caul: CAUL,
    pub cah: CAH,
    pub cauh: CAUH,
    pub caflt: CAFLT,
    pub cadbl: CADBL,
    pub cabool: CABOOL,
    pub cascode: CASCODE,
    pub cacy: CACY,
    pub cadate: CADATE,
    pub cafiletime: CAFILETIME,
    pub cauuid: CACLSID,
    pub caclipdata: CACLIPDATA,
    pub cabstr: CABSTR,
    pub cabstrblob: CABSTRBLOB,
    pub calpstr: CALPSTR,
    pub calpwstr: CALPWSTR,
    pub capropvar: CAPROPVARIANT,
    pub pcVal: windows_core::PSTR,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub puiVal: *mut u16,
    pub plVal: *mut i32,
    pub pulVal: *mut u32,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut VARIANT_BOOL,
    pub pdecVal: *mut DECIMAL,
    pub pscode: *mut i32,
    pub pcyVal: *mut CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut windows_core::BSTR,
    pub ppunkVal: *mut Option<windows_core::IUnknown>,
    pub ppdispVal: *mut Option<IDispatch>,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut PROPVARIANT,
}
impl Clone for PROPVARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for PROPVARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: ADVANCED_FEATURE_FLAGS,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
impl Default for SAFEARRAYBOUND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STGM(pub u32);
impl STGM {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for STGM {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for STGM {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for STGM {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for STGM {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for STGM {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VARENUM(pub u16);
impl VARENUM {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for VARENUM {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for VARENUM {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for VARENUM {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for VARENUM {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for VARENUM {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct VARIANT_BOOL(pub i16);
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct VERSIONEDSTREAM {
    pub guidVersion: windows_core::GUID,
    pub pStream: core::mem::ManuallyDrop<Option<IStream>>,
}
impl Default for VERSIONEDSTREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
