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
    IEnumVARIANT,
    IEnumVARIANT_Vtbl,
    0x00020404_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IEnumVARIANT, windows_core::IUnknown);
impl IEnumVARIANT {
    pub unsafe fn Next(
        &self,
        rgvar: &mut [VARIANT],
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                rgvar.len().try_into().unwrap(),
                core::mem::transmute(rgvar.as_ptr()),
                pceltfetched as _,
            )
        }
    }
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
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut VARIANT,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IEnumVARIANT_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgvar: *mut VARIANT, pceltfetched: *mut u32)
        -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumVARIANT>;
}
impl IEnumVARIANT_Vtbl {
    pub const fn new<Identity: IEnumVARIANT_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumVARIANT_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
            rgvar: *mut VARIANT,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumVARIANT_Impl::Next(
                    this,
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&rgvar),
                    core::mem::transmute_copy(&pceltfetched),
                )
            }
        }
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
            Next: Next::<Identity, OFFSET>,
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
windows_core::imp::define_interface!(
    IRecordInfo,
    IRecordInfo_Vtbl,
    0x0000002f_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IRecordInfo, windows_core::IUnknown);
impl IRecordInfo {
    pub unsafe fn RecordInit(&self, pvnew: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RecordInit)(
                windows_core::Interface::as_raw(self),
                pvnew as _,
            )
            .ok()
        }
    }
    pub unsafe fn RecordClear(
        &self,
        pvexisting: *const core::ffi::c_void,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RecordClear)(
                windows_core::Interface::as_raw(self),
                pvexisting,
            )
            .ok()
        }
    }
    pub unsafe fn RecordCopy(
        &self,
        pvexisting: *const core::ffi::c_void,
        pvnew: *mut core::ffi::c_void,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RecordCopy)(
                windows_core::Interface::as_raw(self),
                pvexisting,
                pvnew as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetGuid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGuid)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetField<P1>(
        &self,
        pvdata: *const core::ffi::c_void,
        szfieldname: P1,
    ) -> windows_core::Result<VARIANT>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetField)(
                windows_core::Interface::as_raw(self),
                pvdata,
                szfieldname.param().abi(),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetFieldNoCopy<P1>(
        &self,
        pvdata: *const core::ffi::c_void,
        szfieldname: P1,
        pvarfield: *mut VARIANT,
        ppvdatacarray: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetFieldNoCopy)(
                windows_core::Interface::as_raw(self),
                pvdata,
                szfieldname.param().abi(),
                core::mem::transmute(pvarfield),
                ppvdatacarray as _,
            )
            .ok()
        }
    }
    pub unsafe fn PutField<P2>(
        &self,
        wflags: u32,
        pvdata: *mut core::ffi::c_void,
        szfieldname: P2,
        pvarfield: *const VARIANT,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PutField)(
                windows_core::Interface::as_raw(self),
                wflags,
                pvdata as _,
                szfieldname.param().abi(),
                core::mem::transmute(pvarfield),
            )
            .ok()
        }
    }
    pub unsafe fn PutFieldNoCopy<P2>(
        &self,
        wflags: u32,
        pvdata: *mut core::ffi::c_void,
        szfieldname: P2,
        pvarfield: *const VARIANT,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PutFieldNoCopy)(
                windows_core::Interface::as_raw(self),
                wflags,
                pvdata as _,
                szfieldname.param().abi(),
                core::mem::transmute(pvarfield),
            )
            .ok()
        }
    }
    pub unsafe fn GetFieldNames(
        &self,
        pcnames: *mut u32,
        rgbstrnames: *mut windows_core::BSTR,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetFieldNames)(
                windows_core::Interface::as_raw(self),
                pcnames as _,
                core::mem::transmute(rgbstrnames),
            )
            .ok()
        }
    }
    pub unsafe fn IsMatchingType<P0>(&self, precordinfo: P0) -> windows_core::BOOL
    where
        P0: windows_core::Param<IRecordInfo>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).IsMatchingType)(
                windows_core::Interface::as_raw(self),
                precordinfo.param().abi(),
            )
        }
    }
    pub unsafe fn RecordCreate(&self) -> *mut core::ffi::c_void {
        unsafe {
            (windows_core::Interface::vtable(self).RecordCreate)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn RecordCreateCopy(
        &self,
        pvsource: *const core::ffi::c_void,
        ppvdest: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RecordCreateCopy)(
                windows_core::Interface::as_raw(self),
                pvsource,
                ppvdest as _,
            )
            .ok()
        }
    }
    pub unsafe fn RecordDestroy(
        &self,
        pvrecord: *const core::ffi::c_void,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RecordDestroy)(
                windows_core::Interface::as_raw(self),
                pvrecord,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IRecordInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RecordInit: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RecordClear: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RecordCopy: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetGuid: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    GetTypeInfo: usize,
    pub GetField: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        windows_core::PCWSTR,
        *mut VARIANT,
    ) -> windows_core::HRESULT,
    pub GetFieldNoCopy: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        windows_core::PCWSTR,
        *mut VARIANT,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PutField: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *const VARIANT,
    ) -> windows_core::HRESULT,
    pub PutFieldNoCopy: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *const VARIANT,
    ) -> windows_core::HRESULT,
    pub GetFieldNames: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsMatchingType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::BOOL,
    pub RecordCreate: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
    pub RecordCreateCopy: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RecordDestroy: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IRecordInfo_Impl: windows_core::IUnknownImpl {
    fn RecordInit(&self, pvnew: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RecordClear(&self, pvexisting: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn RecordCopy(
        &self,
        pvexisting: *const core::ffi::c_void,
        pvnew: *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn GetGuid(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSize(&self) -> windows_core::Result<u32>;
    fn GetField(
        &self,
        pvdata: *const core::ffi::c_void,
        szfieldname: &windows_core::PCWSTR,
    ) -> windows_core::Result<VARIANT>;
    fn GetFieldNoCopy(
        &self,
        pvdata: *const core::ffi::c_void,
        szfieldname: &windows_core::PCWSTR,
        pvarfield: *mut VARIANT,
        ppvdatacarray: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn PutField(
        &self,
        wflags: u32,
        pvdata: *mut core::ffi::c_void,
        szfieldname: &windows_core::PCWSTR,
        pvarfield: *const VARIANT,
    ) -> windows_core::Result<()>;
    fn PutFieldNoCopy(
        &self,
        wflags: u32,
        pvdata: *mut core::ffi::c_void,
        szfieldname: &windows_core::PCWSTR,
        pvarfield: *const VARIANT,
    ) -> windows_core::Result<()>;
    fn GetFieldNames(
        &self,
        pcnames: *mut u32,
        rgbstrnames: *mut windows_core::BSTR,
    ) -> windows_core::Result<()>;
    fn IsMatchingType(&self, precordinfo: windows_core::Ref<'_, IRecordInfo>)
        -> windows_core::BOOL;
    fn RecordCreate(&self) -> *mut core::ffi::c_void;
    fn RecordCreateCopy(
        &self,
        pvsource: *const core::ffi::c_void,
        ppvdest: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn RecordDestroy(&self, pvrecord: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl IRecordInfo_Vtbl {
    pub const fn new<Identity: IRecordInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RecordInit<Identity: IRecordInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pvnew: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::RecordInit(this, core::mem::transmute_copy(&pvnew)).into()
            }
        }
        unsafe extern "system" fn RecordClear<Identity: IRecordInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pvexisting: *const core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::RecordClear(this, core::mem::transmute_copy(&pvexisting)).into()
            }
        }
        unsafe extern "system" fn RecordCopy<Identity: IRecordInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pvexisting: *const core::ffi::c_void,
            pvnew: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::RecordCopy(
                    this,
                    core::mem::transmute_copy(&pvexisting),
                    core::mem::transmute_copy(&pvnew),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetGuid<Identity: IRecordInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pguid: *mut windows_core::GUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRecordInfo_Impl::GetGuid(this) {
                    Ok(ok__) => {
                        pguid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetName<Identity: IRecordInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pbstrname: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRecordInfo_Impl::GetName(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSize<Identity: IRecordInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pcbsize: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRecordInfo_Impl::GetSize(this) {
                    Ok(ok__) => {
                        pcbsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetField<Identity: IRecordInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pvdata: *const core::ffi::c_void,
            szfieldname: windows_core::PCWSTR,
            pvarfield: *mut VARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRecordInfo_Impl::GetField(
                    this,
                    core::mem::transmute_copy(&pvdata),
                    core::mem::transmute(&szfieldname),
                ) {
                    Ok(ok__) => {
                        pvarfield.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFieldNoCopy<
            Identity: IRecordInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvdata: *const core::ffi::c_void,
            szfieldname: windows_core::PCWSTR,
            pvarfield: *mut VARIANT,
            ppvdatacarray: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::GetFieldNoCopy(
                    this,
                    core::mem::transmute_copy(&pvdata),
                    core::mem::transmute(&szfieldname),
                    core::mem::transmute_copy(&pvarfield),
                    core::mem::transmute_copy(&ppvdatacarray),
                )
                .into()
            }
        }
        unsafe extern "system" fn PutField<Identity: IRecordInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            wflags: u32,
            pvdata: *mut core::ffi::c_void,
            szfieldname: windows_core::PCWSTR,
            pvarfield: *const VARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::PutField(
                    this,
                    core::mem::transmute_copy(&wflags),
                    core::mem::transmute_copy(&pvdata),
                    core::mem::transmute(&szfieldname),
                    core::mem::transmute_copy(&pvarfield),
                )
                .into()
            }
        }
        unsafe extern "system" fn PutFieldNoCopy<
            Identity: IRecordInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            wflags: u32,
            pvdata: *mut core::ffi::c_void,
            szfieldname: windows_core::PCWSTR,
            pvarfield: *const VARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::PutFieldNoCopy(
                    this,
                    core::mem::transmute_copy(&wflags),
                    core::mem::transmute_copy(&pvdata),
                    core::mem::transmute(&szfieldname),
                    core::mem::transmute_copy(&pvarfield),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetFieldNames<Identity: IRecordInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pcnames: *mut u32,
            rgbstrnames: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::GetFieldNames(
                    this,
                    core::mem::transmute_copy(&pcnames),
                    core::mem::transmute_copy(&rgbstrnames),
                )
                .into()
            }
        }
        unsafe extern "system" fn IsMatchingType<
            Identity: IRecordInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            precordinfo: *mut core::ffi::c_void,
        ) -> windows_core::BOOL {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::IsMatchingType(this, core::mem::transmute_copy(&precordinfo))
            }
        }
        unsafe extern "system" fn RecordCreate<Identity: IRecordInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::RecordCreate(this)
            }
        }
        unsafe extern "system" fn RecordCreateCopy<
            Identity: IRecordInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvsource: *const core::ffi::c_void,
            ppvdest: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::RecordCreateCopy(
                    this,
                    core::mem::transmute_copy(&pvsource),
                    core::mem::transmute_copy(&ppvdest),
                )
                .into()
            }
        }
        unsafe extern "system" fn RecordDestroy<Identity: IRecordInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pvrecord: *const core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRecordInfo_Impl::RecordDestroy(this, core::mem::transmute_copy(&pvrecord)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RecordInit: RecordInit::<Identity, OFFSET>,
            RecordClear: RecordClear::<Identity, OFFSET>,
            RecordCopy: RecordCopy::<Identity, OFFSET>,
            GetGuid: GetGuid::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            GetTypeInfo: 0,
            GetField: GetField::<Identity, OFFSET>,
            GetFieldNoCopy: GetFieldNoCopy::<Identity, OFFSET>,
            PutField: PutField::<Identity, OFFSET>,
            PutFieldNoCopy: PutFieldNoCopy::<Identity, OFFSET>,
            GetFieldNames: GetFieldNames::<Identity, OFFSET>,
            IsMatchingType: IsMatchingType::<Identity, OFFSET>,
            RecordCreate: RecordCreate::<Identity, OFFSET>,
            RecordCreateCopy: RecordCreateCopy::<Identity, OFFSET>,
            RecordDestroy: RecordDestroy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRecordInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRecordInfo {}
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
#[repr(C)]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
impl Default for VARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union VARIANT_0 {
    pub Anonymous: core::mem::ManuallyDrop<VARIANT_0_0>,
    pub decVal: DECIMAL,
}
impl Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for VARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct VARIANT_0_0 {
    pub vt: VARENUM,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
impl Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for VARIANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union VARIANT_0_0_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: VARIANT_BOOL,
    pub scode: i32,
    pub cyVal: CY,
    pub date: f64,
    pub bstrVal: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub punkVal: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub pdispVal: core::mem::ManuallyDrop<Option<IDispatch>>,
    pub parray: *mut SAFEARRAY,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_PBOOL: *mut VARIANT_BOOL,
    pub pscode: *mut i32,
    pub pcyVal: *mut CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut windows_core::BSTR,
    pub ppunkVal: *mut Option<windows_core::IUnknown>,
    pub ppdispVal: *mut Option<IDispatch>,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut VARIANT,
    pub byref: *mut core::ffi::c_void,
    pub cVal: i8,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub pdecVal: *mut DECIMAL,
    pub pcVal: windows_core::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub Anonymous: core::mem::ManuallyDrop<VARIANT_0_0_0_0>,
}
impl Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for VARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut core::ffi::c_void,
    pub pRecInfo: core::mem::ManuallyDrop<Option<IRecordInfo>>,
}
impl Default for VARIANT_0_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct VARIANT_BOOL(pub i16);
