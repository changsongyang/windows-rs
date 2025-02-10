#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    AsyncActionCompletedHandler,
    AsyncActionCompletedHandler_Vtbl,
    0xa4ed5c81_76c9_40bd_8be6_b1d90fb20ae7
);
impl windows_core::RuntimeType for AsyncActionCompletedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl AsyncActionCompletedHandler {
    pub fn new<
        F: FnMut(windows_core::Ref<'_, IAsyncAction>, AsyncStatus) -> windows_core::Result<()>
            + Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = AsyncActionCompletedHandlerBox {
            vtable: &AsyncActionCompletedHandlerBox::<F>::VTABLE,
            count: windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, asyncinfo: P0, asyncstatus: AsyncStatus) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAsyncAction>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Invoke)(
                windows_core::Interface::as_raw(this),
                asyncinfo.param().abi(),
                asyncstatus,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct AsyncActionCompletedHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        asyncinfo: *mut core::ffi::c_void,
        asyncstatus: AsyncStatus,
    ) -> windows_core::HRESULT,
}
#[repr(C)]
struct AsyncActionCompletedHandlerBox<
    F: FnMut(windows_core::Ref<'_, IAsyncAction>, AsyncStatus) -> windows_core::Result<()>
        + Send
        + 'static,
> {
    vtable: *const AsyncActionCompletedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<
        F: FnMut(windows_core::Ref<'_, IAsyncAction>, AsyncStatus) -> windows_core::Result<()>
            + Send
            + 'static,
    > AsyncActionCompletedHandlerBox<F>
{
    const VTABLE: AsyncActionCompletedHandler_Vtbl = AsyncActionCompletedHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut core::ffi::c_void,
        iid: *const windows_core::GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <AsyncActionCompletedHandler as windows_core::Interface>::IID
                || *iid == <windows_core::IUnknown as windows_core::Interface>::IID
                || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID
            {
                &mut (*this).vtable as *mut _ as _
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        asyncinfo: *mut core::ffi::c_void,
        asyncstatus: AsyncStatus,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&asyncinfo), asyncstatus).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AsyncStatus(pub i32);
impl AsyncStatus {
    pub const Canceled: Self = Self(2i32);
    pub const Completed: Self = Self(1i32);
    pub const Error: Self = Self(3i32);
    pub const Started: Self = Self(0i32);
}
impl windows_core::TypeKind for AsyncStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AsyncStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.AsyncStatus;i4)");
}
windows_core::imp::define_interface!(
    IAsyncAction,
    IAsyncAction_Vtbl,
    0x5a648006_843a_4da9_865b_9d26e5dfad7b
);
impl windows_core::RuntimeType for IAsyncAction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IAsyncAction,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(IAsyncAction, IAsyncInfo);
impl IAsyncAction {
    pub fn SetCompleted<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AsyncActionCompletedHandler>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetCompleted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn Completed(&self) -> windows_core::Result<AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetResults(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetResults)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<AsyncStatus> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
unsafe impl Send for IAsyncAction {}
unsafe impl Sync for IAsyncAction {}
impl windows_core::RuntimeName for IAsyncAction {
    const NAME: &'static str = "Windows.Foundation.IAsyncAction";
}
pub trait IAsyncAction_Impl: IAsyncInfo_Impl {
    fn SetCompleted(
        &self,
        handler: windows_core::Ref<'_, AsyncActionCompletedHandler>,
    ) -> windows_core::Result<()>;
    fn Completed(&self) -> windows_core::Result<AsyncActionCompletedHandler>;
    fn GetResults(&self) -> windows_core::Result<()>;
}
impl IAsyncAction_Vtbl {
    pub const fn new<Identity: IAsyncAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCompleted<Identity: IAsyncAction_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncAction_Impl::SetCompleted(this, core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn Completed<Identity: IAsyncAction_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncAction_Impl::Completed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResults<Identity: IAsyncAction_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncAction_Impl::GetResults(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAsyncAction, OFFSET>(),
            SetCompleted: SetCompleted::<Identity, OFFSET>,
            Completed: Completed::<Identity, OFFSET>,
            GetResults: GetResults::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncAction as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IAsyncAction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetCompleted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetResults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAsyncInfo,
    IAsyncInfo_Vtbl,
    0x00000036_0000_0000_c000_000000000046
);
impl windows_core::RuntimeType for IAsyncInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IAsyncInfo,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IAsyncInfo {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<AsyncStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
impl windows_core::RuntimeName for IAsyncInfo {
    const NAME: &'static str = "Windows.Foundation.IAsyncInfo";
}
pub trait IAsyncInfo_Impl: windows_core::IUnknownImpl {
    fn Id(&self) -> windows_core::Result<u32>;
    fn Status(&self) -> windows_core::Result<AsyncStatus>;
    fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl IAsyncInfo_Vtbl {
    pub const fn new<Identity: IAsyncInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: IAsyncInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncInfo_Impl::Id(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Status<Identity: IAsyncInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut AsyncStatus,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncInfo_Impl::Status(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ErrorCode<Identity: IAsyncInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut windows_core::HRESULT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncInfo_Impl::ErrorCode(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IAsyncInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncInfo_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IAsyncInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncInfo_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAsyncInfo, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            ErrorCode: ErrorCode::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncInfo as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IAsyncInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut AsyncStatus,
    ) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::HRESULT,
    ) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
