#[inline]
pub unsafe fn WinBioAcquireFocus() -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioAcquireFocus() -> windows_core::HRESULT);
    unsafe { WinBioAcquireFocus().ok() }
}
#[inline]
pub unsafe fn WinBioAsyncEnumBiometricUnits(frameworkhandle: u32, factor: u32) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioAsyncEnumBiometricUnits(frameworkhandle : u32, factor : u32) -> windows_core::HRESULT);
    unsafe { WinBioAsyncEnumBiometricUnits(frameworkhandle, factor).ok() }
}
#[inline]
pub unsafe fn WinBioAsyncEnumDatabases(frameworkhandle: u32, factor: u32) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioAsyncEnumDatabases(frameworkhandle : u32, factor : u32) -> windows_core::HRESULT);
    unsafe { WinBioAsyncEnumDatabases(frameworkhandle, factor).ok() }
}
#[inline]
pub unsafe fn WinBioAsyncEnumServiceProviders(frameworkhandle: u32, factor: u32) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioAsyncEnumServiceProviders(frameworkhandle : u32, factor : u32) -> windows_core::HRESULT);
    unsafe { WinBioAsyncEnumServiceProviders(frameworkhandle, factor).ok() }
}
#[inline]
pub unsafe fn WinBioAsyncMonitorFrameworkChanges(frameworkhandle: u32, changetypes: u32) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioAsyncMonitorFrameworkChanges(frameworkhandle : u32, changetypes : u32) -> windows_core::HRESULT);
    unsafe { WinBioAsyncMonitorFrameworkChanges(frameworkhandle, changetypes).ok() }
}
#[inline]
pub unsafe fn WinBioAsyncOpenFramework(notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: Option<super::super::Foundation::HWND>, messagecode: Option<u32>, callbackroutine: PWINBIO_ASYNC_COMPLETION_CALLBACK, userdata: Option<*const core::ffi::c_void>, asynchronousopen: bool, frameworkhandle: Option<*mut u32>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioAsyncOpenFramework(notificationmethod : WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow : super::super::Foundation:: HWND, messagecode : u32, callbackroutine : PWINBIO_ASYNC_COMPLETION_CALLBACK, userdata : *const core::ffi::c_void, asynchronousopen : windows_core::BOOL, frameworkhandle : *mut u32) -> windows_core::HRESULT);
    unsafe { WinBioAsyncOpenFramework(notificationmethod, targetwindow.unwrap_or(core::mem::zeroed()) as _, messagecode.unwrap_or(core::mem::zeroed()) as _, callbackroutine, userdata.unwrap_or(core::mem::zeroed()) as _, asynchronousopen.into(), frameworkhandle.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WinBioAsyncOpenSession(factor: u32, pooltype: WINBIO_POOL, flags: u32, unitarray: Option<&[u32]>, databaseid: Option<*const windows_core::GUID>, notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: Option<super::super::Foundation::HWND>, messagecode: Option<u32>, callbackroutine: PWINBIO_ASYNC_COMPLETION_CALLBACK, userdata: Option<*const core::ffi::c_void>, asynchronousopen: bool, sessionhandle: Option<*mut u32>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioAsyncOpenSession(factor : u32, pooltype : WINBIO_POOL, flags : u32, unitarray : *const u32, unitcount : usize, databaseid : *const windows_core::GUID, notificationmethod : WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow : super::super::Foundation:: HWND, messagecode : u32, callbackroutine : PWINBIO_ASYNC_COMPLETION_CALLBACK, userdata : *const core::ffi::c_void, asynchronousopen : windows_core::BOOL, sessionhandle : *mut u32) -> windows_core::HRESULT);
    unsafe {
        WinBioAsyncOpenSession(
            factor,
            pooltype,
            flags,
            core::mem::transmute(unitarray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            unitarray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            databaseid.unwrap_or(core::mem::zeroed()) as _,
            notificationmethod,
            targetwindow.unwrap_or(core::mem::zeroed()) as _,
            messagecode.unwrap_or(core::mem::zeroed()) as _,
            callbackroutine,
            userdata.unwrap_or(core::mem::zeroed()) as _,
            asynchronousopen.into(),
            sessionhandle.unwrap_or(core::mem::zeroed()) as _,
        )
        .ok()
    }
}
#[inline]
pub unsafe fn WinBioCancel(sessionhandle: u32) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioCancel(sessionhandle : u32) -> windows_core::HRESULT);
    unsafe { WinBioCancel(sessionhandle).ok() }
}
#[inline]
pub unsafe fn WinBioCaptureSample(sessionhandle: u32, purpose: u8, flags: u8, unitid: Option<*mut u32>, sample: *mut *mut WINBIO_BIR, samplesize: Option<*mut usize>, rejectdetail: Option<*mut u32>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioCaptureSample(sessionhandle : u32, purpose : u8, flags : u8, unitid : *mut u32, sample : *mut *mut WINBIO_BIR, samplesize : *mut usize, rejectdetail : *mut u32) -> windows_core::HRESULT);
    unsafe { WinBioCaptureSample(sessionhandle, purpose, flags, unitid.unwrap_or(core::mem::zeroed()) as _, sample as _, samplesize.unwrap_or(core::mem::zeroed()) as _, rejectdetail.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WinBioCaptureSampleWithCallback(sessionhandle: u32, purpose: u8, flags: u8, capturecallback: PWINBIO_CAPTURE_CALLBACK, capturecallbackcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioCaptureSampleWithCallback(sessionhandle : u32, purpose : u8, flags : u8, capturecallback : PWINBIO_CAPTURE_CALLBACK, capturecallbackcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WinBioCaptureSampleWithCallback(sessionhandle, purpose, flags, capturecallback, capturecallbackcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WinBioCloseFramework(frameworkhandle: u32) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioCloseFramework(frameworkhandle : u32) -> windows_core::HRESULT);
    unsafe { WinBioCloseFramework(frameworkhandle).ok() }
}
#[inline]
pub unsafe fn WinBioCloseSession(sessionhandle: u32) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioCloseSession(sessionhandle : u32) -> windows_core::HRESULT);
    unsafe { WinBioCloseSession(sessionhandle).ok() }
}
#[inline]
pub unsafe fn WinBioControlUnit(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: &[u8], receivebuffer: &mut [u8], receivedatasize: *mut usize, operationstatus: Option<*mut u32>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioControlUnit(sessionhandle : u32, unitid : u32, component : WINBIO_COMPONENT, controlcode : u32, sendbuffer : *const u8, sendbuffersize : usize, receivebuffer : *mut u8, receivebuffersize : usize, receivedatasize : *mut usize, operationstatus : *mut u32) -> windows_core::HRESULT);
    unsafe { WinBioControlUnit(sessionhandle, unitid, component, controlcode, core::mem::transmute(sendbuffer.as_ptr()), sendbuffer.len().try_into().unwrap(), core::mem::transmute(receivebuffer.as_ptr()), receivebuffer.len().try_into().unwrap(), receivedatasize as _, operationstatus.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WinBioControlUnitPrivileged(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: &[u8], receivebuffer: &mut [u8], receivedatasize: *mut usize, operationstatus: Option<*mut u32>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioControlUnitPrivileged(sessionhandle : u32, unitid : u32, component : WINBIO_COMPONENT, controlcode : u32, sendbuffer : *const u8, sendbuffersize : usize, receivebuffer : *mut u8, receivebuffersize : usize, receivedatasize : *mut usize, operationstatus : *mut u32) -> windows_core::HRESULT);
    unsafe { WinBioControlUnitPrivileged(sessionhandle, unitid, component, controlcode, core::mem::transmute(sendbuffer.as_ptr()), sendbuffer.len().try_into().unwrap(), core::mem::transmute(receivebuffer.as_ptr()), receivebuffer.len().try_into().unwrap(), receivedatasize as _, operationstatus.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WinBioDeleteTemplate(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioDeleteTemplate(sessionhandle : u32, unitid : u32, identity : *const WINBIO_IDENTITY, subfactor : u8) -> windows_core::HRESULT);
    unsafe { WinBioDeleteTemplate(sessionhandle, unitid, identity, subfactor).ok() }
}
#[inline]
pub unsafe fn WinBioEnrollBegin(sessionhandle: u32, subfactor: u8, unitid: u32) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioEnrollBegin(sessionhandle : u32, subfactor : u8, unitid : u32) -> windows_core::HRESULT);
    unsafe { WinBioEnrollBegin(sessionhandle, subfactor, unitid).ok() }
}
#[inline]
pub unsafe fn WinBioEnrollCapture(sessionhandle: u32, rejectdetail: Option<*mut u32>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioEnrollCapture(sessionhandle : u32, rejectdetail : *mut u32) -> windows_core::HRESULT);
    unsafe { WinBioEnrollCapture(sessionhandle, rejectdetail.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WinBioEnrollCaptureWithCallback(sessionhandle: u32, enrollcallback: PWINBIO_ENROLL_CAPTURE_CALLBACK, enrollcallbackcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioEnrollCaptureWithCallback(sessionhandle : u32, enrollcallback : PWINBIO_ENROLL_CAPTURE_CALLBACK, enrollcallbackcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WinBioEnrollCaptureWithCallback(sessionhandle, enrollcallback, enrollcallbackcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WinBioEnrollCommit(sessionhandle: u32, identity: Option<*mut WINBIO_IDENTITY>, isnewtemplate: Option<*mut u8>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioEnrollCommit(sessionhandle : u32, identity : *mut WINBIO_IDENTITY, isnewtemplate : *mut u8) -> windows_core::HRESULT);
    unsafe { WinBioEnrollCommit(sessionhandle, identity.unwrap_or(core::mem::zeroed()) as _, isnewtemplate.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WinBioEnrollDiscard(sessionhandle: u32) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioEnrollDiscard(sessionhandle : u32) -> windows_core::HRESULT);
    unsafe { WinBioEnrollDiscard(sessionhandle).ok() }
}
#[inline]
pub unsafe fn WinBioEnrollSelect(sessionhandle: u32, selectorvalue: u64) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioEnrollSelect(sessionhandle : u32, selectorvalue : u64) -> windows_core::HRESULT);
    unsafe { WinBioEnrollSelect(sessionhandle, selectorvalue).ok() }
}
#[inline]
pub unsafe fn WinBioEnumBiometricUnits(factor: u32, unitschemaarray: *mut *mut WINBIO_UNIT_SCHEMA, unitcount: *mut usize) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioEnumBiometricUnits(factor : u32, unitschemaarray : *mut *mut WINBIO_UNIT_SCHEMA, unitcount : *mut usize) -> windows_core::HRESULT);
    unsafe { WinBioEnumBiometricUnits(factor, unitschemaarray as _, unitcount as _).ok() }
}
#[inline]
pub unsafe fn WinBioEnumDatabases(factor: u32, storageschemaarray: *mut *mut WINBIO_STORAGE_SCHEMA, storagecount: *mut usize) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioEnumDatabases(factor : u32, storageschemaarray : *mut *mut WINBIO_STORAGE_SCHEMA, storagecount : *mut usize) -> windows_core::HRESULT);
    unsafe { WinBioEnumDatabases(factor, storageschemaarray as _, storagecount as _).ok() }
}
#[inline]
pub unsafe fn WinBioEnumEnrollments(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactorarray: *mut *mut u8, subfactorcount: Option<*mut usize>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioEnumEnrollments(sessionhandle : u32, unitid : u32, identity : *const WINBIO_IDENTITY, subfactorarray : *mut *mut u8, subfactorcount : *mut usize) -> windows_core::HRESULT);
    unsafe { WinBioEnumEnrollments(sessionhandle, unitid, identity, subfactorarray as _, subfactorcount.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WinBioEnumServiceProviders(factor: u32, bspschemaarray: *mut *mut WINBIO_BSP_SCHEMA, bspcount: *mut usize) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioEnumServiceProviders(factor : u32, bspschemaarray : *mut *mut WINBIO_BSP_SCHEMA, bspcount : *mut usize) -> windows_core::HRESULT);
    unsafe { WinBioEnumServiceProviders(factor, bspschemaarray as _, bspcount as _).ok() }
}
#[inline]
pub unsafe fn WinBioFree(address: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioFree(address : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WinBioFree(address).ok() }
}
#[inline]
pub unsafe fn WinBioGetCredentialState(identity: WINBIO_IDENTITY, r#type: WINBIO_CREDENTIAL_TYPE) -> windows_core::Result<WINBIO_CREDENTIAL_STATE> {
    windows_link::link!("winbio.dll" "system" fn WinBioGetCredentialState(identity : WINBIO_IDENTITY, r#type : WINBIO_CREDENTIAL_TYPE, credentialstate : *mut WINBIO_CREDENTIAL_STATE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WinBioGetCredentialState(core::mem::transmute(identity), r#type, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WinBioGetDomainLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE) {
    windows_link::link!("winbio.dll" "system" fn WinBioGetDomainLogonSetting(value : *mut u8, source : *mut WINBIO_SETTING_SOURCE));
    unsafe { WinBioGetDomainLogonSetting(value as _, source as _) }
}
#[inline]
pub unsafe fn WinBioGetEnabledSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE) {
    windows_link::link!("winbio.dll" "system" fn WinBioGetEnabledSetting(value : *mut u8, source : *mut WINBIO_SETTING_SOURCE));
    unsafe { WinBioGetEnabledSetting(value as _, source as _) }
}
#[inline]
pub unsafe fn WinBioGetEnrolledFactors(accountowner: *const WINBIO_IDENTITY) -> windows_core::Result<u32> {
    windows_link::link!("winbio.dll" "system" fn WinBioGetEnrolledFactors(accountowner : *const WINBIO_IDENTITY, enrolledfactors : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WinBioGetEnrolledFactors(accountowner, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WinBioGetLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE) {
    windows_link::link!("winbio.dll" "system" fn WinBioGetLogonSetting(value : *mut u8, source : *mut WINBIO_SETTING_SOURCE));
    unsafe { WinBioGetLogonSetting(value as _, source as _) }
}
#[inline]
pub unsafe fn WinBioGetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: Option<u32>, identity: Option<*const WINBIO_IDENTITY>, subfactor: Option<u8>, propertybuffer: *mut *mut core::ffi::c_void, propertybuffersize: Option<*mut usize>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioGetProperty(sessionhandle : u32, propertytype : u32, propertyid : u32, unitid : u32, identity : *const WINBIO_IDENTITY, subfactor : u8, propertybuffer : *mut *mut core::ffi::c_void, propertybuffersize : *mut usize) -> windows_core::HRESULT);
    unsafe { WinBioGetProperty(sessionhandle, propertytype, propertyid, unitid.unwrap_or(core::mem::zeroed()) as _, identity.unwrap_or(core::mem::zeroed()) as _, subfactor.unwrap_or(core::mem::zeroed()) as _, propertybuffer as _, propertybuffersize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WinBioIdentify(sessionhandle: u32, unitid: Option<*mut u32>, identity: Option<*mut WINBIO_IDENTITY>, subfactor: Option<*mut u8>, rejectdetail: Option<*mut u32>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioIdentify(sessionhandle : u32, unitid : *mut u32, identity : *mut WINBIO_IDENTITY, subfactor : *mut u8, rejectdetail : *mut u32) -> windows_core::HRESULT);
    unsafe { WinBioIdentify(sessionhandle, unitid.unwrap_or(core::mem::zeroed()) as _, identity.unwrap_or(core::mem::zeroed()) as _, subfactor.unwrap_or(core::mem::zeroed()) as _, rejectdetail.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WinBioIdentifyWithCallback(sessionhandle: u32, identifycallback: PWINBIO_IDENTIFY_CALLBACK, identifycallbackcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioIdentifyWithCallback(sessionhandle : u32, identifycallback : PWINBIO_IDENTIFY_CALLBACK, identifycallbackcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WinBioIdentifyWithCallback(sessionhandle, identifycallback, identifycallbackcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WinBioImproveBegin(sessionhandle: u32, unitid: u32) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioImproveBegin(sessionhandle : u32, unitid : u32) -> windows_core::HRESULT);
    unsafe { WinBioImproveBegin(sessionhandle, unitid).ok() }
}
#[inline]
pub unsafe fn WinBioImproveEnd(sessionhandle: u32) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioImproveEnd(sessionhandle : u32) -> windows_core::HRESULT);
    unsafe { WinBioImproveEnd(sessionhandle).ok() }
}
#[inline]
pub unsafe fn WinBioLocateSensor(sessionhandle: u32, unitid: Option<*mut u32>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioLocateSensor(sessionhandle : u32, unitid : *mut u32) -> windows_core::HRESULT);
    unsafe { WinBioLocateSensor(sessionhandle, unitid.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WinBioLocateSensorWithCallback(sessionhandle: u32, locatecallback: PWINBIO_LOCATE_SENSOR_CALLBACK, locatecallbackcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioLocateSensorWithCallback(sessionhandle : u32, locatecallback : PWINBIO_LOCATE_SENSOR_CALLBACK, locatecallbackcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WinBioLocateSensorWithCallback(sessionhandle, locatecallback, locatecallbackcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WinBioLockUnit(sessionhandle: u32, unitid: u32) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioLockUnit(sessionhandle : u32, unitid : u32) -> windows_core::HRESULT);
    unsafe { WinBioLockUnit(sessionhandle, unitid).ok() }
}
#[inline]
pub unsafe fn WinBioLogonIdentifiedUser(sessionhandle: u32) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioLogonIdentifiedUser(sessionhandle : u32) -> windows_core::HRESULT);
    unsafe { WinBioLogonIdentifiedUser(sessionhandle).ok() }
}
#[inline]
pub unsafe fn WinBioMonitorPresence(sessionhandle: u32, unitid: u32) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioMonitorPresence(sessionhandle : u32, unitid : u32) -> windows_core::HRESULT);
    unsafe { WinBioMonitorPresence(sessionhandle, unitid).ok() }
}
#[inline]
pub unsafe fn WinBioOpenSession(factor: u32, pooltype: WINBIO_POOL, flags: u32, unitarray: Option<&[u32]>, databaseid: Option<*const windows_core::GUID>) -> windows_core::Result<u32> {
    windows_link::link!("winbio.dll" "system" fn WinBioOpenSession(factor : u32, pooltype : WINBIO_POOL, flags : u32, unitarray : *const u32, unitcount : usize, databaseid : *const windows_core::GUID, sessionhandle : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WinBioOpenSession(factor, pooltype, flags, core::mem::transmute(unitarray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), unitarray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), databaseid.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WinBioRegisterEventMonitor(sessionhandle: u32, eventmask: u32, eventcallback: PWINBIO_EVENT_CALLBACK, eventcallbackcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioRegisterEventMonitor(sessionhandle : u32, eventmask : u32, eventcallback : PWINBIO_EVENT_CALLBACK, eventcallbackcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WinBioRegisterEventMonitor(sessionhandle, eventmask, eventcallback, eventcallbackcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WinBioReleaseFocus() -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioReleaseFocus() -> windows_core::HRESULT);
    unsafe { WinBioReleaseFocus().ok() }
}
#[inline]
pub unsafe fn WinBioRemoveAllCredentials() -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioRemoveAllCredentials() -> windows_core::HRESULT);
    unsafe { WinBioRemoveAllCredentials().ok() }
}
#[inline]
pub unsafe fn WinBioRemoveAllDomainCredentials() -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioRemoveAllDomainCredentials() -> windows_core::HRESULT);
    unsafe { WinBioRemoveAllDomainCredentials().ok() }
}
#[inline]
pub unsafe fn WinBioRemoveCredential(identity: WINBIO_IDENTITY, r#type: WINBIO_CREDENTIAL_TYPE) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioRemoveCredential(identity : WINBIO_IDENTITY, r#type : WINBIO_CREDENTIAL_TYPE) -> windows_core::HRESULT);
    unsafe { WinBioRemoveCredential(core::mem::transmute(identity), r#type).ok() }
}
#[inline]
pub unsafe fn WinBioSetCredential(r#type: WINBIO_CREDENTIAL_TYPE, credential: &[u8], format: WINBIO_CREDENTIAL_FORMAT) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioSetCredential(r#type : WINBIO_CREDENTIAL_TYPE, credential : *const u8, credentialsize : usize, format : WINBIO_CREDENTIAL_FORMAT) -> windows_core::HRESULT);
    unsafe { WinBioSetCredential(r#type, core::mem::transmute(credential.as_ptr()), credential.len().try_into().unwrap(), format).ok() }
}
#[inline]
pub unsafe fn WinBioSetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: Option<u32>, identity: Option<*const WINBIO_IDENTITY>, subfactor: Option<u8>, propertybuffer: *const core::ffi::c_void, propertybuffersize: usize) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioSetProperty(sessionhandle : u32, propertytype : u32, propertyid : u32, unitid : u32, identity : *const WINBIO_IDENTITY, subfactor : u8, propertybuffer : *const core::ffi::c_void, propertybuffersize : usize) -> windows_core::HRESULT);
    unsafe { WinBioSetProperty(sessionhandle, propertytype, propertyid, unitid.unwrap_or(core::mem::zeroed()) as _, identity.unwrap_or(core::mem::zeroed()) as _, subfactor.unwrap_or(core::mem::zeroed()) as _, propertybuffer, propertybuffersize).ok() }
}
#[inline]
pub unsafe fn WinBioUnlockUnit(sessionhandle: u32, unitid: u32) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioUnlockUnit(sessionhandle : u32, unitid : u32) -> windows_core::HRESULT);
    unsafe { WinBioUnlockUnit(sessionhandle, unitid).ok() }
}
#[inline]
pub unsafe fn WinBioUnregisterEventMonitor(sessionhandle: u32) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioUnregisterEventMonitor(sessionhandle : u32) -> windows_core::HRESULT);
    unsafe { WinBioUnregisterEventMonitor(sessionhandle).ok() }
}
#[inline]
pub unsafe fn WinBioVerify(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, unitid: Option<*mut u32>, r#match: Option<*mut u8>, rejectdetail: Option<*mut u32>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioVerify(sessionhandle : u32, identity : *const WINBIO_IDENTITY, subfactor : u8, unitid : *mut u32, r#match : *mut u8, rejectdetail : *mut u32) -> windows_core::HRESULT);
    unsafe { WinBioVerify(sessionhandle, identity, subfactor, unitid.unwrap_or(core::mem::zeroed()) as _, r#match.unwrap_or(core::mem::zeroed()) as _, rejectdetail.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WinBioVerifyWithCallback(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, verifycallback: PWINBIO_VERIFY_CALLBACK, verifycallbackcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioVerifyWithCallback(sessionhandle : u32, identity : *const WINBIO_IDENTITY, subfactor : u8, verifycallback : PWINBIO_VERIFY_CALLBACK, verifycallbackcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WinBioVerifyWithCallback(sessionhandle, identity, subfactor, verifycallback, verifycallbackcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WinBioWait(sessionhandle: u32) -> windows_core::Result<()> {
    windows_link::link!("winbio.dll" "system" fn WinBioWait(sessionhandle : u32) -> windows_core::HRESULT);
    unsafe { WinBioWait(sessionhandle).ok() }
}
pub const FACILITY_NONE: u32 = 0u32;
pub const FACILITY_WINBIO: u32 = 9u32;
pub const GUID_DEVINTERFACE_BIOMETRIC_READER: windows_core::GUID = windows_core::GUID::from_u128(0xe2b5183a_99ea_4cc3_ad6b_80ca8d715b80);
pub const IOCTL_BIOMETRIC_VENDOR: u32 = 4464640u32;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_ACCEPT_PRIVATE_SENSOR_TYPE_INFO_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, typeinfobufferaddress: *const u8, typeinfobuffersize: usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_ACCEPT_SAMPLE_DATA_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, samplebuffer: *const WINBIO_BIR, samplesize: usize, purpose: u8, rejectdetail: *mut u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_ACTIVATE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_ATTACH_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_CHECK_FOR_DUPLICATE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, duplicate: *mut bool) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_CLEAR_CONTEXT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_COMMIT_ENROLLMENT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8, payloadblob: *const u8, payloadblobsize: usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_CONTROL_UNIT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_CONTROL_UNIT_PRIVILEGED_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_CREATE_ENROLLMENT_AUTHENTICATED_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, nonce: *mut *mut u8, noncesize: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_CREATE_ENROLLMENT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_CREATE_KEY_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, key: *const u8, keysize: usize, keyidentifier: *mut u8, keyidentifiersize: usize, resultsize: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_DEACTIVATE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_DETACH_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_DISCARD_ENROLLMENT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_EXPORT_ENGINE_DATA_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, flags: u8, samplebuffer: *mut *mut WINBIO_BIR, samplesize: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_GET_ENROLLMENT_HASH_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, hashvalue: *mut *mut u8, hashsize: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_GET_ENROLLMENT_STATUS_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, rejectdetail: *mut u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_IDENTIFY_ALL_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, presencecount: *mut usize, presencearray: *mut *mut WINBIO_PRESENCE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_IDENTIFY_FEATURE_SET_AUTHENTICATED_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, nonce: *const u8, noncesize: usize, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, rejectdetail: *mut u32, authentication: *mut *mut u8, authenticationsize: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_IDENTIFY_FEATURE_SET_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, payloadblob: *mut *mut u8, payloadblobsize: *mut usize, hashvalue: *mut *mut u8, hashsize: *mut usize, rejectdetail: *mut u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_IDENTIFY_FEATURE_SET_SECURE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, nonce: *const u8, noncesize: usize, keyidentifier: *const u8, keyidentifiersize: usize, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, rejectdetail: *mut u32, authorization: *mut *mut u8, authorizationsize: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_NOTIFY_POWER_CHANGE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, powereventtype: u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_PIPELINE_CLEANUP_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_PIPELINE_INIT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_QUERY_CALIBRATION_DATA_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, discardandrepeatcapture: *mut bool, calibrationbuffer: *mut u8, calibrationbuffersize: *mut usize, maxbuffersize: usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_QUERY_EXTENDED_ENROLLMENT_STATUS_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, enrollmentstatus: *mut WINBIO_EXTENDED_ENROLLMENT_STATUS, enrollmentstatussize: usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_QUERY_EXTENDED_INFO_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, engineinfo: *mut WINBIO_EXTENDED_ENGINE_INFO, engineinfosize: usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_QUERY_HASH_ALGORITHMS_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, algorithmcount: *mut usize, algorithmbuffersize: *mut usize, algorithmbuffer: *mut *mut u8) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_QUERY_INDEX_VECTOR_SIZE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, indexelementcount: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_QUERY_PREFERRED_FORMAT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, standardformat: *mut WINBIO_REGISTERED_FORMAT, vendorformat: *mut windows_core::GUID) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_QUERY_SAMPLE_HINT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, samplehint: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_REFRESH_CACHE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_RESERVED_1_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *mut WINBIO_IDENTITY) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_SELECT_CALIBRATION_FORMAT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, formatarray: *const windows_core::GUID, formatcount: usize, selectedformat: *mut windows_core::GUID, maxbuffersize: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_SET_ACCOUNT_POLICY_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, policyitemarray: *const WINBIO_ACCOUNT_POLICY, policyitemcount: usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_SET_ENROLLMENT_PARAMETERS_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, parameters: *const WINBIO_EXTENDED_ENROLLMENT_PARAMETERS) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_SET_ENROLLMENT_SELECTOR_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, selectorvalue: u64) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_SET_HASH_ALGORITHM_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, algorithmbuffersize: usize, algorithmbuffer: *const u8) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_UPDATE_ENROLLMENT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, rejectdetail: *mut u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_ENGINE_VERIFY_FEATURE_SET_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8, r#match: *mut bool, payloadblob: *mut *mut u8, payloadblobsize: *mut usize, hashvalue: *mut *mut u8, hashsize: *mut usize, rejectdetail: *mut u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_ALLOCATE_MEMORY_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, allocationsize: usize, address: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_FREE_MEMORY_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, address: *const core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_GET_PROPERTY_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, propertytype: u32, propertyid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *mut *mut core::ffi::c_void, propertybuffersize: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_LOCK_AND_VALIDATE_SECURE_BUFFER_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, securebufferidentifier: windows_core::GUID, securebufferaddress: *mut *mut core::ffi::c_void, securebuffersize: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_RELEASE_SECURE_BUFFER_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, securebufferidentifier: windows_core::GUID) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_SET_UNIT_STATUS_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, extendedstatus: *const WINBIO_EXTENDED_UNIT_STATUS, extendedstatussize: usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_VSM_CACHE_CLEAR_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_BEGIN_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, requiredcapacity: *mut usize, maxbuffersize: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_END_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_NEXT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, bufferaddress: *mut u8, buffersize: usize, returneddatasize: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_BEGIN_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, requiredcapacity: usize, maxbuffersize: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_END_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_NEXT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, bufferaddress: *const u8, buffersize: usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_VSM_DECRYPT_SAMPLE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, authentication: *const u8, authenticationsize: usize, iv: *const u8, ivsize: usize, encrypteddata: *mut u8, encrypteddatasize: usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_VSM_QUERY_AUTHORIZED_ENROLLMENTS_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, secureidentitycount: *mut usize, secureidentities: *mut *mut WINBIO_IDENTITY) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_1_FN = Option<unsafe extern "system" fn(pipeline: *const WINBIO_PIPELINE, reserved1: usize, reserved2: *const usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_2_FN = Option<unsafe extern "system" fn(pipeline: *const WINBIO_PIPELINE, reserved1: *const u8, reserved2: usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_3_FN = Option<unsafe extern "system" fn(pipeline: *const WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_ACCEPT_CALIBRATION_DATA_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, calibrationbuffer: *const u8, calibrationbuffersize: usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_ACTIVATE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_ASYNC_IMPORT_RAW_BUFFER_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, rawbufferaddress: *const u8, rawbuffersize: usize, resultbufferaddress: *mut *mut u8, resultbuffersize: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_ASYNC_IMPORT_SECURE_BUFFER_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, securebufferidentifier: windows_core::GUID, metadatabufferaddress: *const u8, metadatabuffersize: usize, resultbufferaddress: *mut *mut u8, resultbuffersize: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_ATTACH_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_CANCEL_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_CLEAR_CONTEXT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_CONNECT_SECURE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, connectionparams: *const WINBIO_SECURE_CONNECTION_PARAMS, connectiondata: *mut *mut WINBIO_SECURE_CONNECTION_DATA) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_CONTROL_UNIT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_CONTROL_UNIT_PRIVILEGED_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_DEACTIVATE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_DETACH_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_EXPORT_SENSOR_DATA_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, samplebuffer: *mut *mut WINBIO_BIR, samplesize: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_FINISH_CAPTURE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, rejectdetail: *mut u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_FINISH_NOTIFY_WAKE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, reason: *mut u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_GET_INDICATOR_STATUS_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, indicatorstatus: *mut u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_NOTIFY_POWER_CHANGE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, powereventtype: u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_PIPELINE_CLEANUP_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_PIPELINE_INIT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_PUSH_DATA_TO_ENGINE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, purpose: u8, flags: u8, rejectdetail: *mut u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_QUERY_CALIBRATION_FORMATS_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, formatarray: *mut windows_core::GUID, formatarraysize: usize, formatcount: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_QUERY_EXTENDED_INFO_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, sensorinfo: *mut WINBIO_EXTENDED_SENSOR_INFO, sensorinfosize: usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_QUERY_PRIVATE_SENSOR_TYPE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, typeinfobufferaddress: *mut u8, typeinfobuffersize: usize, typeinfodatasize: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_QUERY_STATUS_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, status: *mut u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_RESET_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_SET_CALIBRATION_FORMAT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, format: *const windows_core::GUID) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_SET_INDICATOR_STATUS_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, indicatorstatus: u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_SET_MODE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, mode: u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_START_CAPTURE_EX_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, purpose: u8, nonce: *const u8, noncesize: usize, flags: u8, overlapped: *mut *mut super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_START_CAPTURE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, purpose: u8, overlapped: *mut *mut super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_SENSOR_START_NOTIFY_WAKE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, overlapped: *mut *mut super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_ACTIVATE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_ADD_RECORD_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordcontents: *const WINBIO_STORAGE_RECORD) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_ATTACH_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_CLEAR_CONTEXT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_CLOSE_DATABASE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_CONTROL_UNIT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_CONTROL_UNIT_PRIVILEGED_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_CREATE_DATABASE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, databaseid: *const windows_core::GUID, factor: u32, format: *const windows_core::GUID, filepath: windows_core::PCWSTR, connectstring: windows_core::PCWSTR, indexelementcount: usize, initialsize: usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_DEACTIVATE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_DELETE_RECORD_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_DETACH_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_ERASE_DATABASE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, databaseid: *const windows_core::GUID, filepath: windows_core::PCWSTR, connectstring: windows_core::PCWSTR) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_FIRST_RECORD_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_GET_CURRENT_RECORD_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordcontents: *mut WINBIO_STORAGE_RECORD) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_GET_DATABASE_SIZE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, availablerecordcount: *mut usize, totalrecordcount: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_GET_DATA_FORMAT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, format: *mut windows_core::GUID, version: *mut WINBIO_VERSION) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_GET_RECORD_COUNT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordcount: *mut usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_NEXT_RECORD_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_NOTIFY_DATABASE_CHANGE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordsadded: bool) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_NOTIFY_POWER_CHANGE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, powereventtype: u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_OPEN_DATABASE_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, databaseid: *const windows_core::GUID, filepath: windows_core::PCWSTR, connectstring: windows_core::PCWSTR) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_PIPELINE_CLEANUP_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_PIPELINE_INIT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_QUERY_BY_CONTENT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, subfactor: u8, indexvector: *const u32, indexelementcount: usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_QUERY_BY_SUBJECT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_QUERY_EXTENDED_INFO_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, storageinfo: *mut WINBIO_EXTENDED_STORAGE_INFO, storageinfosize: usize) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_RESERVED_1_FN = Option<unsafe extern "system" fn(pipeline: *const WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, reserved1: *const u64, reserved2: *const u64) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_RESERVED_2_FN = Option<unsafe extern "system" fn(pipeline: *const WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_UPDATE_RECORD_BEGIN_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8, recordcontents: *mut WINBIO_STORAGE_RECORD) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PIBIO_STORAGE_UPDATE_RECORD_COMMIT_FN = Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordcontents: *const WINBIO_STORAGE_RECORD) -> windows_core::HRESULT>;
pub type PWINBIO_ASYNC_COMPLETION_CALLBACK = Option<unsafe extern "system" fn(asyncresult: *const WINBIO_ASYNC_RESULT)>;
pub type PWINBIO_CAPTURE_CALLBACK = Option<unsafe extern "system" fn(capturecallbackcontext: *const core::ffi::c_void, operationstatus: windows_core::HRESULT, unitid: u32, sample: *const WINBIO_BIR, samplesize: usize, rejectdetail: u32)>;
pub type PWINBIO_ENROLL_CAPTURE_CALLBACK = Option<unsafe extern "system" fn(enrollcallbackcontext: *const core::ffi::c_void, operationstatus: windows_core::HRESULT, rejectdetail: u32)>;
pub type PWINBIO_EVENT_CALLBACK = Option<unsafe extern "system" fn(eventcallbackcontext: *const core::ffi::c_void, operationstatus: windows_core::HRESULT, event: *const WINBIO_EVENT)>;
pub type PWINBIO_IDENTIFY_CALLBACK = Option<unsafe extern "system" fn(identifycallbackcontext: *const core::ffi::c_void, operationstatus: windows_core::HRESULT, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, rejectdetail: u32)>;
pub type PWINBIO_LOCATE_SENSOR_CALLBACK = Option<unsafe extern "system" fn(locatecallbackcontext: *const core::ffi::c_void, operationstatus: windows_core::HRESULT, unitid: u32)>;
#[cfg(feature = "Win32_System_IO")]
pub type PWINBIO_QUERY_ENGINE_INTERFACE_FN = Option<unsafe extern "system" fn(engineinterface: *mut *mut WINBIO_ENGINE_INTERFACE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PWINBIO_QUERY_SENSOR_INTERFACE_FN = Option<unsafe extern "system" fn(sensorinterface: *mut *mut WINBIO_SENSOR_INTERFACE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type PWINBIO_QUERY_STORAGE_INTERFACE_FN = Option<unsafe extern "system" fn(storageinterface: *mut *mut WINBIO_STORAGE_INTERFACE) -> windows_core::HRESULT>;
pub type PWINBIO_VERIFY_CALLBACK = Option<unsafe extern "system" fn(verifycallbackcontext: *const core::ffi::c_void, operationstatus: windows_core::HRESULT, unitid: u32, r#match: bool, rejectdetail: u32)>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_ACCOUNT_POLICY {
    pub Identity: WINBIO_IDENTITY,
    pub AntiSpoofBehavior: WINBIO_ANTI_SPOOF_POLICY_ACTION,
}
impl Default for WINBIO_ACCOUNT_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_ADAPTER_INTERFACE_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
pub const WINBIO_ANSI_381_IMG_BIT_PACKED: u16 = 1u16;
pub const WINBIO_ANSI_381_IMG_COMPRESSED_JPEG: u16 = 3u16;
pub const WINBIO_ANSI_381_IMG_COMPRESSED_JPEG2000: u16 = 4u16;
pub const WINBIO_ANSI_381_IMG_COMPRESSED_PNG: u16 = 5u16;
pub const WINBIO_ANSI_381_IMG_COMPRESSED_WSQ: u16 = 2u16;
pub const WINBIO_ANSI_381_IMG_UNCOMPRESSED: u16 = 0u16;
pub const WINBIO_ANSI_381_IMP_TYPE_LATENT: u16 = 7u16;
pub const WINBIO_ANSI_381_IMP_TYPE_LIVE_SCAN_CONTACTLESS: u16 = 9u16;
pub const WINBIO_ANSI_381_IMP_TYPE_LIVE_SCAN_PLAIN: u16 = 0u16;
pub const WINBIO_ANSI_381_IMP_TYPE_LIVE_SCAN_ROLLED: u16 = 1u16;
pub const WINBIO_ANSI_381_IMP_TYPE_NONLIVE_SCAN_PLAIN: u16 = 2u16;
pub const WINBIO_ANSI_381_IMP_TYPE_NONLIVE_SCAN_ROLLED: u16 = 3u16;
pub const WINBIO_ANSI_381_IMP_TYPE_SWIPE: u16 = 8u16;
pub const WINBIO_ANSI_381_PIXELS_PER_CM: u16 = 2u16;
pub const WINBIO_ANSI_381_PIXELS_PER_INCH: u16 = 1u16;
pub const WINBIO_ANTI_SPOOF_DISABLE: WINBIO_ANTI_SPOOF_POLICY_ACTION = WINBIO_ANTI_SPOOF_POLICY_ACTION(0i32);
pub const WINBIO_ANTI_SPOOF_ENABLE: WINBIO_ANTI_SPOOF_POLICY_ACTION = WINBIO_ANTI_SPOOF_POLICY_ACTION(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_ANTI_SPOOF_POLICY {
    pub Action: WINBIO_ANTI_SPOOF_POLICY_ACTION,
    pub Source: WINBIO_POLICY_SOURCE,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINBIO_ANTI_SPOOF_POLICY_ACTION(pub i32);
pub const WINBIO_ANTI_SPOOF_REMOVE: WINBIO_ANTI_SPOOF_POLICY_ACTION = WINBIO_ANTI_SPOOF_POLICY_ACTION(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINBIO_ASYNC_NOTIFICATION_METHOD(pub i32);
pub const WINBIO_ASYNC_NOTIFY_CALLBACK: WINBIO_ASYNC_NOTIFICATION_METHOD = WINBIO_ASYNC_NOTIFICATION_METHOD(1i32);
pub const WINBIO_ASYNC_NOTIFY_MAXIMUM_VALUE: WINBIO_ASYNC_NOTIFICATION_METHOD = WINBIO_ASYNC_NOTIFICATION_METHOD(3i32);
pub const WINBIO_ASYNC_NOTIFY_MESSAGE: WINBIO_ASYNC_NOTIFICATION_METHOD = WINBIO_ASYNC_NOTIFICATION_METHOD(2i32);
pub const WINBIO_ASYNC_NOTIFY_NONE: WINBIO_ASYNC_NOTIFICATION_METHOD = WINBIO_ASYNC_NOTIFICATION_METHOD(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT {
    pub SessionHandle: u32,
    pub Operation: u32,
    pub SequenceNumber: u64,
    pub TimeStamp: i64,
    pub ApiStatus: windows_core::HRESULT,
    pub UnitId: u32,
    pub UserData: *mut core::ffi::c_void,
    pub Parameters: WINBIO_ASYNC_RESULT_0,
}
impl Default for WINBIO_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINBIO_ASYNC_RESULT_0 {
    pub Verify: WINBIO_ASYNC_RESULT_0_0,
    pub Identify: WINBIO_ASYNC_RESULT_0_1,
    pub EnrollBegin: WINBIO_ASYNC_RESULT_0_2,
    pub EnrollCapture: WINBIO_ASYNC_RESULT_0_3,
    pub EnrollCommit: WINBIO_ASYNC_RESULT_0_4,
    pub EnumEnrollments: WINBIO_ASYNC_RESULT_0_5,
    pub CaptureSample: WINBIO_ASYNC_RESULT_0_6,
    pub DeleteTemplate: WINBIO_ASYNC_RESULT_0_7,
    pub GetProperty: WINBIO_ASYNC_RESULT_0_8,
    pub SetProperty: WINBIO_ASYNC_RESULT_0_9,
    pub GetEvent: WINBIO_ASYNC_RESULT_0_10,
    pub ControlUnit: WINBIO_ASYNC_RESULT_0_11,
    pub EnumServiceProviders: WINBIO_ASYNC_RESULT_0_12,
    pub EnumBiometricUnits: WINBIO_ASYNC_RESULT_0_13,
    pub EnumDatabases: WINBIO_ASYNC_RESULT_0_14,
    pub VerifyAndReleaseTicket: WINBIO_ASYNC_RESULT_0_15,
    pub IdentifyAndReleaseTicket: WINBIO_ASYNC_RESULT_0_16,
    pub EnrollSelect: WINBIO_ASYNC_RESULT_0_17,
    pub MonitorPresence: WINBIO_ASYNC_RESULT_0_18,
    pub GetProtectionPolicy: WINBIO_ASYNC_RESULT_0_19,
    pub NotifyUnitStatusChange: WINBIO_ASYNC_RESULT_0_20,
}
impl Default for WINBIO_ASYNC_RESULT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_6 {
    pub Sample: *mut WINBIO_BIR,
    pub SampleSize: usize,
    pub RejectDetail: u32,
}
impl Default for WINBIO_ASYNC_RESULT_0_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_11 {
    pub Component: WINBIO_COMPONENT,
    pub ControlCode: u32,
    pub OperationStatus: u32,
    pub SendBuffer: *mut u8,
    pub SendBufferSize: usize,
    pub ReceiveBuffer: *mut u8,
    pub ReceiveBufferSize: usize,
    pub ReceiveDataSize: usize,
}
impl Default for WINBIO_ASYNC_RESULT_0_11 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_7 {
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
}
impl Default for WINBIO_ASYNC_RESULT_0_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_2 {
    pub SubFactor: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_3 {
    pub RejectDetail: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_4 {
    pub Identity: WINBIO_IDENTITY,
    pub IsNewTemplate: bool,
}
impl Default for WINBIO_ASYNC_RESULT_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_17 {
    pub SelectorValue: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_13 {
    pub UnitCount: usize,
    pub UnitSchemaArray: *mut WINBIO_UNIT_SCHEMA,
}
impl Default for WINBIO_ASYNC_RESULT_0_13 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_14 {
    pub StorageCount: usize,
    pub StorageSchemaArray: *mut WINBIO_STORAGE_SCHEMA,
}
impl Default for WINBIO_ASYNC_RESULT_0_14 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_5 {
    pub Identity: WINBIO_IDENTITY,
    pub SubFactorCount: usize,
    pub SubFactorArray: *mut u8,
}
impl Default for WINBIO_ASYNC_RESULT_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_12 {
    pub BspCount: usize,
    pub BspSchemaArray: *mut WINBIO_BSP_SCHEMA,
}
impl Default for WINBIO_ASYNC_RESULT_0_12 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_10 {
    pub Event: WINBIO_EVENT,
}
impl Default for WINBIO_ASYNC_RESULT_0_10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_8 {
    pub PropertyType: u32,
    pub PropertyId: u32,
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub PropertyBufferSize: usize,
    pub PropertyBuffer: *mut core::ffi::c_void,
}
impl Default for WINBIO_ASYNC_RESULT_0_8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_19 {
    pub Identity: WINBIO_IDENTITY,
    pub Policy: WINBIO_PROTECTION_POLICY,
}
impl Default for WINBIO_ASYNC_RESULT_0_19 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_16 {
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub RejectDetail: u32,
    pub Ticket: u64,
}
impl Default for WINBIO_ASYNC_RESULT_0_16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_1 {
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub RejectDetail: u32,
}
impl Default for WINBIO_ASYNC_RESULT_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_18 {
    pub ChangeType: u32,
    pub PresenceCount: usize,
    pub PresenceArray: *mut WINBIO_PRESENCE,
}
impl Default for WINBIO_ASYNC_RESULT_0_18 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_20 {
    pub ExtendedStatus: WINBIO_EXTENDED_UNIT_STATUS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_9 {
    pub PropertyType: u32,
    pub PropertyId: u32,
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub PropertyBufferSize: usize,
    pub PropertyBuffer: *mut core::ffi::c_void,
}
impl Default for WINBIO_ASYNC_RESULT_0_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_15 {
    pub Match: bool,
    pub RejectDetail: u32,
    pub Ticket: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_0 {
    pub Match: bool,
    pub RejectDetail: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_BDB_ANSI_381_HEADER {
    pub RecordLength: u64,
    pub FormatIdentifier: u32,
    pub VersionNumber: u32,
    pub ProductId: WINBIO_REGISTERED_FORMAT,
    pub CaptureDeviceId: u16,
    pub ImageAcquisitionLevel: u16,
    pub HorizontalScanResolution: u16,
    pub VerticalScanResolution: u16,
    pub HorizontalImageResolution: u16,
    pub VerticalImageResolution: u16,
    pub ElementCount: u8,
    pub ScaleUnits: u8,
    pub PixelDepth: u8,
    pub ImageCompressionAlg: u8,
    pub Reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_BDB_ANSI_381_RECORD {
    pub BlockLength: u32,
    pub HorizontalLineLength: u16,
    pub VerticalLineLength: u16,
    pub Position: u8,
    pub CountOfViews: u8,
    pub ViewNumber: u8,
    pub ImageQuality: u8,
    pub ImpressionType: u8,
    pub Reserved: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_BIR {
    pub HeaderBlock: WINBIO_BIR_DATA,
    pub StandardDataBlock: WINBIO_BIR_DATA,
    pub VendorDataBlock: WINBIO_BIR_DATA,
    pub SignatureBlock: WINBIO_BIR_DATA,
}
pub const WINBIO_BIR_ALGIN_SIZE: u32 = 8u32;
pub const WINBIO_BIR_ALIGN_SIZE: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_BIR_DATA {
    pub Size: u32,
    pub Offset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_BIR_HEADER {
    pub ValidFields: u16,
    pub HeaderVersion: u8,
    pub PatronHeaderVersion: u8,
    pub DataFlags: u8,
    pub Type: u32,
    pub Subtype: u8,
    pub Purpose: u8,
    pub DataQuality: i8,
    pub CreationDate: i64,
    pub ValidityPeriod: WINBIO_BIR_HEADER_0,
    pub BiometricDataFormat: WINBIO_REGISTERED_FORMAT,
    pub ProductId: WINBIO_REGISTERED_FORMAT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_BIR_HEADER_0 {
    pub BeginDate: i64,
    pub EndDate: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_BLANK_PAYLOAD {
    pub PayloadSize: u32,
    pub WinBioHresult: windows_core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_BSP_SCHEMA {
    pub BiometricFactor: u32,
    pub BspId: windows_core::GUID,
    pub Description: [u16; 256],
    pub Vendor: [u16; 256],
    pub Version: WINBIO_VERSION,
}
impl Default for WINBIO_BSP_SCHEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_CALIBRATION_INFO {
    pub PayloadSize: u32,
    pub WinBioHresult: windows_core::HRESULT,
    pub CalibrationData: WINBIO_DATA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_CAPTURE_DATA {
    pub PayloadSize: u32,
    pub WinBioHresult: windows_core::HRESULT,
    pub SensorStatus: u32,
    pub RejectDetail: u32,
    pub CaptureData: WINBIO_DATA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_CAPTURE_PARAMETERS {
    pub PayloadSize: u32,
    pub Purpose: u8,
    pub Format: WINBIO_REGISTERED_FORMAT,
    pub VendorFormat: windows_core::GUID,
    pub Flags: u8,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINBIO_COMPONENT(pub u32);
pub const WINBIO_COMPONENT_ENGINE: WINBIO_COMPONENT = WINBIO_COMPONENT(2u32);
pub const WINBIO_COMPONENT_SENSOR: WINBIO_COMPONENT = WINBIO_COMPONENT(1u32);
pub const WINBIO_COMPONENT_STORAGE: WINBIO_COMPONENT = WINBIO_COMPONENT(3u32);
pub const WINBIO_CREDENTIAL_ALL: WINBIO_CREDENTIAL_TYPE = WINBIO_CREDENTIAL_TYPE(-1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINBIO_CREDENTIAL_FORMAT(pub i32);
pub const WINBIO_CREDENTIAL_NOT_SET: WINBIO_CREDENTIAL_STATE = WINBIO_CREDENTIAL_STATE(1i32);
pub const WINBIO_CREDENTIAL_PASSWORD: WINBIO_CREDENTIAL_TYPE = WINBIO_CREDENTIAL_TYPE(1i32);
pub const WINBIO_CREDENTIAL_SET: WINBIO_CREDENTIAL_STATE = WINBIO_CREDENTIAL_STATE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINBIO_CREDENTIAL_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINBIO_CREDENTIAL_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_DATA {
    pub Size: u32,
    pub Data: [u8; 1],
}
impl Default for WINBIO_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINBIO_DATA_FLAG_INTEGRITY: u16 = 1u16;
pub const WINBIO_DATA_FLAG_INTERMEDIATE: u16 = 64u16;
pub const WINBIO_DATA_FLAG_OPTION_MASK_PRESENT: u16 = 8u16;
pub const WINBIO_DATA_FLAG_PRIVACY: u16 = 2u16;
pub const WINBIO_DATA_FLAG_PROCESSED: u16 = 128u16;
pub const WINBIO_DATA_FLAG_RAW: u16 = 32u16;
pub const WINBIO_DATA_FLAG_SIGNED: u16 = 4u16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_DIAGNOSTICS {
    pub PayloadSize: u32,
    pub WinBioHresult: windows_core::HRESULT,
    pub SensorStatus: u32,
    pub VendorDiagnostics: WINBIO_DATA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    pub PayloadSize: u32,
    pub Purpose: u8,
    pub Format: WINBIO_REGISTERED_FORMAT,
    pub VendorFormat: windows_core::GUID,
    pub Flags: u8,
    pub NonceSize: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_IO")]
#[derive(Clone, Copy, Debug, Default)]
pub struct WINBIO_ENGINE_INTERFACE {
    pub Version: WINBIO_ADAPTER_INTERFACE_VERSION,
    pub Type: u32,
    pub Size: usize,
    pub AdapterId: windows_core::GUID,
    pub Attach: PIBIO_ENGINE_ATTACH_FN,
    pub Detach: PIBIO_ENGINE_DETACH_FN,
    pub ClearContext: PIBIO_ENGINE_CLEAR_CONTEXT_FN,
    pub QueryPreferredFormat: PIBIO_ENGINE_QUERY_PREFERRED_FORMAT_FN,
    pub QueryIndexVectorSize: PIBIO_ENGINE_QUERY_INDEX_VECTOR_SIZE_FN,
    pub QueryHashAlgorithms: PIBIO_ENGINE_QUERY_HASH_ALGORITHMS_FN,
    pub SetHashAlgorithm: PIBIO_ENGINE_SET_HASH_ALGORITHM_FN,
    pub QuerySampleHint: PIBIO_ENGINE_QUERY_SAMPLE_HINT_FN,
    pub AcceptSampleData: PIBIO_ENGINE_ACCEPT_SAMPLE_DATA_FN,
    pub ExportEngineData: PIBIO_ENGINE_EXPORT_ENGINE_DATA_FN,
    pub VerifyFeatureSet: PIBIO_ENGINE_VERIFY_FEATURE_SET_FN,
    pub IdentifyFeatureSet: PIBIO_ENGINE_IDENTIFY_FEATURE_SET_FN,
    pub CreateEnrollment: PIBIO_ENGINE_CREATE_ENROLLMENT_FN,
    pub UpdateEnrollment: PIBIO_ENGINE_UPDATE_ENROLLMENT_FN,
    pub GetEnrollmentStatus: PIBIO_ENGINE_GET_ENROLLMENT_STATUS_FN,
    pub GetEnrollmentHash: PIBIO_ENGINE_GET_ENROLLMENT_HASH_FN,
    pub CheckForDuplicate: PIBIO_ENGINE_CHECK_FOR_DUPLICATE_FN,
    pub CommitEnrollment: PIBIO_ENGINE_COMMIT_ENROLLMENT_FN,
    pub DiscardEnrollment: PIBIO_ENGINE_DISCARD_ENROLLMENT_FN,
    pub ControlUnit: PIBIO_ENGINE_CONTROL_UNIT_FN,
    pub ControlUnitPrivileged: PIBIO_ENGINE_CONTROL_UNIT_PRIVILEGED_FN,
    pub NotifyPowerChange: PIBIO_ENGINE_NOTIFY_POWER_CHANGE_FN,
    pub Reserved_1: PIBIO_ENGINE_RESERVED_1_FN,
    pub PipelineInit: PIBIO_ENGINE_PIPELINE_INIT_FN,
    pub PipelineCleanup: PIBIO_ENGINE_PIPELINE_CLEANUP_FN,
    pub Activate: PIBIO_ENGINE_ACTIVATE_FN,
    pub Deactivate: PIBIO_ENGINE_DEACTIVATE_FN,
    pub QueryExtendedInfo: PIBIO_ENGINE_QUERY_EXTENDED_INFO_FN,
    pub IdentifyAll: PIBIO_ENGINE_IDENTIFY_ALL_FN,
    pub SetEnrollmentSelector: PIBIO_ENGINE_SET_ENROLLMENT_SELECTOR_FN,
    pub SetEnrollmentParameters: PIBIO_ENGINE_SET_ENROLLMENT_PARAMETERS_FN,
    pub QueryExtendedEnrollmentStatus: PIBIO_ENGINE_QUERY_EXTENDED_ENROLLMENT_STATUS_FN,
    pub RefreshCache: PIBIO_ENGINE_REFRESH_CACHE_FN,
    pub SelectCalibrationFormat: PIBIO_ENGINE_SELECT_CALIBRATION_FORMAT_FN,
    pub QueryCalibrationData: PIBIO_ENGINE_QUERY_CALIBRATION_DATA_FN,
    pub SetAccountPolicy: PIBIO_ENGINE_SET_ACCOUNT_POLICY_FN,
    pub CreateKey: PIBIO_ENGINE_CREATE_KEY_FN,
    pub IdentifyFeatureSetSecure: PIBIO_ENGINE_IDENTIFY_FEATURE_SET_SECURE_FN,
    pub AcceptPrivateSensorTypeInfo: PIBIO_ENGINE_ACCEPT_PRIVATE_SENSOR_TYPE_INFO_FN,
    pub CreateEnrollmentAuthenticated: PIBIO_ENGINE_CREATE_ENROLLMENT_AUTHENTICATED_FN,
    pub IdentifyFeatureSetAuthenticated: PIBIO_ENGINE_IDENTIFY_FEATURE_SET_AUTHENTICATED_FN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_EVENT {
    pub Type: u32,
    pub Parameters: WINBIO_EVENT_0,
}
impl Default for WINBIO_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINBIO_EVENT_0 {
    pub Unclaimed: WINBIO_EVENT_0_0,
    pub UnclaimedIdentify: WINBIO_EVENT_0_1,
    pub Error: WINBIO_EVENT_0_2,
}
impl Default for WINBIO_EVENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EVENT_0_2 {
    pub ErrorCode: windows_core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_EVENT_0_1 {
    pub UnitId: u32,
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub RejectDetail: u32,
}
impl Default for WINBIO_EVENT_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EVENT_0_0 {
    pub UnitId: u32,
    pub RejectDetail: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_EXTENDED_ENGINE_INFO {
    pub GenericEngineCapabilities: u32,
    pub Factor: u32,
    pub Specific: WINBIO_EXTENDED_ENGINE_INFO_0,
}
impl Default for WINBIO_EXTENDED_ENGINE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINBIO_EXTENDED_ENGINE_INFO_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_ENGINE_INFO_0_0,
    pub Fingerprint: WINBIO_EXTENDED_ENGINE_INFO_0_1,
    pub Iris: WINBIO_EXTENDED_ENGINE_INFO_0_2,
    pub Voice: WINBIO_EXTENDED_ENGINE_INFO_0_3,
}
impl Default for WINBIO_EXTENDED_ENGINE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    pub Capabilities: u32,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_0_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    pub Null: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    pub Capabilities: u32,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_1_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    pub GeneralSamples: u32,
    pub Center: u32,
    pub TopEdge: u32,
    pub BottomEdge: u32,
    pub LeftEdge: u32,
    pub RightEdge: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    pub Capabilities: u32,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_2_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    pub Null: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    pub Capabilities: u32,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_3_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    pub Null: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    pub Size: usize,
    pub SubFactor: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS {
    pub TemplateStatus: windows_core::HRESULT,
    pub RejectDetail: u32,
    pub PercentComplete: u32,
    pub Factor: u32,
    pub SubFactor: u8,
    pub Specific: WINBIO_EXTENDED_ENROLLMENT_STATUS_0,
}
impl Default for WINBIO_EXTENDED_ENROLLMENT_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0,
    pub Fingerprint: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1,
    pub Iris: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2,
    pub Voice: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3,
}
impl Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    pub BoundingBox: super::super::Foundation::RECT,
    pub Distance: i32,
    pub OpaqueEngineData: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    pub AdapterId: windows_core::GUID,
    pub Data: [u32; 78],
}
impl Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    pub GeneralSamples: u32,
    pub Center: u32,
    pub TopEdge: u32,
    pub BottomEdge: u32,
    pub LeftEdge: u32,
    pub RightEdge: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {
    pub EyeBoundingBox_1: super::super::Foundation::RECT,
    pub EyeBoundingBox_2: super::super::Foundation::RECT,
    pub PupilCenter_1: super::super::Foundation::POINT,
    pub PupilCenter_2: super::super::Foundation::POINT,
    pub Distance: i32,
    pub GridPointCompletionPercent: u32,
    pub GridPointIndex: u16,
    pub Point3D: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0,
    pub StopCaptureAndShowCriticalFeedback: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_EXTENDED_SENSOR_INFO {
    pub GenericSensorCapabilities: u32,
    pub Factor: u32,
    pub Specific: WINBIO_EXTENDED_SENSOR_INFO_0,
}
impl Default for WINBIO_EXTENDED_SENSOR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINBIO_EXTENDED_SENSOR_INFO_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_SENSOR_INFO_0_0,
    pub Fingerprint: WINBIO_EXTENDED_SENSOR_INFO_0_1,
    pub Iris: WINBIO_EXTENDED_SENSOR_INFO_0_2,
    pub Voice: WINBIO_EXTENDED_SENSOR_INFO_0_3,
}
impl Default for WINBIO_EXTENDED_SENSOR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    pub FrameSize: super::super::Foundation::RECT,
    pub FrameOffset: super::super::Foundation::POINT,
    pub MandatoryOrientation: u32,
    pub HardwareInfo: WINBIO_EXTENDED_SENSOR_INFO_0_0_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    pub ColorSensorId: [u16; 260],
    pub InfraredSensorId: [u16; 260],
    pub InfraredSensorRotationAngle: u32,
}
impl Default for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    pub FrameSize: super::super::Foundation::RECT,
    pub FrameOffset: super::super::Foundation::POINT,
    pub MandatoryOrientation: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_EXTENDED_STORAGE_INFO {
    pub GenericStorageCapabilities: u32,
    pub Factor: u32,
    pub Specific: WINBIO_EXTENDED_STORAGE_INFO_0,
}
impl Default for WINBIO_EXTENDED_STORAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINBIO_EXTENDED_STORAGE_INFO_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_STORAGE_INFO_0_0,
    pub Fingerprint: WINBIO_EXTENDED_STORAGE_INFO_0_1,
    pub Iris: WINBIO_EXTENDED_STORAGE_INFO_0_2,
    pub Voice: WINBIO_EXTENDED_STORAGE_INFO_0_3,
}
impl Default for WINBIO_EXTENDED_STORAGE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    pub Capabilities: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    pub Capabilities: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    pub Capabilities: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    pub Capabilities: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_EXTENDED_UNIT_STATUS {
    pub Availability: u32,
    pub ReasonCode: u32,
}
pub const WINBIO_E_ADAPTER_INTEGRITY_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8009803D_u32 as _);
pub const WINBIO_E_AUTO_LOGON_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80098043_u32 as _);
pub const WINBIO_E_BAD_CAPTURE: windows_core::HRESULT = windows_core::HRESULT(0x80098008_u32 as _);
pub const WINBIO_E_CALIBRATION_BUFFER_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80098051_u32 as _);
pub const WINBIO_E_CALIBRATION_BUFFER_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x80098050_u32 as _);
pub const WINBIO_E_CALIBRATION_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x8009804F_u32 as _);
pub const WINBIO_E_CANCELED: windows_core::HRESULT = windows_core::HRESULT(0x80098004_u32 as _);
pub const WINBIO_E_CAPTURE_ABORTED: windows_core::HRESULT = windows_core::HRESULT(0x80098006_u32 as _);
pub const WINBIO_E_CONFIGURATION_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80098033_u32 as _);
pub const WINBIO_E_CRED_PROV_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80098030_u32 as _);
pub const WINBIO_E_CRED_PROV_NO_CREDENTIAL: windows_core::HRESULT = windows_core::HRESULT(0x80098031_u32 as _);
pub const WINBIO_E_CRED_PROV_SECURITY_LOCKOUT: windows_core::HRESULT = windows_core::HRESULT(0x80098047_u32 as _);
pub const WINBIO_E_DATABASE_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80098016_u32 as _);
pub const WINBIO_E_DATABASE_BAD_INDEX_VECTOR: windows_core::HRESULT = windows_core::HRESULT(0x80098022_u32 as _);
pub const WINBIO_E_DATABASE_CANT_CLOSE: windows_core::HRESULT = windows_core::HRESULT(0x80098013_u32 as _);
pub const WINBIO_E_DATABASE_CANT_CREATE: windows_core::HRESULT = windows_core::HRESULT(0x80098011_u32 as _);
pub const WINBIO_E_DATABASE_CANT_ERASE: windows_core::HRESULT = windows_core::HRESULT(0x80098014_u32 as _);
pub const WINBIO_E_DATABASE_CANT_FIND: windows_core::HRESULT = windows_core::HRESULT(0x80098015_u32 as _);
pub const WINBIO_E_DATABASE_CANT_OPEN: windows_core::HRESULT = windows_core::HRESULT(0x80098012_u32 as _);
pub const WINBIO_E_DATABASE_CORRUPTED: windows_core::HRESULT = windows_core::HRESULT(0x8009801A_u32 as _);
pub const WINBIO_E_DATABASE_EOF: windows_core::HRESULT = windows_core::HRESULT(0x80098021_u32 as _);
pub const WINBIO_E_DATABASE_FULL: windows_core::HRESULT = windows_core::HRESULT(0x80098018_u32 as _);
pub const WINBIO_E_DATABASE_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80098019_u32 as _);
pub const WINBIO_E_DATABASE_NO_MORE_RECORDS: windows_core::HRESULT = windows_core::HRESULT(0x80098020_u32 as _);
pub const WINBIO_E_DATABASE_NO_RESULTS: windows_core::HRESULT = windows_core::HRESULT(0x8009801F_u32 as _);
pub const WINBIO_E_DATABASE_NO_SUCH_RECORD: windows_core::HRESULT = windows_core::HRESULT(0x8009801B_u32 as _);
pub const WINBIO_E_DATABASE_READ_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8009801D_u32 as _);
pub const WINBIO_E_DATABASE_WRITE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8009801E_u32 as _);
pub const WINBIO_E_DATA_COLLECTION_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x8009800B_u32 as _);
pub const WINBIO_E_DATA_PROTECTION_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80098046_u32 as _);
pub const WINBIO_E_DEADLOCK_DETECTED: windows_core::HRESULT = windows_core::HRESULT(0x80098040_u32 as _);
pub const WINBIO_E_DEVICE_BUSY: windows_core::HRESULT = windows_core::HRESULT(0x80098010_u32 as _);
pub const WINBIO_E_DEVICE_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80098036_u32 as _);
pub const WINBIO_E_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80098032_u32 as _);
pub const WINBIO_E_DUPLICATE_ENROLLMENT: windows_core::HRESULT = windows_core::HRESULT(0x8009801C_u32 as _);
pub const WINBIO_E_DUPLICATE_TEMPLATE: windows_core::HRESULT = windows_core::HRESULT(0x8009802B_u32 as _);
pub const WINBIO_E_ENROLLMENT_CANCELED_BY_SUSPEND: windows_core::HRESULT = windows_core::HRESULT(0x8009805B_u32 as _);
pub const WINBIO_E_ENROLLMENT_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80098007_u32 as _);
pub const WINBIO_E_EVENT_MONITOR_ACTIVE: windows_core::HRESULT = windows_core::HRESULT(0x80098039_u32 as _);
pub const WINBIO_E_FAST_USER_SWITCH_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80098037_u32 as _);
pub const WINBIO_E_INCORRECT_BSP: windows_core::HRESULT = windows_core::HRESULT(0x80098024_u32 as _);
pub const WINBIO_E_INCORRECT_SENSOR_POOL: windows_core::HRESULT = windows_core::HRESULT(0x80098025_u32 as _);
pub const WINBIO_E_INCORRECT_SESSION_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8009803E_u32 as _);
pub const WINBIO_E_INSECURE_SENSOR: windows_core::HRESULT = windows_core::HRESULT(0x80098057_u32 as _);
pub const WINBIO_E_INVALID_BUFFER: windows_core::HRESULT = windows_core::HRESULT(0x80098059_u32 as _);
pub const WINBIO_E_INVALID_BUFFER_ID: windows_core::HRESULT = windows_core::HRESULT(0x80098058_u32 as _);
pub const WINBIO_E_INVALID_CALIBRATION_FORMAT_ARRAY: windows_core::HRESULT = windows_core::HRESULT(0x8009804C_u32 as _);
pub const WINBIO_E_INVALID_CONTROL_CODE: windows_core::HRESULT = windows_core::HRESULT(0x80098009_u32 as _);
pub const WINBIO_E_INVALID_DEVICE_STATE: windows_core::HRESULT = windows_core::HRESULT(0x8009800F_u32 as _);
pub const WINBIO_E_INVALID_KEY_IDENTIFIER: windows_core::HRESULT = windows_core::HRESULT(0x80098052_u32 as _);
pub const WINBIO_E_INVALID_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0x8009802C_u32 as _);
pub const WINBIO_E_INVALID_PROPERTY_ID: windows_core::HRESULT = windows_core::HRESULT(0x8009803B_u32 as _);
pub const WINBIO_E_INVALID_PROPERTY_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8009803A_u32 as _);
pub const WINBIO_E_INVALID_SENSOR_MODE: windows_core::HRESULT = windows_core::HRESULT(0x80098027_u32 as _);
pub const WINBIO_E_INVALID_SUBFACTOR: windows_core::HRESULT = windows_core::HRESULT(0x8009804B_u32 as _);
pub const WINBIO_E_INVALID_TICKET: windows_core::HRESULT = windows_core::HRESULT(0x80098044_u32 as _);
pub const WINBIO_E_INVALID_UNIT: windows_core::HRESULT = windows_core::HRESULT(0x80098002_u32 as _);
pub const WINBIO_E_KEY_CREATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80098053_u32 as _);
pub const WINBIO_E_KEY_IDENTIFIER_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x80098054_u32 as _);
pub const WINBIO_E_LOCK_VIOLATION: windows_core::HRESULT = windows_core::HRESULT(0x8009802A_u32 as _);
pub const WINBIO_E_MAX_ERROR_COUNT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x80098042_u32 as _);
pub const WINBIO_E_NOT_ACTIVE_CONSOLE: windows_core::HRESULT = windows_core::HRESULT(0x80098038_u32 as _);
pub const WINBIO_E_NO_CAPTURE_DATA: windows_core::HRESULT = windows_core::HRESULT(0x80098026_u32 as _);
pub const WINBIO_E_NO_MATCH: windows_core::HRESULT = windows_core::HRESULT(0x80098005_u32 as _);
pub const WINBIO_E_NO_PREBOOT_IDENTITY: windows_core::HRESULT = windows_core::HRESULT(0x80098041_u32 as _);
pub const WINBIO_E_NO_SUPPORTED_CALIBRATION_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8009804D_u32 as _);
pub const WINBIO_E_POLICY_PROTECTION_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80098056_u32 as _);
pub const WINBIO_E_PRESENCE_MONITOR_ACTIVE: windows_core::HRESULT = windows_core::HRESULT(0x8009804A_u32 as _);
pub const WINBIO_E_PROPERTY_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80098055_u32 as _);
pub const WINBIO_E_SAS_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0x80098035_u32 as _);
pub const WINBIO_E_SELECTION_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80098049_u32 as _);
pub const WINBIO_E_SENSOR_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80098034_u32 as _);
pub const WINBIO_E_SESSION_BUSY: windows_core::HRESULT = windows_core::HRESULT(0x8009802D_u32 as _);
pub const WINBIO_E_SESSION_HANDLE_CLOSED: windows_core::HRESULT = windows_core::HRESULT(0x8009803F_u32 as _);
pub const WINBIO_E_TICKET_QUOTA_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x80098045_u32 as _);
pub const WINBIO_E_TRUSTLET_INTEGRITY_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x8009805A_u32 as _);
pub const WINBIO_E_UNKNOWN_ID: windows_core::HRESULT = windows_core::HRESULT(0x80098003_u32 as _);
pub const WINBIO_E_UNSUPPORTED_DATA_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8009800C_u32 as _);
pub const WINBIO_E_UNSUPPORTED_DATA_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8009800D_u32 as _);
pub const WINBIO_E_UNSUPPORTED_FACTOR: windows_core::HRESULT = windows_core::HRESULT(0x80098001_u32 as _);
pub const WINBIO_E_UNSUPPORTED_POOL_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80098048_u32 as _);
pub const WINBIO_E_UNSUPPORTED_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x8009803C_u32 as _);
pub const WINBIO_E_UNSUPPORTED_PURPOSE: windows_core::HRESULT = windows_core::HRESULT(0x8009800E_u32 as _);
pub const WINBIO_E_UNSUPPORTED_SENSOR_CALIBRATION_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8009804E_u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_FP_BU_STATE {
    pub SensorAttached: windows_core::BOOL,
    pub CreationResult: windows_core::HRESULT,
}
#[repr(C)]
#[cfg(feature = "Win32_System_IO")]
#[derive(Clone, Copy, Debug, Default)]
pub struct WINBIO_FRAMEWORK_INTERFACE {
    pub Version: WINBIO_ADAPTER_INTERFACE_VERSION,
    pub Type: u32,
    pub Size: usize,
    pub AdapterId: windows_core::GUID,
    pub SetUnitStatus: PIBIO_FRAMEWORK_SET_UNIT_STATUS_FN,
    pub VsmStorageAttach: PIBIO_STORAGE_ATTACH_FN,
    pub VsmStorageDetach: PIBIO_STORAGE_DETACH_FN,
    pub VsmStorageClearContext: PIBIO_STORAGE_CLEAR_CONTEXT_FN,
    pub VsmStorageCreateDatabase: PIBIO_STORAGE_CREATE_DATABASE_FN,
    pub VsmStorageOpenDatabase: PIBIO_STORAGE_OPEN_DATABASE_FN,
    pub VsmStorageCloseDatabase: PIBIO_STORAGE_CLOSE_DATABASE_FN,
    pub VsmStorageDeleteRecord: PIBIO_STORAGE_DELETE_RECORD_FN,
    pub VsmStorageNotifyPowerChange: PIBIO_STORAGE_NOTIFY_POWER_CHANGE_FN,
    pub VsmStoragePipelineInit: PIBIO_STORAGE_PIPELINE_INIT_FN,
    pub VsmStoragePipelineCleanup: PIBIO_STORAGE_PIPELINE_CLEANUP_FN,
    pub VsmStorageActivate: PIBIO_STORAGE_ACTIVATE_FN,
    pub VsmStorageDeactivate: PIBIO_STORAGE_DEACTIVATE_FN,
    pub VsmStorageQueryExtendedInfo: PIBIO_STORAGE_QUERY_EXTENDED_INFO_FN,
    pub VsmStorageCacheClear: PIBIO_FRAMEWORK_VSM_CACHE_CLEAR_FN,
    pub VsmStorageCacheImportBegin: PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_BEGIN_FN,
    pub VsmStorageCacheImportNext: PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_NEXT_FN,
    pub VsmStorageCacheImportEnd: PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_END_FN,
    pub VsmStorageCacheExportBegin: PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_BEGIN_FN,
    pub VsmStorageCacheExportNext: PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_NEXT_FN,
    pub VsmStorageCacheExportEnd: PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_END_FN,
    pub VsmSensorAttach: PIBIO_SENSOR_ATTACH_FN,
    pub VsmSensorDetach: PIBIO_SENSOR_DETACH_FN,
    pub VsmSensorClearContext: PIBIO_SENSOR_CLEAR_CONTEXT_FN,
    pub VsmSensorPushDataToEngine: PIBIO_SENSOR_PUSH_DATA_TO_ENGINE_FN,
    pub VsmSensorNotifyPowerChange: PIBIO_SENSOR_NOTIFY_POWER_CHANGE_FN,
    pub VsmSensorPipelineInit: PIBIO_SENSOR_PIPELINE_INIT_FN,
    pub VsmSensorPipelineCleanup: PIBIO_SENSOR_PIPELINE_CLEANUP_FN,
    pub VsmSensorActivate: PIBIO_SENSOR_ACTIVATE_FN,
    pub VsmSensorDeactivate: PIBIO_SENSOR_DEACTIVATE_FN,
    pub VsmSensorAsyncImportRawBuffer: PIBIO_SENSOR_ASYNC_IMPORT_RAW_BUFFER_FN,
    pub VsmSensorAsyncImportSecureBuffer: PIBIO_SENSOR_ASYNC_IMPORT_SECURE_BUFFER_FN,
    pub Reserved1: PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_1_FN,
    pub Reserved2: PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_2_FN,
    pub Reserved3: PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_3_FN,
    pub Reserved4: PIBIO_STORAGE_RESERVED_1_FN,
    pub Reserved5: PIBIO_STORAGE_RESERVED_2_FN,
    pub AllocateMemory: PIBIO_FRAMEWORK_ALLOCATE_MEMORY_FN,
    pub FreeMemory: PIBIO_FRAMEWORK_FREE_MEMORY_FN,
    pub GetProperty: PIBIO_FRAMEWORK_GET_PROPERTY_FN,
    pub LockAndValidateSecureBuffer: PIBIO_FRAMEWORK_LOCK_AND_VALIDATE_SECURE_BUFFER_FN,
    pub ReleaseSecureBuffer: PIBIO_FRAMEWORK_RELEASE_SECURE_BUFFER_FN,
    pub QueryAuthorizedEnrollments: PIBIO_FRAMEWORK_VSM_QUERY_AUTHORIZED_ENROLLMENTS_FN,
    pub DecryptSample: PIBIO_FRAMEWORK_VSM_DECRYPT_SAMPLE_FN,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_GESTURE_METADATA {
    pub Size: usize,
    pub BiometricType: u32,
    pub MatchType: u32,
    pub ProtectionType: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_GET_INDICATOR {
    pub PayloadSize: u32,
    pub WinBioHresult: windows_core::HRESULT,
    pub IndicatorStatus: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_IDENTITY {
    pub Type: u32,
    pub Value: WINBIO_IDENTITY_0,
}
impl Default for WINBIO_IDENTITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINBIO_IDENTITY_0 {
    pub Null: u32,
    pub Wildcard: u32,
    pub TemplateGuid: windows_core::GUID,
    pub AccountSid: WINBIO_IDENTITY_0_0,
    pub SecureId: [u8; 32],
}
impl Default for WINBIO_IDENTITY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_IDENTITY_0_0 {
    pub Size: u32,
    pub Data: [u8; 68],
}
impl Default for WINBIO_IDENTITY_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINBIO_I_EXTENDED_STATUS_INFORMATION: windows_core::HRESULT = windows_core::HRESULT(0x90002_u32 as _);
pub const WINBIO_I_MORE_DATA: windows_core::HRESULT = windows_core::HRESULT(0x90001_u32 as _);
pub const WINBIO_MAX_STRING_LEN: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_NOTIFY_WAKE {
    pub PayloadSize: u32,
    pub WinBioHresult: windows_core::HRESULT,
    pub Reason: u32,
}
pub const WINBIO_PASSWORD_GENERIC: WINBIO_CREDENTIAL_FORMAT = WINBIO_CREDENTIAL_FORMAT(1i32);
pub const WINBIO_PASSWORD_PACKED: WINBIO_CREDENTIAL_FORMAT = WINBIO_CREDENTIAL_FORMAT(2i32);
pub const WINBIO_PASSWORD_PROTECTED: WINBIO_CREDENTIAL_FORMAT = WINBIO_CREDENTIAL_FORMAT(3i32);
#[repr(C)]
#[cfg(feature = "Win32_System_IO")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_PIPELINE {
    pub SensorHandle: super::super::Foundation::HANDLE,
    pub EngineHandle: super::super::Foundation::HANDLE,
    pub StorageHandle: super::super::Foundation::HANDLE,
    pub SensorInterface: *mut WINBIO_SENSOR_INTERFACE,
    pub EngineInterface: *mut WINBIO_ENGINE_INTERFACE,
    pub StorageInterface: *mut WINBIO_STORAGE_INTERFACE,
    pub SensorContext: *mut WINIBIO_SENSOR_CONTEXT,
    pub EngineContext: *mut WINIBIO_ENGINE_CONTEXT,
    pub StorageContext: *mut WINIBIO_STORAGE_CONTEXT,
    pub FrameworkInterface: *mut WINBIO_FRAMEWORK_INTERFACE,
}
#[cfg(feature = "Win32_System_IO")]
impl Default for WINBIO_PIPELINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINBIO_POLICY_ADMIN: WINBIO_POLICY_SOURCE = WINBIO_POLICY_SOURCE(3i32);
pub const WINBIO_POLICY_DEFAULT: WINBIO_POLICY_SOURCE = WINBIO_POLICY_SOURCE(1i32);
pub const WINBIO_POLICY_LOCAL: WINBIO_POLICY_SOURCE = WINBIO_POLICY_SOURCE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINBIO_POLICY_SOURCE(pub i32);
pub const WINBIO_POLICY_UNKNOWN: WINBIO_POLICY_SOURCE = WINBIO_POLICY_SOURCE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINBIO_POOL(pub u32);
pub const WINBIO_POOL_PRIVATE: WINBIO_POOL = WINBIO_POOL(2u32);
pub const WINBIO_POOL_SYSTEM: WINBIO_POOL = WINBIO_POOL(1u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_PRESENCE {
    pub Factor: u32,
    pub SubFactor: u8,
    pub Status: windows_core::HRESULT,
    pub RejectDetail: u32,
    pub Identity: WINBIO_IDENTITY,
    pub TrackingId: u64,
    pub Ticket: u64,
    pub Properties: WINBIO_PRESENCE_PROPERTIES,
    pub Authorization: WINBIO_PRESENCE_0,
}
impl Default for WINBIO_PRESENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_PRESENCE_0 {
    pub Size: u32,
    pub Data: [u8; 32],
}
impl Default for WINBIO_PRESENCE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINBIO_PRESENCE_PROPERTIES {
    pub FacialFeatures: WINBIO_PRESENCE_PROPERTIES_0,
    pub Iris: WINBIO_PRESENCE_PROPERTIES_1,
}
impl Default for WINBIO_PRESENCE_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_PRESENCE_PROPERTIES_0 {
    pub BoundingBox: super::super::Foundation::RECT,
    pub Distance: i32,
    pub OpaqueEngineData: WINBIO_PRESENCE_PROPERTIES_0_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_PRESENCE_PROPERTIES_0_0 {
    pub AdapterId: windows_core::GUID,
    pub Data: [u32; 78],
}
impl Default for WINBIO_PRESENCE_PROPERTIES_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_PRESENCE_PROPERTIES_1 {
    pub EyeBoundingBox_1: super::super::Foundation::RECT,
    pub EyeBoundingBox_2: super::super::Foundation::RECT,
    pub PupilCenter_1: super::super::Foundation::POINT,
    pub PupilCenter_2: super::super::Foundation::POINT,
    pub Distance: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    pub PayloadSize: u32,
    pub WinBioHresult: windows_core::HRESULT,
    pub PrivateSensorTypeInfo: WINBIO_DATA,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_PROTECTION_POLICY {
    pub Version: u32,
    pub Identity: WINBIO_IDENTITY,
    pub DatabaseId: windows_core::GUID,
    pub UserState: u64,
    pub PolicySize: usize,
    pub Policy: [u8; 128],
}
impl Default for WINBIO_PROTECTION_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_REGISTERED_FORMAT {
    pub Owner: u16,
    pub Type: u16,
}
pub const WINBIO_SCP_CURVE_FIELD_SIZE_V1: u32 = 32u32;
pub const WINBIO_SCP_DIGEST_SIZE_V1: u32 = 32u32;
pub const WINBIO_SCP_ENCRYPTION_BLOCK_SIZE_V1: u32 = 16u32;
pub const WINBIO_SCP_ENCRYPTION_KEY_SIZE_V1: u32 = 32u32;
pub const WINBIO_SCP_PRIVATE_KEY_SIZE_V1: u32 = 32u32;
pub const WINBIO_SCP_PUBLIC_KEY_SIZE_V1: u32 = 65u32;
pub const WINBIO_SCP_RANDOM_SIZE_V1: u32 = 32u32;
pub const WINBIO_SCP_SIGNATURE_SIZE_V1: u32 = 64u32;
pub const WINBIO_SCP_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_SECURE_BUFFER_HEADER_V1 {
    pub Type: u32,
    pub Size: u32,
    pub Flags: u32,
    pub ValidationTag: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_SECURE_CONNECTION_DATA {
    pub Size: u32,
    pub Version: u16,
    pub Flags: u16,
    pub ModelCertificateSize: u32,
    pub IntermediateCA1Size: u32,
    pub IntermediateCA2Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_SECURE_CONNECTION_PARAMS {
    pub PayloadSize: u32,
    pub Version: u16,
    pub Flags: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_SENSOR_ATTRIBUTES {
    pub PayloadSize: u32,
    pub WinBioHresult: windows_core::HRESULT,
    pub WinBioVersion: WINBIO_VERSION,
    pub SensorType: u32,
    pub SensorSubType: u32,
    pub Capabilities: u32,
    pub ManufacturerName: [u16; 256],
    pub ModelName: [u16; 256],
    pub SerialNumber: [u16; 256],
    pub FirmwareVersion: WINBIO_VERSION,
    pub SupportedFormatEntries: u32,
    pub SupportedFormat: [WINBIO_REGISTERED_FORMAT; 1],
}
impl Default for WINBIO_SENSOR_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_IO")]
#[derive(Clone, Copy, Debug, Default)]
pub struct WINBIO_SENSOR_INTERFACE {
    pub Version: WINBIO_ADAPTER_INTERFACE_VERSION,
    pub Type: u32,
    pub Size: usize,
    pub AdapterId: windows_core::GUID,
    pub Attach: PIBIO_SENSOR_ATTACH_FN,
    pub Detach: PIBIO_SENSOR_DETACH_FN,
    pub ClearContext: PIBIO_SENSOR_CLEAR_CONTEXT_FN,
    pub QueryStatus: PIBIO_SENSOR_QUERY_STATUS_FN,
    pub Reset: PIBIO_SENSOR_RESET_FN,
    pub SetMode: PIBIO_SENSOR_SET_MODE_FN,
    pub SetIndicatorStatus: PIBIO_SENSOR_SET_INDICATOR_STATUS_FN,
    pub GetIndicatorStatus: PIBIO_SENSOR_GET_INDICATOR_STATUS_FN,
    pub StartCapture: PIBIO_SENSOR_START_CAPTURE_FN,
    pub FinishCapture: PIBIO_SENSOR_FINISH_CAPTURE_FN,
    pub ExportSensorData: PIBIO_SENSOR_EXPORT_SENSOR_DATA_FN,
    pub Cancel: PIBIO_SENSOR_CANCEL_FN,
    pub PushDataToEngine: PIBIO_SENSOR_PUSH_DATA_TO_ENGINE_FN,
    pub ControlUnit: PIBIO_SENSOR_CONTROL_UNIT_FN,
    pub ControlUnitPrivileged: PIBIO_SENSOR_CONTROL_UNIT_PRIVILEGED_FN,
    pub NotifyPowerChange: PIBIO_SENSOR_NOTIFY_POWER_CHANGE_FN,
    pub PipelineInit: PIBIO_SENSOR_PIPELINE_INIT_FN,
    pub PipelineCleanup: PIBIO_SENSOR_PIPELINE_CLEANUP_FN,
    pub Activate: PIBIO_SENSOR_ACTIVATE_FN,
    pub Deactivate: PIBIO_SENSOR_DEACTIVATE_FN,
    pub QueryExtendedInfo: PIBIO_SENSOR_QUERY_EXTENDED_INFO_FN,
    pub QueryCalibrationFormats: PIBIO_SENSOR_QUERY_CALIBRATION_FORMATS_FN,
    pub SetCalibrationFormat: PIBIO_SENSOR_SET_CALIBRATION_FORMAT_FN,
    pub AcceptCalibrationData: PIBIO_SENSOR_ACCEPT_CALIBRATION_DATA_FN,
    pub AsyncImportRawBuffer: PIBIO_SENSOR_ASYNC_IMPORT_RAW_BUFFER_FN,
    pub AsyncImportSecureBuffer: PIBIO_SENSOR_ASYNC_IMPORT_SECURE_BUFFER_FN,
    pub QueryPrivateSensorType: PIBIO_SENSOR_QUERY_PRIVATE_SENSOR_TYPE_FN,
    pub ConnectSecure: PIBIO_SENSOR_CONNECT_SECURE_FN,
    pub StartCaptureEx: PIBIO_SENSOR_START_CAPTURE_EX_FN,
    pub StartNotifyWake: PIBIO_SENSOR_START_NOTIFY_WAKE_FN,
    pub FinishNotifyWake: PIBIO_SENSOR_FINISH_NOTIFY_WAKE_FN,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINBIO_SETTING_SOURCE(pub u32);
pub const WINBIO_SETTING_SOURCE_DEFAULT: WINBIO_SETTING_SOURCE = WINBIO_SETTING_SOURCE(1u32);
pub const WINBIO_SETTING_SOURCE_INVALID: WINBIO_SETTING_SOURCE = WINBIO_SETTING_SOURCE(0u32);
pub const WINBIO_SETTING_SOURCE_LOCAL: WINBIO_SETTING_SOURCE = WINBIO_SETTING_SOURCE(3u32);
pub const WINBIO_SETTING_SOURCE_POLICY: WINBIO_SETTING_SOURCE = WINBIO_SETTING_SOURCE(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_SET_INDICATOR {
    pub PayloadSize: u32,
    pub IndicatorStatus: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_IO")]
#[derive(Clone, Copy, Debug, Default)]
pub struct WINBIO_STORAGE_INTERFACE {
    pub Version: WINBIO_ADAPTER_INTERFACE_VERSION,
    pub Type: u32,
    pub Size: usize,
    pub AdapterId: windows_core::GUID,
    pub Attach: PIBIO_STORAGE_ATTACH_FN,
    pub Detach: PIBIO_STORAGE_DETACH_FN,
    pub ClearContext: PIBIO_STORAGE_CLEAR_CONTEXT_FN,
    pub CreateDatabase: PIBIO_STORAGE_CREATE_DATABASE_FN,
    pub EraseDatabase: PIBIO_STORAGE_ERASE_DATABASE_FN,
    pub OpenDatabase: PIBIO_STORAGE_OPEN_DATABASE_FN,
    pub CloseDatabase: PIBIO_STORAGE_CLOSE_DATABASE_FN,
    pub GetDataFormat: PIBIO_STORAGE_GET_DATA_FORMAT_FN,
    pub GetDatabaseSize: PIBIO_STORAGE_GET_DATABASE_SIZE_FN,
    pub AddRecord: PIBIO_STORAGE_ADD_RECORD_FN,
    pub DeleteRecord: PIBIO_STORAGE_DELETE_RECORD_FN,
    pub QueryBySubject: PIBIO_STORAGE_QUERY_BY_SUBJECT_FN,
    pub QueryByContent: PIBIO_STORAGE_QUERY_BY_CONTENT_FN,
    pub GetRecordCount: PIBIO_STORAGE_GET_RECORD_COUNT_FN,
    pub FirstRecord: PIBIO_STORAGE_FIRST_RECORD_FN,
    pub NextRecord: PIBIO_STORAGE_NEXT_RECORD_FN,
    pub GetCurrentRecord: PIBIO_STORAGE_GET_CURRENT_RECORD_FN,
    pub ControlUnit: PIBIO_STORAGE_CONTROL_UNIT_FN,
    pub ControlUnitPrivileged: PIBIO_STORAGE_CONTROL_UNIT_PRIVILEGED_FN,
    pub NotifyPowerChange: PIBIO_STORAGE_NOTIFY_POWER_CHANGE_FN,
    pub PipelineInit: PIBIO_STORAGE_PIPELINE_INIT_FN,
    pub PipelineCleanup: PIBIO_STORAGE_PIPELINE_CLEANUP_FN,
    pub Activate: PIBIO_STORAGE_ACTIVATE_FN,
    pub Deactivate: PIBIO_STORAGE_DEACTIVATE_FN,
    pub QueryExtendedInfo: PIBIO_STORAGE_QUERY_EXTENDED_INFO_FN,
    pub NotifyDatabaseChange: PIBIO_STORAGE_NOTIFY_DATABASE_CHANGE_FN,
    pub Reserved1: PIBIO_STORAGE_RESERVED_1_FN,
    pub Reserved2: PIBIO_STORAGE_RESERVED_2_FN,
    pub UpdateRecordBegin: PIBIO_STORAGE_UPDATE_RECORD_BEGIN_FN,
    pub UpdateRecordCommit: PIBIO_STORAGE_UPDATE_RECORD_COMMIT_FN,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_STORAGE_RECORD {
    pub Identity: *mut WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub IndexVector: *mut u32,
    pub IndexElementCount: usize,
    pub TemplateBlob: *mut u8,
    pub TemplateBlobSize: usize,
    pub PayloadBlob: *mut u8,
    pub PayloadBlobSize: usize,
}
impl Default for WINBIO_STORAGE_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_STORAGE_SCHEMA {
    pub BiometricFactor: u32,
    pub DatabaseId: windows_core::GUID,
    pub DataFormat: windows_core::GUID,
    pub Attributes: u32,
    pub FilePath: [u16; 256],
    pub ConnectionString: [u16; 256],
}
impl Default for WINBIO_STORAGE_SCHEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_SUPPORTED_ALGORITHMS {
    pub PayloadSize: u32,
    pub WinBioHresult: windows_core::HRESULT,
    pub NumberOfAlgorithms: u32,
    pub AlgorithmData: WINBIO_DATA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_UNIT_SCHEMA {
    pub UnitId: u32,
    pub PoolType: u32,
    pub BiometricFactor: u32,
    pub SensorSubType: u32,
    pub Capabilities: u32,
    pub DeviceInstanceId: [u16; 256],
    pub Description: [u16; 256],
    pub Manufacturer: [u16; 256],
    pub Model: [u16; 256],
    pub SerialNumber: [u16; 256],
    pub FirmwareVersion: WINBIO_VERSION,
}
impl Default for WINBIO_UNIT_SCHEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_UPDATE_FIRMWARE {
    pub PayloadSize: u32,
    pub FirmwareData: WINBIO_DATA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_VERSION {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
}
pub const WINBIO_WBDI_MAJOR_VERSION: u32 = 1u32;
pub const WINBIO_WBDI_MINOR_VERSION: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WINIBIO_ENGINE_CONTEXT(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WINIBIO_SENSOR_CONTEXT(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WINIBIO_STORAGE_CONTEXT(pub isize);
