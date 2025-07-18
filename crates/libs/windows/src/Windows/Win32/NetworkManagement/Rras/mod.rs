#[inline]
pub unsafe fn MgmAddGroupMembershipEntry(hprotocol: super::super::Foundation::HANDLE, dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopipaddr: u32, dwflags: u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn MgmAddGroupMembershipEntry(hprotocol : super::super::Foundation:: HANDLE, dwsourceaddr : u32, dwsourcemask : u32, dwgroupaddr : u32, dwgroupmask : u32, dwifindex : u32, dwifnexthopipaddr : u32, dwflags : u32) -> u32);
    unsafe { MgmAddGroupMembershipEntry(hprotocol, dwsourceaddr, dwsourcemask, dwgroupaddr, dwgroupmask, dwifindex, dwifnexthopipaddr, dwflags) }
}
#[inline]
pub unsafe fn MgmDeRegisterMProtocol(hprotocol: super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("rtm.dll" "system" fn MgmDeRegisterMProtocol(hprotocol : super::super::Foundation:: HANDLE) -> u32);
    unsafe { MgmDeRegisterMProtocol(hprotocol) }
}
#[inline]
pub unsafe fn MgmDeleteGroupMembershipEntry(hprotocol: super::super::Foundation::HANDLE, dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopipaddr: u32, dwflags: u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn MgmDeleteGroupMembershipEntry(hprotocol : super::super::Foundation:: HANDLE, dwsourceaddr : u32, dwsourcemask : u32, dwgroupaddr : u32, dwgroupmask : u32, dwifindex : u32, dwifnexthopipaddr : u32, dwflags : u32) -> u32);
    unsafe { MgmDeleteGroupMembershipEntry(hprotocol, dwsourceaddr, dwsourcemask, dwgroupaddr, dwgroupmask, dwifindex, dwifnexthopipaddr, dwflags) }
}
#[inline]
pub unsafe fn MgmGetFirstMfe(pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn MgmGetFirstMfe(pdwbuffersize : *mut u32, pbbuffer : *mut u8, pdwnumentries : *mut u32) -> u32);
    unsafe { MgmGetFirstMfe(pdwbuffersize as _, pbbuffer as _, pdwnumentries as _) }
}
#[inline]
pub unsafe fn MgmGetFirstMfeStats(pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32, dwflags: u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn MgmGetFirstMfeStats(pdwbuffersize : *mut u32, pbbuffer : *mut u8, pdwnumentries : *mut u32, dwflags : u32) -> u32);
    unsafe { MgmGetFirstMfeStats(pdwbuffersize as _, pbbuffer as _, pdwnumentries as _, dwflags) }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[inline]
pub unsafe fn MgmGetMfe(pimm: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8) -> u32 {
    windows_link::link!("rtm.dll" "system" fn MgmGetMfe(pimm : *mut super::IpHelper:: MIB_IPMCAST_MFE, pdwbuffersize : *mut u32, pbbuffer : *mut u8) -> u32);
    unsafe { MgmGetMfe(pimm as _, pdwbuffersize as _, pbbuffer as _) }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[inline]
pub unsafe fn MgmGetMfeStats(pimm: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, dwflags: u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn MgmGetMfeStats(pimm : *mut super::IpHelper:: MIB_IPMCAST_MFE, pdwbuffersize : *mut u32, pbbuffer : *mut u8, dwflags : u32) -> u32);
    unsafe { MgmGetMfeStats(pimm as _, pdwbuffersize as _, pbbuffer as _, dwflags) }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[inline]
pub unsafe fn MgmGetNextMfe(pimmstart: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn MgmGetNextMfe(pimmstart : *mut super::IpHelper:: MIB_IPMCAST_MFE, pdwbuffersize : *mut u32, pbbuffer : *mut u8, pdwnumentries : *mut u32) -> u32);
    unsafe { MgmGetNextMfe(pimmstart as _, pdwbuffersize as _, pbbuffer as _, pdwnumentries as _) }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[inline]
pub unsafe fn MgmGetNextMfeStats(pimmstart: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32, dwflags: u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn MgmGetNextMfeStats(pimmstart : *mut super::IpHelper:: MIB_IPMCAST_MFE, pdwbuffersize : *mut u32, pbbuffer : *mut u8, pdwnumentries : *mut u32, dwflags : u32) -> u32);
    unsafe { MgmGetNextMfeStats(pimmstart as _, pdwbuffersize as _, pbbuffer as _, pdwnumentries as _, dwflags) }
}
#[inline]
pub unsafe fn MgmGetProtocolOnInterface(dwifindex: u32, dwifnexthopaddr: u32, pdwifprotocolid: *mut u32, pdwifcomponentid: *mut u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn MgmGetProtocolOnInterface(dwifindex : u32, dwifnexthopaddr : u32, pdwifprotocolid : *mut u32, pdwifcomponentid : *mut u32) -> u32);
    unsafe { MgmGetProtocolOnInterface(dwifindex, dwifnexthopaddr, pdwifprotocolid as _, pdwifcomponentid as _) }
}
#[inline]
pub unsafe fn MgmGroupEnumerationEnd(henum: super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("rtm.dll" "system" fn MgmGroupEnumerationEnd(henum : super::super::Foundation:: HANDLE) -> u32);
    unsafe { MgmGroupEnumerationEnd(henum) }
}
#[inline]
pub unsafe fn MgmGroupEnumerationGetNext(henum: super::super::Foundation::HANDLE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn MgmGroupEnumerationGetNext(henum : super::super::Foundation:: HANDLE, pdwbuffersize : *mut u32, pbbuffer : *mut u8, pdwnumentries : *mut u32) -> u32);
    unsafe { MgmGroupEnumerationGetNext(henum, pdwbuffersize as _, pbbuffer as _, pdwnumentries as _) }
}
#[inline]
pub unsafe fn MgmGroupEnumerationStart(hprotocol: super::super::Foundation::HANDLE, metenumtype: MGM_ENUM_TYPES, phenumhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("rtm.dll" "system" fn MgmGroupEnumerationStart(hprotocol : super::super::Foundation:: HANDLE, metenumtype : MGM_ENUM_TYPES, phenumhandle : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { MgmGroupEnumerationStart(hprotocol, metenumtype, phenumhandle as _) }
}
#[inline]
pub unsafe fn MgmRegisterMProtocol(prpiinfo: *mut ROUTING_PROTOCOL_CONFIG, dwprotocolid: u32, dwcomponentid: u32, phprotocol: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("rtm.dll" "system" fn MgmRegisterMProtocol(prpiinfo : *mut ROUTING_PROTOCOL_CONFIG, dwprotocolid : u32, dwcomponentid : u32, phprotocol : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { MgmRegisterMProtocol(prpiinfo as _, dwprotocolid, dwcomponentid, phprotocol as _) }
}
#[inline]
pub unsafe fn MgmReleaseInterfaceOwnership(hprotocol: super::super::Foundation::HANDLE, dwifindex: u32, dwifnexthopaddr: u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn MgmReleaseInterfaceOwnership(hprotocol : super::super::Foundation:: HANDLE, dwifindex : u32, dwifnexthopaddr : u32) -> u32);
    unsafe { MgmReleaseInterfaceOwnership(hprotocol, dwifindex, dwifnexthopaddr) }
}
#[inline]
pub unsafe fn MgmTakeInterfaceOwnership(hprotocol: super::super::Foundation::HANDLE, dwifindex: u32, dwifnexthopaddr: u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn MgmTakeInterfaceOwnership(hprotocol : super::super::Foundation:: HANDLE, dwifindex : u32, dwifnexthopaddr : u32) -> u32);
    unsafe { MgmTakeInterfaceOwnership(hprotocol, dwifindex, dwifnexthopaddr) }
}
#[inline]
pub unsafe fn MprAdminBufferFree(pbuffer: *const core::ffi::c_void) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminBufferFree(pbuffer : *const core::ffi::c_void) -> u32);
    unsafe { MprAdminBufferFree(pbuffer) }
}
#[inline]
pub unsafe fn MprAdminConnectionClearStats(hrasserver: isize, hrasconnection: super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminConnectionClearStats(hrasserver : isize, hrasconnection : super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprAdminConnectionClearStats(hrasserver, hrasconnection) }
}
#[inline]
pub unsafe fn MprAdminConnectionEnum(hrasserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*const u32>) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminConnectionEnum(hrasserver : isize, dwlevel : u32, lplpbbuffer : *mut *mut u8, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *const u32) -> u32);
    unsafe { MprAdminConnectionEnum(hrasserver, dwlevel, lplpbbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MprAdminConnectionEnumEx(hrasserver: isize, pobjectheader: *const MPRAPI_OBJECT_HEADER, dwpreferedmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, pprasconn: *mut *mut RAS_CONNECTION_EX, lpdwresumehandle: *const u32) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminConnectionEnumEx(hrasserver : isize, pobjectheader : *const MPRAPI_OBJECT_HEADER, dwpreferedmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, pprasconn : *mut *mut RAS_CONNECTION_EX, lpdwresumehandle : *const u32) -> u32);
    unsafe { MprAdminConnectionEnumEx(hrasserver, pobjectheader, dwpreferedmaxlen, lpdwentriesread as _, lpdwtotalentries as _, pprasconn as _, lpdwresumehandle) }
}
#[inline]
pub unsafe fn MprAdminConnectionGetInfo(hrasserver: isize, dwlevel: u32, hrasconnection: super::super::Foundation::HANDLE, lplpbbuffer: *mut *mut u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminConnectionGetInfo(hrasserver : isize, dwlevel : u32, hrasconnection : super::super::Foundation:: HANDLE, lplpbbuffer : *mut *mut u8) -> u32);
    unsafe { MprAdminConnectionGetInfo(hrasserver, dwlevel, hrasconnection, lplpbbuffer as _) }
}
#[inline]
pub unsafe fn MprAdminConnectionGetInfoEx(hrasserver: isize, hrasconnection: super::super::Foundation::HANDLE, prasconnection: *mut RAS_CONNECTION_EX) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminConnectionGetInfoEx(hrasserver : isize, hrasconnection : super::super::Foundation:: HANDLE, prasconnection : *mut RAS_CONNECTION_EX) -> u32);
    unsafe { MprAdminConnectionGetInfoEx(hrasserver, hrasconnection, prasconnection as _) }
}
#[inline]
pub unsafe fn MprAdminConnectionRemoveQuarantine(hrasserver: super::super::Foundation::HANDLE, hrasconnection: super::super::Foundation::HANDLE, fisipaddress: bool) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminConnectionRemoveQuarantine(hrasserver : super::super::Foundation:: HANDLE, hrasconnection : super::super::Foundation:: HANDLE, fisipaddress : windows_core::BOOL) -> u32);
    unsafe { MprAdminConnectionRemoveQuarantine(hrasserver, hrasconnection, fisipaddress.into()) }
}
#[inline]
pub unsafe fn MprAdminDeregisterConnectionNotification(hmprserver: isize, heventnotification: super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminDeregisterConnectionNotification(hmprserver : isize, heventnotification : super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprAdminDeregisterConnectionNotification(hmprserver, heventnotification) }
}
#[inline]
pub unsafe fn MprAdminDeviceEnum(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, lpdwtotalentries: *mut u32) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminDeviceEnum(hmprserver : isize, dwlevel : u32, lplpbbuffer : *mut *mut u8, lpdwtotalentries : *mut u32) -> u32);
    unsafe { MprAdminDeviceEnum(hmprserver, dwlevel, lplpbbuffer as _, lpdwtotalentries as _) }
}
#[inline]
pub unsafe fn MprAdminEstablishDomainRasServer<P0, P1>(pszdomain: P0, pszmachine: P1, benable: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprAdminEstablishDomainRasServer(pszdomain : windows_core::PCWSTR, pszmachine : windows_core::PCWSTR, benable : windows_core::BOOL) -> u32);
    unsafe { MprAdminEstablishDomainRasServer(pszdomain.param().abi(), pszmachine.param().abi(), benable.into()) }
}
#[inline]
pub unsafe fn MprAdminGetErrorString(dwerror: u32, lplpwserrorstring: *mut windows_core::PWSTR) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminGetErrorString(dwerror : u32, lplpwserrorstring : *mut windows_core::PWSTR) -> u32);
    unsafe { MprAdminGetErrorString(dwerror, lplpwserrorstring as _) }
}
#[inline]
pub unsafe fn MprAdminGetPDCServer<P0, P1>(lpszdomain: P0, lpszserver: P1, lpszpdcserver: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprAdminGetPDCServer(lpszdomain : windows_core::PCWSTR, lpszserver : windows_core::PCWSTR, lpszpdcserver : windows_core::PWSTR) -> u32);
    unsafe { MprAdminGetPDCServer(lpszdomain.param().abi(), lpszserver.param().abi(), core::mem::transmute(lpszpdcserver)) }
}
#[inline]
pub unsafe fn MprAdminInterfaceConnect(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, hevent: super::super::Foundation::HANDLE, fsynchronous: bool) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceConnect(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, hevent : super::super::Foundation:: HANDLE, fsynchronous : windows_core::BOOL) -> u32);
    unsafe { MprAdminInterfaceConnect(hmprserver, hinterface, hevent, fsynchronous.into()) }
}
#[inline]
pub unsafe fn MprAdminInterfaceCreate(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8, phinterface: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceCreate(hmprserver : isize, dwlevel : u32, lpbbuffer : *const u8, phinterface : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprAdminInterfaceCreate(hmprserver, dwlevel, lpbbuffer, phinterface as _) }
}
#[inline]
pub unsafe fn MprAdminInterfaceDelete(hmprserver: isize, hinterface: super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceDelete(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprAdminInterfaceDelete(hmprserver, hinterface) }
}
#[inline]
pub unsafe fn MprAdminInterfaceDeviceGetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwindex: u32, dwlevel: u32, lplpbuffer: *mut *mut u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceDeviceGetInfo(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwindex : u32, dwlevel : u32, lplpbuffer : *mut *mut u8) -> u32);
    unsafe { MprAdminInterfaceDeviceGetInfo(hmprserver, hinterface, dwindex, dwlevel, lplpbuffer as _) }
}
#[inline]
pub unsafe fn MprAdminInterfaceDeviceSetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwindex: u32, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceDeviceSetInfo(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwindex : u32, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    unsafe { MprAdminInterfaceDeviceSetInfo(hmprserver, hinterface, dwindex, dwlevel, lpbbuffer) }
}
#[inline]
pub unsafe fn MprAdminInterfaceDisconnect(hmprserver: isize, hinterface: super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceDisconnect(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprAdminInterfaceDisconnect(hmprserver, hinterface) }
}
#[inline]
pub unsafe fn MprAdminInterfaceEnum(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*const u32>) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceEnum(hmprserver : isize, dwlevel : u32, lplpbbuffer : *mut *mut u8, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *const u32) -> u32);
    unsafe { MprAdminInterfaceEnum(hmprserver, dwlevel, lplpbbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MprAdminInterfaceGetCredentials<P0, P1>(lpwsserver: P0, lpwsinterfacename: P1, lpwsusername: Option<windows_core::PWSTR>, lpwspassword: Option<windows_core::PWSTR>, lpwsdomainname: Option<windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceGetCredentials(lpwsserver : windows_core::PCWSTR, lpwsinterfacename : windows_core::PCWSTR, lpwsusername : windows_core::PWSTR, lpwspassword : windows_core::PWSTR, lpwsdomainname : windows_core::PWSTR) -> u32);
    unsafe { MprAdminInterfaceGetCredentials(lpwsserver.param().abi(), lpwsinterfacename.param().abi(), lpwsusername.unwrap_or(core::mem::zeroed()) as _, lpwspassword.unwrap_or(core::mem::zeroed()) as _, lpwsdomainname.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MprAdminInterfaceGetCredentialsEx(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceGetCredentialsEx(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwlevel : u32, lplpbbuffer : *mut *mut u8) -> u32);
    unsafe { MprAdminInterfaceGetCredentialsEx(hmprserver, hinterface, dwlevel, lplpbbuffer as _) }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprAdminInterfaceGetCustomInfoEx(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, pcustominfo: *mut MPR_IF_CUSTOMINFOEX2) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceGetCustomInfoEx(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, pcustominfo : *mut MPR_IF_CUSTOMINFOEX2) -> u32);
    unsafe { MprAdminInterfaceGetCustomInfoEx(hmprserver, hinterface, pcustominfo as _) }
}
#[inline]
pub unsafe fn MprAdminInterfaceGetHandle<P1>(hmprserver: isize, lpwsinterfacename: P1, phinterface: *mut super::super::Foundation::HANDLE, fincludeclientinterfaces: bool) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceGetHandle(hmprserver : isize, lpwsinterfacename : windows_core::PCWSTR, phinterface : *mut super::super::Foundation:: HANDLE, fincludeclientinterfaces : windows_core::BOOL) -> u32);
    unsafe { MprAdminInterfaceGetHandle(hmprserver, lpwsinterfacename.param().abi(), phinterface as _, fincludeclientinterfaces.into()) }
}
#[inline]
pub unsafe fn MprAdminInterfaceGetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwlevel: u32, lplpbbuffer: *const *const u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceGetInfo(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwlevel : u32, lplpbbuffer : *const *const u8) -> u32);
    unsafe { MprAdminInterfaceGetInfo(hmprserver, hinterface, dwlevel, lplpbbuffer) }
}
#[inline]
pub unsafe fn MprAdminInterfaceQueryUpdateResult(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwprotocolid: u32, lpdwupdateresult: *mut u32) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceQueryUpdateResult(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwprotocolid : u32, lpdwupdateresult : *mut u32) -> u32);
    unsafe { MprAdminInterfaceQueryUpdateResult(hmprserver, hinterface, dwprotocolid, lpdwupdateresult as _) }
}
#[inline]
pub unsafe fn MprAdminInterfaceSetCredentials<P0, P1, P2, P3, P4>(lpwsserver: P0, lpwsinterfacename: P1, lpwsusername: P2, lpwsdomainname: P3, lpwspassword: P4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceSetCredentials(lpwsserver : windows_core::PCWSTR, lpwsinterfacename : windows_core::PCWSTR, lpwsusername : windows_core::PCWSTR, lpwsdomainname : windows_core::PCWSTR, lpwspassword : windows_core::PCWSTR) -> u32);
    unsafe { MprAdminInterfaceSetCredentials(lpwsserver.param().abi(), lpwsinterfacename.param().abi(), lpwsusername.param().abi(), lpwsdomainname.param().abi(), lpwspassword.param().abi()) }
}
#[inline]
pub unsafe fn MprAdminInterfaceSetCredentialsEx(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceSetCredentialsEx(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    unsafe { MprAdminInterfaceSetCredentialsEx(hmprserver, hinterface, dwlevel, lpbbuffer) }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprAdminInterfaceSetCustomInfoEx(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, pcustominfo: *const MPR_IF_CUSTOMINFOEX2) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceSetCustomInfoEx(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, pcustominfo : *const MPR_IF_CUSTOMINFOEX2) -> u32);
    unsafe { MprAdminInterfaceSetCustomInfoEx(hmprserver, hinterface, pcustominfo) }
}
#[inline]
pub unsafe fn MprAdminInterfaceSetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceSetInfo(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    unsafe { MprAdminInterfaceSetInfo(hmprserver, hinterface, dwlevel, lpbbuffer) }
}
#[inline]
pub unsafe fn MprAdminInterfaceTransportAdd(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwtransportid: u32, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceTransportAdd(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwtransportid : u32, pinterfaceinfo : *const u8, dwinterfaceinfosize : u32) -> u32);
    unsafe { MprAdminInterfaceTransportAdd(hmprserver, hinterface, dwtransportid, pinterfaceinfo, dwinterfaceinfosize) }
}
#[inline]
pub unsafe fn MprAdminInterfaceTransportGetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwtransportid: u32, ppinterfaceinfo: *mut *mut u8, lpdwinterfaceinfosize: Option<*mut u32>) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceTransportGetInfo(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwtransportid : u32, ppinterfaceinfo : *mut *mut u8, lpdwinterfaceinfosize : *mut u32) -> u32);
    unsafe { MprAdminInterfaceTransportGetInfo(hmprserver, hinterface, dwtransportid, ppinterfaceinfo as _, lpdwinterfaceinfosize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MprAdminInterfaceTransportRemove(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwtransportid: u32) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceTransportRemove(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwtransportid : u32) -> u32);
    unsafe { MprAdminInterfaceTransportRemove(hmprserver, hinterface, dwtransportid) }
}
#[inline]
pub unsafe fn MprAdminInterfaceTransportSetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwtransportid: u32, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceTransportSetInfo(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwtransportid : u32, pinterfaceinfo : *const u8, dwinterfaceinfosize : u32) -> u32);
    unsafe { MprAdminInterfaceTransportSetInfo(hmprserver, hinterface, dwtransportid, pinterfaceinfo, dwinterfaceinfosize) }
}
#[inline]
pub unsafe fn MprAdminInterfaceUpdatePhonebookInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceUpdatePhonebookInfo(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprAdminInterfaceUpdatePhonebookInfo(hmprserver, hinterface) }
}
#[inline]
pub unsafe fn MprAdminInterfaceUpdateRoutes(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwprotocolid: u32, hevent: super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminInterfaceUpdateRoutes(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwprotocolid : u32, hevent : super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprAdminInterfaceUpdateRoutes(hmprserver, hinterface, dwprotocolid, hevent) }
}
#[inline]
pub unsafe fn MprAdminIsDomainRasServer<P0, P1>(pszdomain: P0, pszmachine: P1, pbisrasserver: *mut windows_core::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprAdminIsDomainRasServer(pszdomain : windows_core::PCWSTR, pszmachine : windows_core::PCWSTR, pbisrasserver : *mut windows_core::BOOL) -> u32);
    unsafe { MprAdminIsDomainRasServer(pszdomain.param().abi(), pszmachine.param().abi(), pbisrasserver as _) }
}
#[inline]
pub unsafe fn MprAdminIsServiceInitialized<P0>(lpwsservername: P0, fisserviceinitialized: *const windows_core::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprAdminIsServiceInitialized(lpwsservername : windows_core::PCWSTR, fisserviceinitialized : *const windows_core::BOOL) -> u32);
    unsafe { MprAdminIsServiceInitialized(lpwsservername.param().abi(), fisserviceinitialized) }
}
#[inline]
pub unsafe fn MprAdminIsServiceRunning<P0>(lpwsservername: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprAdminIsServiceRunning(lpwsservername : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { MprAdminIsServiceRunning(lpwsservername.param().abi()) }
}
#[inline]
pub unsafe fn MprAdminMIBBufferFree(pbuffer: *const core::ffi::c_void) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminMIBBufferFree(pbuffer : *const core::ffi::c_void) -> u32);
    unsafe { MprAdminMIBBufferFree(pbuffer) }
}
#[inline]
pub unsafe fn MprAdminMIBEntryCreate(hmibserver: isize, dwpid: u32, dwroutingpid: u32, lpentry: *const core::ffi::c_void, dwentrysize: u32) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminMIBEntryCreate(hmibserver : isize, dwpid : u32, dwroutingpid : u32, lpentry : *const core::ffi::c_void, dwentrysize : u32) -> u32);
    unsafe { MprAdminMIBEntryCreate(hmibserver, dwpid, dwroutingpid, lpentry, dwentrysize) }
}
#[inline]
pub unsafe fn MprAdminMIBEntryDelete(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpentry: *const core::ffi::c_void, dwentrysize: u32) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminMIBEntryDelete(hmibserver : isize, dwprotocolid : u32, dwroutingpid : u32, lpentry : *const core::ffi::c_void, dwentrysize : u32) -> u32);
    unsafe { MprAdminMIBEntryDelete(hmibserver, dwprotocolid, dwroutingpid, lpentry, dwentrysize) }
}
#[inline]
pub unsafe fn MprAdminMIBEntryGet(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut core::ffi::c_void, lpoutentrysize: *mut u32) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminMIBEntryGet(hmibserver : isize, dwprotocolid : u32, dwroutingpid : u32, lpinentry : *const core::ffi::c_void, dwinentrysize : u32, lplpoutentry : *mut *mut core::ffi::c_void, lpoutentrysize : *mut u32) -> u32);
    unsafe { MprAdminMIBEntryGet(hmibserver, dwprotocolid, dwroutingpid, lpinentry, dwinentrysize, lplpoutentry as _, lpoutentrysize as _) }
}
#[inline]
pub unsafe fn MprAdminMIBEntryGetFirst(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut core::ffi::c_void, lpoutentrysize: *mut u32) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminMIBEntryGetFirst(hmibserver : isize, dwprotocolid : u32, dwroutingpid : u32, lpinentry : *const core::ffi::c_void, dwinentrysize : u32, lplpoutentry : *mut *mut core::ffi::c_void, lpoutentrysize : *mut u32) -> u32);
    unsafe { MprAdminMIBEntryGetFirst(hmibserver, dwprotocolid, dwroutingpid, lpinentry, dwinentrysize, lplpoutentry as _, lpoutentrysize as _) }
}
#[inline]
pub unsafe fn MprAdminMIBEntryGetNext(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut core::ffi::c_void, lpoutentrysize: *mut u32) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminMIBEntryGetNext(hmibserver : isize, dwprotocolid : u32, dwroutingpid : u32, lpinentry : *const core::ffi::c_void, dwinentrysize : u32, lplpoutentry : *mut *mut core::ffi::c_void, lpoutentrysize : *mut u32) -> u32);
    unsafe { MprAdminMIBEntryGetNext(hmibserver, dwprotocolid, dwroutingpid, lpinentry, dwinentrysize, lplpoutentry as _, lpoutentrysize as _) }
}
#[inline]
pub unsafe fn MprAdminMIBEntrySet(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpentry: *const core::ffi::c_void, dwentrysize: u32) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminMIBEntrySet(hmibserver : isize, dwprotocolid : u32, dwroutingpid : u32, lpentry : *const core::ffi::c_void, dwentrysize : u32) -> u32);
    unsafe { MprAdminMIBEntrySet(hmibserver, dwprotocolid, dwroutingpid, lpentry, dwentrysize) }
}
#[inline]
pub unsafe fn MprAdminMIBServerConnect<P0>(lpwsservername: P0, phmibserver: *mut isize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprAdminMIBServerConnect(lpwsservername : windows_core::PCWSTR, phmibserver : *mut isize) -> u32);
    unsafe { MprAdminMIBServerConnect(lpwsservername.param().abi(), phmibserver as _) }
}
#[inline]
pub unsafe fn MprAdminMIBServerDisconnect(hmibserver: isize) {
    windows_link::link!("mprapi.dll" "system" fn MprAdminMIBServerDisconnect(hmibserver : isize));
    unsafe { MprAdminMIBServerDisconnect(hmibserver) }
}
#[inline]
pub unsafe fn MprAdminPortClearStats(hrasserver: isize, hport: super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminPortClearStats(hrasserver : isize, hport : super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprAdminPortClearStats(hrasserver, hport) }
}
#[inline]
pub unsafe fn MprAdminPortDisconnect(hrasserver: isize, hport: super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminPortDisconnect(hrasserver : isize, hport : super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprAdminPortDisconnect(hrasserver, hport) }
}
#[inline]
pub unsafe fn MprAdminPortEnum(hrasserver: isize, dwlevel: u32, hrasconnection: super::super::Foundation::HANDLE, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*const u32>) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminPortEnum(hrasserver : isize, dwlevel : u32, hrasconnection : super::super::Foundation:: HANDLE, lplpbbuffer : *mut *mut u8, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *const u32) -> u32);
    unsafe { MprAdminPortEnum(hrasserver, dwlevel, hrasconnection, lplpbbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MprAdminPortGetInfo(hrasserver: isize, dwlevel: u32, hport: super::super::Foundation::HANDLE, lplpbbuffer: *mut *mut u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminPortGetInfo(hrasserver : isize, dwlevel : u32, hport : super::super::Foundation:: HANDLE, lplpbbuffer : *mut *mut u8) -> u32);
    unsafe { MprAdminPortGetInfo(hrasserver, dwlevel, hport, lplpbbuffer as _) }
}
#[inline]
pub unsafe fn MprAdminPortReset(hrasserver: isize, hport: super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminPortReset(hrasserver : isize, hport : super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprAdminPortReset(hrasserver, hport) }
}
#[inline]
pub unsafe fn MprAdminRegisterConnectionNotification(hmprserver: isize, heventnotification: super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminRegisterConnectionNotification(hmprserver : isize, heventnotification : super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprAdminRegisterConnectionNotification(hmprserver, heventnotification) }
}
#[inline]
pub unsafe fn MprAdminSendUserMessage<P2>(hmprserver: isize, hconnection: super::super::Foundation::HANDLE, lpwszmessage: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprAdminSendUserMessage(hmprserver : isize, hconnection : super::super::Foundation:: HANDLE, lpwszmessage : windows_core::PCWSTR) -> u32);
    unsafe { MprAdminSendUserMessage(hmprserver, hconnection, lpwszmessage.param().abi()) }
}
#[inline]
pub unsafe fn MprAdminServerConnect<P0>(lpwsservername: P0, phmprserver: *mut isize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprAdminServerConnect(lpwsservername : windows_core::PCWSTR, phmprserver : *mut isize) -> u32);
    unsafe { MprAdminServerConnect(lpwsservername.param().abi(), phmprserver as _) }
}
#[inline]
pub unsafe fn MprAdminServerDisconnect(hmprserver: isize) {
    windows_link::link!("mprapi.dll" "system" fn MprAdminServerDisconnect(hmprserver : isize));
    unsafe { MprAdminServerDisconnect(hmprserver) }
}
#[inline]
pub unsafe fn MprAdminServerGetCredentials(hmprserver: isize, dwlevel: u32, lplpbbuffer: *const *const u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminServerGetCredentials(hmprserver : isize, dwlevel : u32, lplpbbuffer : *const *const u8) -> u32);
    unsafe { MprAdminServerGetCredentials(hmprserver, dwlevel, lplpbbuffer) }
}
#[inline]
pub unsafe fn MprAdminServerGetInfo(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminServerGetInfo(hmprserver : isize, dwlevel : u32, lplpbbuffer : *mut *mut u8) -> u32);
    unsafe { MprAdminServerGetInfo(hmprserver, dwlevel, lplpbbuffer as _) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn MprAdminServerGetInfoEx(hmprserver: isize, pserverinfo: *mut MPR_SERVER_EX1) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminServerGetInfoEx(hmprserver : isize, pserverinfo : *mut MPR_SERVER_EX1) -> u32);
    unsafe { MprAdminServerGetInfoEx(hmprserver, pserverinfo as _) }
}
#[inline]
pub unsafe fn MprAdminServerSetCredentials(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminServerSetCredentials(hmprserver : isize, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    unsafe { MprAdminServerSetCredentials(hmprserver, dwlevel, lpbbuffer) }
}
#[inline]
pub unsafe fn MprAdminServerSetInfo(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminServerSetInfo(hmprserver : isize, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    unsafe { MprAdminServerSetInfo(hmprserver, dwlevel, lpbbuffer) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn MprAdminServerSetInfoEx(hmprserver: isize, pserverinfo: *const MPR_SERVER_SET_CONFIG_EX1) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminServerSetInfoEx(hmprserver : isize, pserverinfo : *const MPR_SERVER_SET_CONFIG_EX1) -> u32);
    unsafe { MprAdminServerSetInfoEx(hmprserver, pserverinfo) }
}
#[inline]
pub unsafe fn MprAdminTransportCreate<P2, P7>(hmprserver: isize, dwtransportid: u32, lpwstransportname: P2, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: Option<*const u8>, dwclientinterfaceinfosize: Option<u32>, lpwsdllpath: P7) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprAdminTransportCreate(hmprserver : isize, dwtransportid : u32, lpwstransportname : windows_core::PCWSTR, pglobalinfo : *const u8, dwglobalinfosize : u32, pclientinterfaceinfo : *const u8, dwclientinterfaceinfosize : u32, lpwsdllpath : windows_core::PCWSTR) -> u32);
    unsafe { MprAdminTransportCreate(hmprserver, dwtransportid, lpwstransportname.param().abi(), pglobalinfo, dwglobalinfosize, pclientinterfaceinfo.unwrap_or(core::mem::zeroed()) as _, dwclientinterfaceinfosize.unwrap_or(core::mem::zeroed()) as _, lpwsdllpath.param().abi()) }
}
#[inline]
pub unsafe fn MprAdminTransportGetInfo(hmprserver: isize, dwtransportid: u32, ppglobalinfo: Option<*mut *mut u8>, lpdwglobalinfosize: Option<*mut u32>, ppclientinterfaceinfo: Option<*mut *mut u8>, lpdwclientinterfaceinfosize: Option<*mut u32>) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminTransportGetInfo(hmprserver : isize, dwtransportid : u32, ppglobalinfo : *mut *mut u8, lpdwglobalinfosize : *mut u32, ppclientinterfaceinfo : *mut *mut u8, lpdwclientinterfaceinfosize : *mut u32) -> u32);
    unsafe { MprAdminTransportGetInfo(hmprserver, dwtransportid, ppglobalinfo.unwrap_or(core::mem::zeroed()) as _, lpdwglobalinfosize.unwrap_or(core::mem::zeroed()) as _, ppclientinterfaceinfo.unwrap_or(core::mem::zeroed()) as _, lpdwclientinterfaceinfosize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MprAdminTransportSetInfo(hmprserver: isize, dwtransportid: u32, pglobalinfo: Option<*const u8>, dwglobalinfosize: u32, pclientinterfaceinfo: Option<*const u8>, dwclientinterfaceinfosize: u32) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminTransportSetInfo(hmprserver : isize, dwtransportid : u32, pglobalinfo : *const u8, dwglobalinfosize : u32, pclientinterfaceinfo : *const u8, dwclientinterfaceinfosize : u32) -> u32);
    unsafe { MprAdminTransportSetInfo(hmprserver, dwtransportid, pglobalinfo.unwrap_or(core::mem::zeroed()) as _, dwglobalinfosize, pclientinterfaceinfo.unwrap_or(core::mem::zeroed()) as _, dwclientinterfaceinfosize) }
}
#[inline]
pub unsafe fn MprAdminUpdateConnection(hrasserver: isize, hrasconnection: super::super::Foundation::HANDLE, prasupdateconnection: *const RAS_UPDATE_CONNECTION) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprAdminUpdateConnection(hrasserver : isize, hrasconnection : super::super::Foundation:: HANDLE, prasupdateconnection : *const RAS_UPDATE_CONNECTION) -> u32);
    unsafe { MprAdminUpdateConnection(hrasserver, hrasconnection, prasupdateconnection) }
}
#[inline]
pub unsafe fn MprAdminUserGetInfo<P0, P1>(lpszserver: P0, lpszuser: P1, dwlevel: u32, lpbbuffer: *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprAdminUserGetInfo(lpszserver : windows_core::PCWSTR, lpszuser : windows_core::PCWSTR, dwlevel : u32, lpbbuffer : *mut u8) -> u32);
    unsafe { MprAdminUserGetInfo(lpszserver.param().abi(), lpszuser.param().abi(), dwlevel, lpbbuffer as _) }
}
#[inline]
pub unsafe fn MprAdminUserSetInfo<P0, P1>(lpszserver: P0, lpszuser: P1, dwlevel: u32, lpbbuffer: *const u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprAdminUserSetInfo(lpszserver : windows_core::PCWSTR, lpszuser : windows_core::PCWSTR, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    unsafe { MprAdminUserSetInfo(lpszserver.param().abi(), lpszuser.param().abi(), dwlevel, lpbbuffer) }
}
#[inline]
pub unsafe fn MprConfigBufferFree(pbuffer: *const core::ffi::c_void) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigBufferFree(pbuffer : *const core::ffi::c_void) -> u32);
    unsafe { MprConfigBufferFree(pbuffer) }
}
#[inline]
pub unsafe fn MprConfigFilterGetInfo(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, dwtransportid: u32, lpbuffer: *mut u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigFilterGetInfo(hmprconfig : super::super::Foundation:: HANDLE, dwlevel : u32, dwtransportid : u32, lpbuffer : *mut u8) -> u32);
    unsafe { MprConfigFilterGetInfo(hmprconfig, dwlevel, dwtransportid, lpbuffer as _) }
}
#[inline]
pub unsafe fn MprConfigFilterSetInfo(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, dwtransportid: u32, lpbuffer: *const u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigFilterSetInfo(hmprconfig : super::super::Foundation:: HANDLE, dwlevel : u32, dwtransportid : u32, lpbuffer : *const u8) -> u32);
    unsafe { MprConfigFilterSetInfo(hmprconfig, dwlevel, dwtransportid, lpbuffer) }
}
#[inline]
pub unsafe fn MprConfigGetFriendlyName<P1>(hmprconfig: super::super::Foundation::HANDLE, pszguidname: P1, pszbuffer: windows_core::PWSTR, dwbuffersize: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprConfigGetFriendlyName(hmprconfig : super::super::Foundation:: HANDLE, pszguidname : windows_core::PCWSTR, pszbuffer : windows_core::PWSTR, dwbuffersize : u32) -> u32);
    unsafe { MprConfigGetFriendlyName(hmprconfig, pszguidname.param().abi(), core::mem::transmute(pszbuffer), dwbuffersize) }
}
#[inline]
pub unsafe fn MprConfigGetGuidName<P1>(hmprconfig: super::super::Foundation::HANDLE, pszfriendlyname: P1, pszbuffer: windows_core::PWSTR, dwbuffersize: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprConfigGetGuidName(hmprconfig : super::super::Foundation:: HANDLE, pszfriendlyname : windows_core::PCWSTR, pszbuffer : windows_core::PWSTR, dwbuffersize : u32) -> u32);
    unsafe { MprConfigGetGuidName(hmprconfig, pszfriendlyname.param().abi(), core::mem::transmute(pszbuffer), dwbuffersize) }
}
#[inline]
pub unsafe fn MprConfigInterfaceCreate(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, lpbbuffer: *const u8, phrouterinterface: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigInterfaceCreate(hmprconfig : super::super::Foundation:: HANDLE, dwlevel : u32, lpbbuffer : *const u8, phrouterinterface : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprConfigInterfaceCreate(hmprconfig, dwlevel, lpbbuffer, phrouterinterface as _) }
}
#[inline]
pub unsafe fn MprConfigInterfaceDelete(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigInterfaceDelete(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprConfigInterfaceDelete(hmprconfig, hrouterinterface) }
}
#[inline]
pub unsafe fn MprConfigInterfaceEnum(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*mut u32>) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigInterfaceEnum(hmprconfig : super::super::Foundation:: HANDLE, dwlevel : u32, lplpbuffer : *mut *mut u8, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *mut u32) -> u32);
    unsafe { MprConfigInterfaceEnum(hmprconfig, dwlevel, lplpbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprConfigInterfaceGetCustomInfoEx(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, pcustominfo: *mut MPR_IF_CUSTOMINFOEX2) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigInterfaceGetCustomInfoEx(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, pcustominfo : *mut MPR_IF_CUSTOMINFOEX2) -> u32);
    unsafe { MprConfigInterfaceGetCustomInfoEx(hmprconfig, hrouterinterface, pcustominfo as _) }
}
#[inline]
pub unsafe fn MprConfigInterfaceGetHandle<P1>(hmprconfig: super::super::Foundation::HANDLE, lpwsinterfacename: P1, phrouterinterface: *mut super::super::Foundation::HANDLE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprConfigInterfaceGetHandle(hmprconfig : super::super::Foundation:: HANDLE, lpwsinterfacename : windows_core::PCWSTR, phrouterinterface : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprConfigInterfaceGetHandle(hmprconfig, lpwsinterfacename.param().abi(), phrouterinterface as _) }
}
#[inline]
pub unsafe fn MprConfigInterfaceGetInfo(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwlevel: u32, lplpbuffer: *mut *mut u8, lpdwbuffersize: *mut u32) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigInterfaceGetInfo(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, dwlevel : u32, lplpbuffer : *mut *mut u8, lpdwbuffersize : *mut u32) -> u32);
    unsafe { MprConfigInterfaceGetInfo(hmprconfig, hrouterinterface, dwlevel, lplpbuffer as _, lpdwbuffersize as _) }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprConfigInterfaceSetCustomInfoEx(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, pcustominfo: *const MPR_IF_CUSTOMINFOEX2) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigInterfaceSetCustomInfoEx(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, pcustominfo : *const MPR_IF_CUSTOMINFOEX2) -> u32);
    unsafe { MprConfigInterfaceSetCustomInfoEx(hmprconfig, hrouterinterface, pcustominfo) }
}
#[inline]
pub unsafe fn MprConfigInterfaceSetInfo(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigInterfaceSetInfo(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    unsafe { MprConfigInterfaceSetInfo(hmprconfig, hrouterinterface, dwlevel, lpbbuffer) }
}
#[inline]
pub unsafe fn MprConfigInterfaceTransportAdd<P3>(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwtransportid: u32, lpwstransportname: P3, pinterfaceinfo: &[u8], phrouteriftransport: *mut super::super::Foundation::HANDLE) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportAdd(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, dwtransportid : u32, lpwstransportname : windows_core::PCWSTR, pinterfaceinfo : *const u8, dwinterfaceinfosize : u32, phrouteriftransport : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprConfigInterfaceTransportAdd(hmprconfig, hrouterinterface, dwtransportid, lpwstransportname.param().abi(), core::mem::transmute(pinterfaceinfo.as_ptr()), pinterfaceinfo.len().try_into().unwrap(), phrouteriftransport as _) }
}
#[inline]
pub unsafe fn MprConfigInterfaceTransportEnum(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*mut u32>) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportEnum(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, dwlevel : u32, lplpbuffer : *mut *mut u8, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *mut u32) -> u32);
    unsafe { MprConfigInterfaceTransportEnum(hmprconfig, hrouterinterface, dwlevel, lplpbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MprConfigInterfaceTransportGetHandle(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwtransportid: u32, phrouteriftransport: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportGetHandle(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, dwtransportid : u32, phrouteriftransport : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprConfigInterfaceTransportGetHandle(hmprconfig, hrouterinterface, dwtransportid, phrouteriftransport as _) }
}
#[inline]
pub unsafe fn MprConfigInterfaceTransportGetInfo(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, hrouteriftransport: super::super::Foundation::HANDLE, ppinterfaceinfo: Option<*mut *mut u8>, lpdwinterfaceinfosize: Option<*mut u32>) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportGetInfo(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, hrouteriftransport : super::super::Foundation:: HANDLE, ppinterfaceinfo : *mut *mut u8, lpdwinterfaceinfosize : *mut u32) -> u32);
    unsafe { MprConfigInterfaceTransportGetInfo(hmprconfig, hrouterinterface, hrouteriftransport, ppinterfaceinfo.unwrap_or(core::mem::zeroed()) as _, lpdwinterfaceinfosize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MprConfigInterfaceTransportRemove(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, hrouteriftransport: super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportRemove(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, hrouteriftransport : super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprConfigInterfaceTransportRemove(hmprconfig, hrouterinterface, hrouteriftransport) }
}
#[inline]
pub unsafe fn MprConfigInterfaceTransportSetInfo(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, hrouteriftransport: super::super::Foundation::HANDLE, pinterfaceinfo: Option<&[u8]>) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportSetInfo(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, hrouteriftransport : super::super::Foundation:: HANDLE, pinterfaceinfo : *const u8, dwinterfaceinfosize : u32) -> u32);
    unsafe { MprConfigInterfaceTransportSetInfo(hmprconfig, hrouterinterface, hrouteriftransport, core::mem::transmute(pinterfaceinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pinterfaceinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn MprConfigServerBackup<P1>(hmprconfig: super::super::Foundation::HANDLE, lpwspath: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprConfigServerBackup(hmprconfig : super::super::Foundation:: HANDLE, lpwspath : windows_core::PCWSTR) -> u32);
    unsafe { MprConfigServerBackup(hmprconfig, lpwspath.param().abi()) }
}
#[inline]
pub unsafe fn MprConfigServerConnect<P0>(lpwsservername: P0, phmprconfig: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprConfigServerConnect(lpwsservername : windows_core::PCWSTR, phmprconfig : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprConfigServerConnect(lpwsservername.param().abi(), phmprconfig as _) }
}
#[inline]
pub unsafe fn MprConfigServerDisconnect(hmprconfig: super::super::Foundation::HANDLE) {
    windows_link::link!("mprapi.dll" "system" fn MprConfigServerDisconnect(hmprconfig : super::super::Foundation:: HANDLE));
    unsafe { MprConfigServerDisconnect(hmprconfig) }
}
#[inline]
pub unsafe fn MprConfigServerGetInfo(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigServerGetInfo(hmprconfig : super::super::Foundation:: HANDLE, dwlevel : u32, lplpbbuffer : *mut *mut u8) -> u32);
    unsafe { MprConfigServerGetInfo(hmprconfig, dwlevel, lplpbbuffer as _) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn MprConfigServerGetInfoEx(hmprconfig: super::super::Foundation::HANDLE, pserverinfo: *mut MPR_SERVER_EX1) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigServerGetInfoEx(hmprconfig : super::super::Foundation:: HANDLE, pserverinfo : *mut MPR_SERVER_EX1) -> u32);
    unsafe { MprConfigServerGetInfoEx(hmprconfig, pserverinfo as _) }
}
#[inline]
pub unsafe fn MprConfigServerInstall(dwlevel: u32, pbuffer: *const core::ffi::c_void) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigServerInstall(dwlevel : u32, pbuffer : *const core::ffi::c_void) -> u32);
    unsafe { MprConfigServerInstall(dwlevel, pbuffer) }
}
#[inline]
pub unsafe fn MprConfigServerRefresh(hmprconfig: super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigServerRefresh(hmprconfig : super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprConfigServerRefresh(hmprconfig) }
}
#[inline]
pub unsafe fn MprConfigServerRestore<P1>(hmprconfig: super::super::Foundation::HANDLE, lpwspath: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprConfigServerRestore(hmprconfig : super::super::Foundation:: HANDLE, lpwspath : windows_core::PCWSTR) -> u32);
    unsafe { MprConfigServerRestore(hmprconfig, lpwspath.param().abi()) }
}
#[inline]
pub unsafe fn MprConfigServerSetInfo(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigServerSetInfo(hmprserver : isize, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    unsafe { MprConfigServerSetInfo(hmprserver, dwlevel, lpbbuffer) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn MprConfigServerSetInfoEx(hmprconfig: super::super::Foundation::HANDLE, psetserverconfig: *const MPR_SERVER_SET_CONFIG_EX1) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigServerSetInfoEx(hmprconfig : super::super::Foundation:: HANDLE, psetserverconfig : *const MPR_SERVER_SET_CONFIG_EX1) -> u32);
    unsafe { MprConfigServerSetInfoEx(hmprconfig, psetserverconfig) }
}
#[inline]
pub unsafe fn MprConfigTransportCreate<P2, P7>(hmprconfig: super::super::Foundation::HANDLE, dwtransportid: u32, lpwstransportname: P2, pglobalinfo: &[u8], pclientinterfaceinfo: Option<&[u8]>, lpwsdllpath: P7, phroutertransport: *mut super::super::Foundation::HANDLE) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprConfigTransportCreate(hmprconfig : super::super::Foundation:: HANDLE, dwtransportid : u32, lpwstransportname : windows_core::PCWSTR, pglobalinfo : *const u8, dwglobalinfosize : u32, pclientinterfaceinfo : *const u8, dwclientinterfaceinfosize : u32, lpwsdllpath : windows_core::PCWSTR, phroutertransport : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprConfigTransportCreate(hmprconfig, dwtransportid, lpwstransportname.param().abi(), core::mem::transmute(pglobalinfo.as_ptr()), pglobalinfo.len().try_into().unwrap(), core::mem::transmute(pclientinterfaceinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pclientinterfaceinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpwsdllpath.param().abi(), phroutertransport as _) }
}
#[inline]
pub unsafe fn MprConfigTransportDelete(hmprconfig: super::super::Foundation::HANDLE, hroutertransport: super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigTransportDelete(hmprconfig : super::super::Foundation:: HANDLE, hroutertransport : super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprConfigTransportDelete(hmprconfig, hroutertransport) }
}
#[inline]
pub unsafe fn MprConfigTransportEnum(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*mut u32>) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigTransportEnum(hmprconfig : super::super::Foundation:: HANDLE, dwlevel : u32, lplpbuffer : *mut *mut u8, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *mut u32) -> u32);
    unsafe { MprConfigTransportEnum(hmprconfig, dwlevel, lplpbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MprConfigTransportGetHandle(hmprconfig: super::super::Foundation::HANDLE, dwtransportid: u32, phroutertransport: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigTransportGetHandle(hmprconfig : super::super::Foundation:: HANDLE, dwtransportid : u32, phroutertransport : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { MprConfigTransportGetHandle(hmprconfig, dwtransportid, phroutertransport as _) }
}
#[inline]
pub unsafe fn MprConfigTransportGetInfo(hmprconfig: super::super::Foundation::HANDLE, hroutertransport: super::super::Foundation::HANDLE, ppglobalinfo: Option<*mut *mut u8>, lpdwglobalinfosize: Option<*mut u32>, ppclientinterfaceinfo: Option<*mut *mut u8>, lpdwclientinterfaceinfosize: Option<*mut u32>, lplpwsdllpath: Option<*mut windows_core::PWSTR>) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprConfigTransportGetInfo(hmprconfig : super::super::Foundation:: HANDLE, hroutertransport : super::super::Foundation:: HANDLE, ppglobalinfo : *mut *mut u8, lpdwglobalinfosize : *mut u32, ppclientinterfaceinfo : *mut *mut u8, lpdwclientinterfaceinfosize : *mut u32, lplpwsdllpath : *mut windows_core::PWSTR) -> u32);
    unsafe { MprConfigTransportGetInfo(hmprconfig, hroutertransport, ppglobalinfo.unwrap_or(core::mem::zeroed()) as _, lpdwglobalinfosize.unwrap_or(core::mem::zeroed()) as _, ppclientinterfaceinfo.unwrap_or(core::mem::zeroed()) as _, lpdwclientinterfaceinfosize.unwrap_or(core::mem::zeroed()) as _, lplpwsdllpath.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MprConfigTransportSetInfo<P6>(hmprconfig: super::super::Foundation::HANDLE, hroutertransport: super::super::Foundation::HANDLE, pglobalinfo: Option<&[u8]>, pclientinterfaceinfo: Option<&[u8]>, lpwsdllpath: P6) -> u32
where
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("mprapi.dll" "system" fn MprConfigTransportSetInfo(hmprconfig : super::super::Foundation:: HANDLE, hroutertransport : super::super::Foundation:: HANDLE, pglobalinfo : *const u8, dwglobalinfosize : u32, pclientinterfaceinfo : *const u8, dwclientinterfaceinfosize : u32, lpwsdllpath : windows_core::PCWSTR) -> u32);
    unsafe { MprConfigTransportSetInfo(hmprconfig, hroutertransport, core::mem::transmute(pglobalinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pglobalinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pclientinterfaceinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pclientinterfaceinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpwsdllpath.param().abi()) }
}
#[inline]
pub unsafe fn MprInfoBlockAdd(lpheader: *const core::ffi::c_void, dwinfotype: u32, dwitemsize: u32, dwitemcount: u32, lpitemdata: *const u8, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprInfoBlockAdd(lpheader : *const core::ffi::c_void, dwinfotype : u32, dwitemsize : u32, dwitemcount : u32, lpitemdata : *const u8, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    unsafe { MprInfoBlockAdd(lpheader, dwinfotype, dwitemsize, dwitemcount, lpitemdata, lplpnewheader as _) }
}
#[inline]
pub unsafe fn MprInfoBlockFind(lpheader: *const core::ffi::c_void, dwinfotype: u32, lpdwitemsize: *mut u32, lpdwitemcount: *mut u32, lplpitemdata: *mut *mut u8) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprInfoBlockFind(lpheader : *const core::ffi::c_void, dwinfotype : u32, lpdwitemsize : *mut u32, lpdwitemcount : *mut u32, lplpitemdata : *mut *mut u8) -> u32);
    unsafe { MprInfoBlockFind(lpheader, dwinfotype, lpdwitemsize as _, lpdwitemcount as _, lplpitemdata as _) }
}
#[inline]
pub unsafe fn MprInfoBlockQuerySize(lpheader: *const core::ffi::c_void) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprInfoBlockQuerySize(lpheader : *const core::ffi::c_void) -> u32);
    unsafe { MprInfoBlockQuerySize(lpheader) }
}
#[inline]
pub unsafe fn MprInfoBlockRemove(lpheader: *const core::ffi::c_void, dwinfotype: u32, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprInfoBlockRemove(lpheader : *const core::ffi::c_void, dwinfotype : u32, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    unsafe { MprInfoBlockRemove(lpheader, dwinfotype, lplpnewheader as _) }
}
#[inline]
pub unsafe fn MprInfoBlockSet(lpheader: *const core::ffi::c_void, dwinfotype: u32, dwitemsize: u32, dwitemcount: u32, lpitemdata: *const u8, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprInfoBlockSet(lpheader : *const core::ffi::c_void, dwinfotype : u32, dwitemsize : u32, dwitemcount : u32, lpitemdata : *const u8, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    unsafe { MprInfoBlockSet(lpheader, dwinfotype, dwitemsize, dwitemcount, lpitemdata, lplpnewheader as _) }
}
#[inline]
pub unsafe fn MprInfoCreate(dwversion: u32, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprInfoCreate(dwversion : u32, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    unsafe { MprInfoCreate(dwversion, lplpnewheader as _) }
}
#[inline]
pub unsafe fn MprInfoDelete(lpheader: *const core::ffi::c_void) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprInfoDelete(lpheader : *const core::ffi::c_void) -> u32);
    unsafe { MprInfoDelete(lpheader) }
}
#[inline]
pub unsafe fn MprInfoDuplicate(lpheader: *const core::ffi::c_void, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprInfoDuplicate(lpheader : *const core::ffi::c_void, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    unsafe { MprInfoDuplicate(lpheader, lplpnewheader as _) }
}
#[inline]
pub unsafe fn MprInfoRemoveAll(lpheader: *const core::ffi::c_void, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_link::link!("mprapi.dll" "system" fn MprInfoRemoveAll(lpheader : *const core::ffi::c_void, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    unsafe { MprInfoRemoveAll(lpheader, lplpnewheader as _) }
}
#[inline]
pub unsafe fn RasClearConnectionStatistics(hrasconn: HRASCONN) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasClearConnectionStatistics(hrasconn : HRASCONN) -> u32);
    unsafe { RasClearConnectionStatistics(hrasconn) }
}
#[inline]
pub unsafe fn RasClearLinkStatistics(hrasconn: HRASCONN, dwsubentry: u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasClearLinkStatistics(hrasconn : HRASCONN, dwsubentry : u32) -> u32);
    unsafe { RasClearLinkStatistics(hrasconn, dwsubentry) }
}
#[inline]
pub unsafe fn RasConnectionNotificationA(param0: HRASCONN, param1: super::super::Foundation::HANDLE, param2: u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasConnectionNotificationA(param0 : HRASCONN, param1 : super::super::Foundation:: HANDLE, param2 : u32) -> u32);
    unsafe { RasConnectionNotificationA(param0, param1, param2) }
}
#[inline]
pub unsafe fn RasConnectionNotificationW(param0: HRASCONN, param1: super::super::Foundation::HANDLE, param2: u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasConnectionNotificationW(param0 : HRASCONN, param1 : super::super::Foundation:: HANDLE, param2 : u32) -> u32);
    unsafe { RasConnectionNotificationW(param0, param1, param2) }
}
#[inline]
pub unsafe fn RasCreatePhonebookEntryA<P1>(param0: super::super::Foundation::HWND, param1: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasCreatePhonebookEntryA(param0 : super::super::Foundation:: HWND, param1 : windows_core::PCSTR) -> u32);
    unsafe { RasCreatePhonebookEntryA(param0, param1.param().abi()) }
}
#[inline]
pub unsafe fn RasCreatePhonebookEntryW<P1>(param0: super::super::Foundation::HWND, param1: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasCreatePhonebookEntryW(param0 : super::super::Foundation:: HWND, param1 : windows_core::PCWSTR) -> u32);
    unsafe { RasCreatePhonebookEntryW(param0, param1.param().abi()) }
}
#[inline]
pub unsafe fn RasDeleteEntryA<P0, P1>(param0: P0, param1: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasDeleteEntryA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR) -> u32);
    unsafe { RasDeleteEntryA(param0.param().abi(), param1.param().abi()) }
}
#[inline]
pub unsafe fn RasDeleteEntryW<P0, P1>(param0: P0, param1: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasDeleteEntryW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR) -> u32);
    unsafe { RasDeleteEntryW(param0.param().abi(), param1.param().abi()) }
}
#[inline]
pub unsafe fn RasDeleteSubEntryA<P0, P1>(pszphonebook: P0, pszentry: P1, dwsubentryid: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasDeleteSubEntryA(pszphonebook : windows_core::PCSTR, pszentry : windows_core::PCSTR, dwsubentryid : u32) -> u32);
    unsafe { RasDeleteSubEntryA(pszphonebook.param().abi(), pszentry.param().abi(), dwsubentryid) }
}
#[inline]
pub unsafe fn RasDeleteSubEntryW<P0, P1>(pszphonebook: P0, pszentry: P1, dwsubentryid: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasDeleteSubEntryW(pszphonebook : windows_core::PCWSTR, pszentry : windows_core::PCWSTR, dwsubentryid : u32) -> u32);
    unsafe { RasDeleteSubEntryW(pszphonebook.param().abi(), pszentry.param().abi(), dwsubentryid) }
}
#[inline]
pub unsafe fn RasDialA<P1>(param0: Option<*const RASDIALEXTENSIONS>, param1: P1, param2: *const RASDIALPARAMSA, param3: u32, param4: Option<*const core::ffi::c_void>, param5: *mut HRASCONN) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasDialA(param0 : *const RASDIALEXTENSIONS, param1 : windows_core::PCSTR, param2 : *const RASDIALPARAMSA, param3 : u32, param4 : *const core::ffi::c_void, param5 : *mut HRASCONN) -> u32);
    unsafe { RasDialA(param0.unwrap_or(core::mem::zeroed()) as _, param1.param().abi(), param2, param3, param4.unwrap_or(core::mem::zeroed()) as _, param5 as _) }
}
#[inline]
pub unsafe fn RasDialDlgA<P0, P1, P2>(lpszphonebook: P0, lpszentry: P1, lpszphonenumber: P2, lpinfo: *mut RASDIALDLG) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasdlg.dll" "system" fn RasDialDlgA(lpszphonebook : windows_core::PCSTR, lpszentry : windows_core::PCSTR, lpszphonenumber : windows_core::PCSTR, lpinfo : *mut RASDIALDLG) -> windows_core::BOOL);
    unsafe { RasDialDlgA(lpszphonebook.param().abi(), lpszentry.param().abi(), lpszphonenumber.param().abi(), lpinfo as _) }
}
#[inline]
pub unsafe fn RasDialDlgW<P0, P1, P2>(lpszphonebook: P0, lpszentry: P1, lpszphonenumber: P2, lpinfo: *mut RASDIALDLG) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasdlg.dll" "system" fn RasDialDlgW(lpszphonebook : windows_core::PCWSTR, lpszentry : windows_core::PCWSTR, lpszphonenumber : windows_core::PCWSTR, lpinfo : *mut RASDIALDLG) -> windows_core::BOOL);
    unsafe { RasDialDlgW(lpszphonebook.param().abi(), lpszentry.param().abi(), lpszphonenumber.param().abi(), lpinfo as _) }
}
#[inline]
pub unsafe fn RasDialW<P1>(param0: Option<*const RASDIALEXTENSIONS>, param1: P1, param2: *const RASDIALPARAMSW, param3: u32, param4: Option<*const core::ffi::c_void>, param5: *mut HRASCONN) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasDialW(param0 : *const RASDIALEXTENSIONS, param1 : windows_core::PCWSTR, param2 : *const RASDIALPARAMSW, param3 : u32, param4 : *const core::ffi::c_void, param5 : *mut HRASCONN) -> u32);
    unsafe { RasDialW(param0.unwrap_or(core::mem::zeroed()) as _, param1.param().abi(), param2, param3, param4.unwrap_or(core::mem::zeroed()) as _, param5 as _) }
}
#[inline]
pub unsafe fn RasEditPhonebookEntryA<P1, P2>(param0: super::super::Foundation::HWND, param1: P1, param2: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasEditPhonebookEntryA(param0 : super::super::Foundation:: HWND, param1 : windows_core::PCSTR, param2 : windows_core::PCSTR) -> u32);
    unsafe { RasEditPhonebookEntryA(param0, param1.param().abi(), param2.param().abi()) }
}
#[inline]
pub unsafe fn RasEditPhonebookEntryW<P1, P2>(param0: super::super::Foundation::HWND, param1: P1, param2: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasEditPhonebookEntryW(param0 : super::super::Foundation:: HWND, param1 : windows_core::PCWSTR, param2 : windows_core::PCWSTR) -> u32);
    unsafe { RasEditPhonebookEntryW(param0, param1.param().abi(), param2.param().abi()) }
}
#[inline]
pub unsafe fn RasEntryDlgA<P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut RASENTRYDLGA) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasdlg.dll" "system" fn RasEntryDlgA(lpszphonebook : windows_core::PCSTR, lpszentry : windows_core::PCSTR, lpinfo : *mut RASENTRYDLGA) -> windows_core::BOOL);
    unsafe { RasEntryDlgA(lpszphonebook.param().abi(), lpszentry.param().abi(), lpinfo as _) }
}
#[inline]
pub unsafe fn RasEntryDlgW<P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut RASENTRYDLGW) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasdlg.dll" "system" fn RasEntryDlgW(lpszphonebook : windows_core::PCWSTR, lpszentry : windows_core::PCWSTR, lpinfo : *mut RASENTRYDLGW) -> windows_core::BOOL);
    unsafe { RasEntryDlgW(lpszphonebook.param().abi(), lpszentry.param().abi(), lpinfo as _) }
}
#[inline]
pub unsafe fn RasEnumAutodialAddressesA(lpprasautodialaddresses: Option<*mut windows_core::PSTR>, lpdwcbrasautodialaddresses: *mut u32, lpdwcrasautodialaddresses: *mut u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasEnumAutodialAddressesA(lpprasautodialaddresses : *mut windows_core::PSTR, lpdwcbrasautodialaddresses : *mut u32, lpdwcrasautodialaddresses : *mut u32) -> u32);
    unsafe { RasEnumAutodialAddressesA(lpprasautodialaddresses.unwrap_or(core::mem::zeroed()) as _, lpdwcbrasautodialaddresses as _, lpdwcrasautodialaddresses as _) }
}
#[inline]
pub unsafe fn RasEnumAutodialAddressesW(lpprasautodialaddresses: Option<*mut windows_core::PWSTR>, lpdwcbrasautodialaddresses: *mut u32, lpdwcrasautodialaddresses: *mut u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasEnumAutodialAddressesW(lpprasautodialaddresses : *mut windows_core::PWSTR, lpdwcbrasautodialaddresses : *mut u32, lpdwcrasautodialaddresses : *mut u32) -> u32);
    unsafe { RasEnumAutodialAddressesW(lpprasautodialaddresses.unwrap_or(core::mem::zeroed()) as _, lpdwcbrasautodialaddresses as _, lpdwcrasautodialaddresses as _) }
}
#[inline]
pub unsafe fn RasEnumConnectionsA(param0: Option<*mut RASCONNA>, param1: *mut u32, param2: *mut u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasEnumConnectionsA(param0 : *mut RASCONNA, param1 : *mut u32, param2 : *mut u32) -> u32);
    unsafe { RasEnumConnectionsA(param0.unwrap_or(core::mem::zeroed()) as _, param1 as _, param2 as _) }
}
#[inline]
pub unsafe fn RasEnumConnectionsW(param0: Option<*mut RASCONNW>, param1: *mut u32, param2: *mut u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasEnumConnectionsW(param0 : *mut RASCONNW, param1 : *mut u32, param2 : *mut u32) -> u32);
    unsafe { RasEnumConnectionsW(param0.unwrap_or(core::mem::zeroed()) as _, param1 as _, param2 as _) }
}
#[inline]
pub unsafe fn RasEnumDevicesA(param0: Option<*mut RASDEVINFOA>, param1: *mut u32, param2: *mut u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasEnumDevicesA(param0 : *mut RASDEVINFOA, param1 : *mut u32, param2 : *mut u32) -> u32);
    unsafe { RasEnumDevicesA(param0.unwrap_or(core::mem::zeroed()) as _, param1 as _, param2 as _) }
}
#[inline]
pub unsafe fn RasEnumDevicesW(param0: Option<*mut RASDEVINFOW>, param1: *mut u32, param2: *mut u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasEnumDevicesW(param0 : *mut RASDEVINFOW, param1 : *mut u32, param2 : *mut u32) -> u32);
    unsafe { RasEnumDevicesW(param0.unwrap_or(core::mem::zeroed()) as _, param1 as _, param2 as _) }
}
#[inline]
pub unsafe fn RasEnumEntriesA<P0, P1>(param0: P0, param1: P1, param2: Option<*mut RASENTRYNAMEA>, param3: *mut u32, param4: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasEnumEntriesA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR, param2 : *mut RASENTRYNAMEA, param3 : *mut u32, param4 : *mut u32) -> u32);
    unsafe { RasEnumEntriesA(param0.param().abi(), param1.param().abi(), param2.unwrap_or(core::mem::zeroed()) as _, param3 as _, param4 as _) }
}
#[inline]
pub unsafe fn RasEnumEntriesW<P0, P1>(param0: P0, param1: P1, param2: Option<*mut RASENTRYNAMEW>, param3: *mut u32, param4: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasEnumEntriesW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR, param2 : *mut RASENTRYNAMEW, param3 : *mut u32, param4 : *mut u32) -> u32);
    unsafe { RasEnumEntriesW(param0.param().abi(), param1.param().abi(), param2.unwrap_or(core::mem::zeroed()) as _, param3 as _, param4 as _) }
}
#[inline]
pub unsafe fn RasFreeEapUserIdentityA(praseapuseridentity: *const RASEAPUSERIDENTITYA) {
    windows_link::link!("rasapi32.dll" "system" fn RasFreeEapUserIdentityA(praseapuseridentity : *const RASEAPUSERIDENTITYA));
    unsafe { RasFreeEapUserIdentityA(praseapuseridentity) }
}
#[inline]
pub unsafe fn RasFreeEapUserIdentityW(praseapuseridentity: *const RASEAPUSERIDENTITYW) {
    windows_link::link!("rasapi32.dll" "system" fn RasFreeEapUserIdentityW(praseapuseridentity : *const RASEAPUSERIDENTITYW));
    unsafe { RasFreeEapUserIdentityW(praseapuseridentity) }
}
#[inline]
pub unsafe fn RasGetAutodialAddressA<P0>(param0: P0, param1: Option<*const u32>, param2: Option<*mut RASAUTODIALENTRYA>, param3: *mut u32, param4: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialAddressA(param0 : windows_core::PCSTR, param1 : *const u32, param2 : *mut RASAUTODIALENTRYA, param3 : *mut u32, param4 : *mut u32) -> u32);
    unsafe { RasGetAutodialAddressA(param0.param().abi(), param1.unwrap_or(core::mem::zeroed()) as _, param2.unwrap_or(core::mem::zeroed()) as _, param3 as _, param4 as _) }
}
#[inline]
pub unsafe fn RasGetAutodialAddressW<P0>(param0: P0, param1: Option<*const u32>, param2: Option<*mut RASAUTODIALENTRYW>, param3: *mut u32, param4: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialAddressW(param0 : windows_core::PCWSTR, param1 : *const u32, param2 : *mut RASAUTODIALENTRYW, param3 : *mut u32, param4 : *mut u32) -> u32);
    unsafe { RasGetAutodialAddressW(param0.param().abi(), param1.unwrap_or(core::mem::zeroed()) as _, param2.unwrap_or(core::mem::zeroed()) as _, param3 as _, param4 as _) }
}
#[inline]
pub unsafe fn RasGetAutodialEnableA(param0: u32, param1: *mut windows_core::BOOL) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialEnableA(param0 : u32, param1 : *mut windows_core::BOOL) -> u32);
    unsafe { RasGetAutodialEnableA(param0, param1 as _) }
}
#[inline]
pub unsafe fn RasGetAutodialEnableW(param0: u32, param1: *mut windows_core::BOOL) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialEnableW(param0 : u32, param1 : *mut windows_core::BOOL) -> u32);
    unsafe { RasGetAutodialEnableW(param0, param1 as _) }
}
#[inline]
pub unsafe fn RasGetAutodialParamA(param0: u32, param1: *mut core::ffi::c_void, param2: *mut u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialParamA(param0 : u32, param1 : *mut core::ffi::c_void, param2 : *mut u32) -> u32);
    unsafe { RasGetAutodialParamA(param0, param1 as _, param2 as _) }
}
#[inline]
pub unsafe fn RasGetAutodialParamW(param0: u32, param1: *mut core::ffi::c_void, param2: *mut u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialParamW(param0 : u32, param1 : *mut core::ffi::c_void, param2 : *mut u32) -> u32);
    unsafe { RasGetAutodialParamW(param0, param1 as _, param2 as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasGetConnectStatusA(param0: HRASCONN, param1: *mut RASCONNSTATUSA) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetConnectStatusA(param0 : HRASCONN, param1 : *mut RASCONNSTATUSA) -> u32);
    unsafe { RasGetConnectStatusA(param0, param1 as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasGetConnectStatusW(param0: HRASCONN, param1: *mut RASCONNSTATUSW) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetConnectStatusW(param0 : HRASCONN, param1 : *mut RASCONNSTATUSW) -> u32);
    unsafe { RasGetConnectStatusW(param0, param1 as _) }
}
#[inline]
pub unsafe fn RasGetConnectionStatistics(hrasconn: HRASCONN, lpstatistics: *mut RAS_STATS) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetConnectionStatistics(hrasconn : HRASCONN, lpstatistics : *mut RAS_STATS) -> u32);
    unsafe { RasGetConnectionStatistics(hrasconn, lpstatistics as _) }
}
#[inline]
pub unsafe fn RasGetCountryInfoA(param0: Option<*mut RASCTRYINFO>, param1: *mut u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetCountryInfoA(param0 : *mut RASCTRYINFO, param1 : *mut u32) -> u32);
    unsafe { RasGetCountryInfoA(param0.unwrap_or(core::mem::zeroed()) as _, param1 as _) }
}
#[inline]
pub unsafe fn RasGetCountryInfoW(param0: Option<*mut RASCTRYINFO>, param1: *mut u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetCountryInfoW(param0 : *mut RASCTRYINFO, param1 : *mut u32) -> u32);
    unsafe { RasGetCountryInfoW(param0.unwrap_or(core::mem::zeroed()) as _, param1 as _) }
}
#[inline]
pub unsafe fn RasGetCredentialsA<P0, P1>(param0: P0, param1: P1, param2: *mut RASCREDENTIALSA) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasGetCredentialsA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR, param2 : *mut RASCREDENTIALSA) -> u32);
    unsafe { RasGetCredentialsA(param0.param().abi(), param1.param().abi(), param2 as _) }
}
#[inline]
pub unsafe fn RasGetCredentialsW<P0, P1>(param0: P0, param1: P1, param2: *mut RASCREDENTIALSW) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasGetCredentialsW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR, param2 : *mut RASCREDENTIALSW) -> u32);
    unsafe { RasGetCredentialsW(param0.param().abi(), param1.param().abi(), param2 as _) }
}
#[inline]
pub unsafe fn RasGetCustomAuthDataA<P0, P1>(pszphonebook: P0, pszentry: P1, pbcustomauthdata: Option<*mut u8>, pdwsizeofcustomauthdata: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasGetCustomAuthDataA(pszphonebook : windows_core::PCSTR, pszentry : windows_core::PCSTR, pbcustomauthdata : *mut u8, pdwsizeofcustomauthdata : *mut u32) -> u32);
    unsafe { RasGetCustomAuthDataA(pszphonebook.param().abi(), pszentry.param().abi(), pbcustomauthdata.unwrap_or(core::mem::zeroed()) as _, pdwsizeofcustomauthdata as _) }
}
#[inline]
pub unsafe fn RasGetCustomAuthDataW<P0, P1>(pszphonebook: P0, pszentry: P1, pbcustomauthdata: Option<*mut u8>, pdwsizeofcustomauthdata: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasGetCustomAuthDataW(pszphonebook : windows_core::PCWSTR, pszentry : windows_core::PCWSTR, pbcustomauthdata : *mut u8, pdwsizeofcustomauthdata : *mut u32) -> u32);
    unsafe { RasGetCustomAuthDataW(pszphonebook.param().abi(), pszentry.param().abi(), pbcustomauthdata.unwrap_or(core::mem::zeroed()) as _, pdwsizeofcustomauthdata as _) }
}
#[inline]
pub unsafe fn RasGetEapUserDataA<P1, P2>(htoken: Option<super::super::Foundation::HANDLE>, pszphonebook: P1, pszentry: P2, pbeapdata: Option<*mut u8>, pdwsizeofeapdata: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasGetEapUserDataA(htoken : super::super::Foundation:: HANDLE, pszphonebook : windows_core::PCSTR, pszentry : windows_core::PCSTR, pbeapdata : *mut u8, pdwsizeofeapdata : *mut u32) -> u32);
    unsafe { RasGetEapUserDataA(htoken.unwrap_or(core::mem::zeroed()) as _, pszphonebook.param().abi(), pszentry.param().abi(), pbeapdata.unwrap_or(core::mem::zeroed()) as _, pdwsizeofeapdata as _) }
}
#[inline]
pub unsafe fn RasGetEapUserDataW<P1, P2>(htoken: Option<super::super::Foundation::HANDLE>, pszphonebook: P1, pszentry: P2, pbeapdata: Option<*mut u8>, pdwsizeofeapdata: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasGetEapUserDataW(htoken : super::super::Foundation:: HANDLE, pszphonebook : windows_core::PCWSTR, pszentry : windows_core::PCWSTR, pbeapdata : *mut u8, pdwsizeofeapdata : *mut u32) -> u32);
    unsafe { RasGetEapUserDataW(htoken.unwrap_or(core::mem::zeroed()) as _, pszphonebook.param().abi(), pszentry.param().abi(), pbeapdata.unwrap_or(core::mem::zeroed()) as _, pdwsizeofeapdata as _) }
}
#[inline]
pub unsafe fn RasGetEapUserIdentityA<P0, P1>(pszphonebook: P0, pszentry: P1, dwflags: u32, hwnd: super::super::Foundation::HWND, ppraseapuseridentity: *mut *mut RASEAPUSERIDENTITYA) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasGetEapUserIdentityA(pszphonebook : windows_core::PCSTR, pszentry : windows_core::PCSTR, dwflags : u32, hwnd : super::super::Foundation:: HWND, ppraseapuseridentity : *mut *mut RASEAPUSERIDENTITYA) -> u32);
    unsafe { RasGetEapUserIdentityA(pszphonebook.param().abi(), pszentry.param().abi(), dwflags, hwnd, ppraseapuseridentity as _) }
}
#[inline]
pub unsafe fn RasGetEapUserIdentityW<P0, P1>(pszphonebook: P0, pszentry: P1, dwflags: u32, hwnd: super::super::Foundation::HWND, ppraseapuseridentity: *mut *mut RASEAPUSERIDENTITYW) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasGetEapUserIdentityW(pszphonebook : windows_core::PCWSTR, pszentry : windows_core::PCWSTR, dwflags : u32, hwnd : super::super::Foundation:: HWND, ppraseapuseridentity : *mut *mut RASEAPUSERIDENTITYW) -> u32);
    unsafe { RasGetEapUserIdentityW(pszphonebook.param().abi(), pszentry.param().abi(), dwflags, hwnd, ppraseapuseridentity as _) }
}
#[inline]
pub unsafe fn RasGetEntryDialParamsA<P0>(param0: P0, param1: *mut RASDIALPARAMSA, param2: *mut windows_core::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasGetEntryDialParamsA(param0 : windows_core::PCSTR, param1 : *mut RASDIALPARAMSA, param2 : *mut windows_core::BOOL) -> u32);
    unsafe { RasGetEntryDialParamsA(param0.param().abi(), param1 as _, param2 as _) }
}
#[inline]
pub unsafe fn RasGetEntryDialParamsW<P0>(param0: P0, param1: *mut RASDIALPARAMSW, param2: *mut windows_core::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasGetEntryDialParamsW(param0 : windows_core::PCWSTR, param1 : *mut RASDIALPARAMSW, param2 : *mut windows_core::BOOL) -> u32);
    unsafe { RasGetEntryDialParamsW(param0.param().abi(), param1 as _, param2 as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasGetEntryPropertiesA<P0, P1>(param0: P0, param1: P1, param2: Option<*mut RASENTRYA>, param3: *mut u32, param4: Option<*mut u8>, param5: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasGetEntryPropertiesA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR, param2 : *mut RASENTRYA, param3 : *mut u32, param4 : *mut u8, param5 : *mut u32) -> u32);
    unsafe { RasGetEntryPropertiesA(param0.param().abi(), param1.param().abi(), param2.unwrap_or(core::mem::zeroed()) as _, param3 as _, param4.unwrap_or(core::mem::zeroed()) as _, param5.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasGetEntryPropertiesW<P0, P1>(param0: P0, param1: P1, param2: Option<*mut RASENTRYW>, param3: *mut u32, param4: Option<*mut u8>, param5: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasGetEntryPropertiesW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR, param2 : *mut RASENTRYW, param3 : *mut u32, param4 : *mut u8, param5 : *mut u32) -> u32);
    unsafe { RasGetEntryPropertiesW(param0.param().abi(), param1.param().abi(), param2.unwrap_or(core::mem::zeroed()) as _, param3 as _, param4.unwrap_or(core::mem::zeroed()) as _, param5.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RasGetErrorStringA(resourceid: u32, lpszstring: &mut [u8]) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetErrorStringA(resourceid : u32, lpszstring : windows_core::PSTR, inbufsize : u32) -> u32);
    unsafe { RasGetErrorStringA(resourceid, core::mem::transmute(lpszstring.as_ptr()), lpszstring.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn RasGetErrorStringW(resourceid: u32, lpszstring: &mut [u16]) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetErrorStringW(resourceid : u32, lpszstring : windows_core::PWSTR, inbufsize : u32) -> u32);
    unsafe { RasGetErrorStringW(resourceid, core::mem::transmute(lpszstring.as_ptr()), lpszstring.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn RasGetLinkStatistics(hrasconn: HRASCONN, dwsubentry: u32, lpstatistics: *mut RAS_STATS) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetLinkStatistics(hrasconn : HRASCONN, dwsubentry : u32, lpstatistics : *mut RAS_STATS) -> u32);
    unsafe { RasGetLinkStatistics(hrasconn, dwsubentry, lpstatistics as _) }
}
#[inline]
pub unsafe fn RasGetPCscf(lpszpcscf: windows_core::PWSTR) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetPCscf(lpszpcscf : windows_core::PWSTR) -> u32);
    unsafe { RasGetPCscf(core::mem::transmute(lpszpcscf)) }
}
#[inline]
pub unsafe fn RasGetProjectionInfoA(param0: HRASCONN, param1: RASPROJECTION, param2: *mut core::ffi::c_void, param3: *mut u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetProjectionInfoA(param0 : HRASCONN, param1 : RASPROJECTION, param2 : *mut core::ffi::c_void, param3 : *mut u32) -> u32);
    unsafe { RasGetProjectionInfoA(param0, param1, param2 as _, param3 as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasGetProjectionInfoEx(hrasconn: HRASCONN, prasprojection: Option<*mut RAS_PROJECTION_INFO>, lpdwsize: *mut u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetProjectionInfoEx(hrasconn : HRASCONN, prasprojection : *mut RAS_PROJECTION_INFO, lpdwsize : *mut u32) -> u32);
    unsafe { RasGetProjectionInfoEx(hrasconn, prasprojection.unwrap_or(core::mem::zeroed()) as _, lpdwsize as _) }
}
#[inline]
pub unsafe fn RasGetProjectionInfoW(param0: HRASCONN, param1: RASPROJECTION, param2: *mut core::ffi::c_void, param3: *mut u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetProjectionInfoW(param0 : HRASCONN, param1 : RASPROJECTION, param2 : *mut core::ffi::c_void, param3 : *mut u32) -> u32);
    unsafe { RasGetProjectionInfoW(param0, param1, param2 as _, param3 as _) }
}
#[inline]
pub unsafe fn RasGetSubEntryHandleA(param0: HRASCONN, param1: u32, param2: *mut HRASCONN) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetSubEntryHandleA(param0 : HRASCONN, param1 : u32, param2 : *mut HRASCONN) -> u32);
    unsafe { RasGetSubEntryHandleA(param0, param1, param2 as _) }
}
#[inline]
pub unsafe fn RasGetSubEntryHandleW(param0: HRASCONN, param1: u32, param2: *mut HRASCONN) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasGetSubEntryHandleW(param0 : HRASCONN, param1 : u32, param2 : *mut HRASCONN) -> u32);
    unsafe { RasGetSubEntryHandleW(param0, param1, param2 as _) }
}
#[inline]
pub unsafe fn RasGetSubEntryPropertiesA<P0, P1>(param0: P0, param1: P1, param2: u32, param3: Option<*mut RASSUBENTRYA>, param4: Option<*mut u32>, param5: Option<*mut u8>, param6: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasGetSubEntryPropertiesA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR, param2 : u32, param3 : *mut RASSUBENTRYA, param4 : *mut u32, param5 : *mut u8, param6 : *mut u32) -> u32);
    unsafe { RasGetSubEntryPropertiesA(param0.param().abi(), param1.param().abi(), param2, param3.unwrap_or(core::mem::zeroed()) as _, param4.unwrap_or(core::mem::zeroed()) as _, param5.unwrap_or(core::mem::zeroed()) as _, param6.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RasGetSubEntryPropertiesW<P0, P1>(param0: P0, param1: P1, param2: u32, param3: Option<*mut RASSUBENTRYW>, param4: Option<*mut u32>, param5: Option<*mut u8>, param6: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasGetSubEntryPropertiesW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR, param2 : u32, param3 : *mut RASSUBENTRYW, param4 : *mut u32, param5 : *mut u8, param6 : *mut u32) -> u32);
    unsafe { RasGetSubEntryPropertiesW(param0.param().abi(), param1.param().abi(), param2, param3.unwrap_or(core::mem::zeroed()) as _, param4.unwrap_or(core::mem::zeroed()) as _, param5.unwrap_or(core::mem::zeroed()) as _, param6.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RasHangUpA(param0: HRASCONN) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasHangUpA(param0 : HRASCONN) -> u32);
    unsafe { RasHangUpA(param0) }
}
#[inline]
pub unsafe fn RasHangUpW(param0: HRASCONN) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasHangUpW(param0 : HRASCONN) -> u32);
    unsafe { RasHangUpW(param0) }
}
#[inline]
pub unsafe fn RasInvokeEapUI(param0: HRASCONN, param1: u32, param2: *const RASDIALEXTENSIONS, param3: super::super::Foundation::HWND) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasInvokeEapUI(param0 : HRASCONN, param1 : u32, param2 : *const RASDIALEXTENSIONS, param3 : super::super::Foundation:: HWND) -> u32);
    unsafe { RasInvokeEapUI(param0, param1, param2, param3) }
}
#[inline]
pub unsafe fn RasPhonebookDlgA<P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut RASPBDLGA) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasdlg.dll" "system" fn RasPhonebookDlgA(lpszphonebook : windows_core::PCSTR, lpszentry : windows_core::PCSTR, lpinfo : *mut RASPBDLGA) -> windows_core::BOOL);
    unsafe { RasPhonebookDlgA(lpszphonebook.param().abi(), lpszentry.param().abi(), lpinfo as _) }
}
#[inline]
pub unsafe fn RasPhonebookDlgW<P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut RASPBDLGW) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasdlg.dll" "system" fn RasPhonebookDlgW(lpszphonebook : windows_core::PCWSTR, lpszentry : windows_core::PCWSTR, lpinfo : *mut RASPBDLGW) -> windows_core::BOOL);
    unsafe { RasPhonebookDlgW(lpszphonebook.param().abi(), lpszentry.param().abi(), lpinfo as _) }
}
#[inline]
pub unsafe fn RasRenameEntryA<P0, P1, P2>(param0: P0, param1: P1, param2: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasRenameEntryA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR, param2 : windows_core::PCSTR) -> u32);
    unsafe { RasRenameEntryA(param0.param().abi(), param1.param().abi(), param2.param().abi()) }
}
#[inline]
pub unsafe fn RasRenameEntryW<P0, P1, P2>(param0: P0, param1: P1, param2: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasRenameEntryW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR, param2 : windows_core::PCWSTR) -> u32);
    unsafe { RasRenameEntryW(param0.param().abi(), param1.param().abi(), param2.param().abi()) }
}
#[inline]
pub unsafe fn RasSetAutodialAddressA<P0>(param0: P0, param1: u32, param2: Option<*const RASAUTODIALENTRYA>, param3: u32, param4: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialAddressA(param0 : windows_core::PCSTR, param1 : u32, param2 : *const RASAUTODIALENTRYA, param3 : u32, param4 : u32) -> u32);
    unsafe { RasSetAutodialAddressA(param0.param().abi(), param1, param2.unwrap_or(core::mem::zeroed()) as _, param3, param4) }
}
#[inline]
pub unsafe fn RasSetAutodialAddressW<P0>(param0: P0, param1: u32, param2: Option<*const RASAUTODIALENTRYW>, param3: u32, param4: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialAddressW(param0 : windows_core::PCWSTR, param1 : u32, param2 : *const RASAUTODIALENTRYW, param3 : u32, param4 : u32) -> u32);
    unsafe { RasSetAutodialAddressW(param0.param().abi(), param1, param2.unwrap_or(core::mem::zeroed()) as _, param3, param4) }
}
#[inline]
pub unsafe fn RasSetAutodialEnableA(param0: u32, param1: bool) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialEnableA(param0 : u32, param1 : windows_core::BOOL) -> u32);
    unsafe { RasSetAutodialEnableA(param0, param1.into()) }
}
#[inline]
pub unsafe fn RasSetAutodialEnableW(param0: u32, param1: bool) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialEnableW(param0 : u32, param1 : windows_core::BOOL) -> u32);
    unsafe { RasSetAutodialEnableW(param0, param1.into()) }
}
#[inline]
pub unsafe fn RasSetAutodialParamA(param0: u32, param1: *const core::ffi::c_void, param2: u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialParamA(param0 : u32, param1 : *const core::ffi::c_void, param2 : u32) -> u32);
    unsafe { RasSetAutodialParamA(param0, param1, param2) }
}
#[inline]
pub unsafe fn RasSetAutodialParamW(param0: u32, param1: *const core::ffi::c_void, param2: u32) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialParamW(param0 : u32, param1 : *const core::ffi::c_void, param2 : u32) -> u32);
    unsafe { RasSetAutodialParamW(param0, param1, param2) }
}
#[inline]
pub unsafe fn RasSetCredentialsA<P0, P1>(param0: P0, param1: P1, param2: *const RASCREDENTIALSA, param3: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasSetCredentialsA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR, param2 : *const RASCREDENTIALSA, param3 : windows_core::BOOL) -> u32);
    unsafe { RasSetCredentialsA(param0.param().abi(), param1.param().abi(), param2, param3.into()) }
}
#[inline]
pub unsafe fn RasSetCredentialsW<P0, P1>(param0: P0, param1: P1, param2: *const RASCREDENTIALSW, param3: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasSetCredentialsW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR, param2 : *const RASCREDENTIALSW, param3 : windows_core::BOOL) -> u32);
    unsafe { RasSetCredentialsW(param0.param().abi(), param1.param().abi(), param2, param3.into()) }
}
#[inline]
pub unsafe fn RasSetCustomAuthDataA<P0, P1>(pszphonebook: P0, pszentry: P1, pbcustomauthdata: &[u8]) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasSetCustomAuthDataA(pszphonebook : windows_core::PCSTR, pszentry : windows_core::PCSTR, pbcustomauthdata : *const u8, dwsizeofcustomauthdata : u32) -> u32);
    unsafe { RasSetCustomAuthDataA(pszphonebook.param().abi(), pszentry.param().abi(), core::mem::transmute(pbcustomauthdata.as_ptr()), pbcustomauthdata.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn RasSetCustomAuthDataW<P0, P1>(pszphonebook: P0, pszentry: P1, pbcustomauthdata: &[u8]) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasSetCustomAuthDataW(pszphonebook : windows_core::PCWSTR, pszentry : windows_core::PCWSTR, pbcustomauthdata : *const u8, dwsizeofcustomauthdata : u32) -> u32);
    unsafe { RasSetCustomAuthDataW(pszphonebook.param().abi(), pszentry.param().abi(), core::mem::transmute(pbcustomauthdata.as_ptr()), pbcustomauthdata.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn RasSetEapUserDataA<P1, P2>(htoken: Option<super::super::Foundation::HANDLE>, pszphonebook: P1, pszentry: P2, pbeapdata: *const u8, dwsizeofeapdata: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasSetEapUserDataA(htoken : super::super::Foundation:: HANDLE, pszphonebook : windows_core::PCSTR, pszentry : windows_core::PCSTR, pbeapdata : *const u8, dwsizeofeapdata : u32) -> u32);
    unsafe { RasSetEapUserDataA(htoken.unwrap_or(core::mem::zeroed()) as _, pszphonebook.param().abi(), pszentry.param().abi(), pbeapdata, dwsizeofeapdata) }
}
#[inline]
pub unsafe fn RasSetEapUserDataW<P1, P2>(htoken: Option<super::super::Foundation::HANDLE>, pszphonebook: P1, pszentry: P2, pbeapdata: *const u8, dwsizeofeapdata: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasSetEapUserDataW(htoken : super::super::Foundation:: HANDLE, pszphonebook : windows_core::PCWSTR, pszentry : windows_core::PCWSTR, pbeapdata : *const u8, dwsizeofeapdata : u32) -> u32);
    unsafe { RasSetEapUserDataW(htoken.unwrap_or(core::mem::zeroed()) as _, pszphonebook.param().abi(), pszentry.param().abi(), pbeapdata, dwsizeofeapdata) }
}
#[inline]
pub unsafe fn RasSetEntryDialParamsA<P0>(param0: P0, param1: *const RASDIALPARAMSA, param2: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasSetEntryDialParamsA(param0 : windows_core::PCSTR, param1 : *const RASDIALPARAMSA, param2 : windows_core::BOOL) -> u32);
    unsafe { RasSetEntryDialParamsA(param0.param().abi(), param1, param2.into()) }
}
#[inline]
pub unsafe fn RasSetEntryDialParamsW<P0>(param0: P0, param1: *const RASDIALPARAMSW, param2: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasSetEntryDialParamsW(param0 : windows_core::PCWSTR, param1 : *const RASDIALPARAMSW, param2 : windows_core::BOOL) -> u32);
    unsafe { RasSetEntryDialParamsW(param0.param().abi(), param1, param2.into()) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasSetEntryPropertiesA<P0, P1>(param0: P0, param1: P1, param2: *const RASENTRYA, param3: u32, param4: Option<*const u8>, param5: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasSetEntryPropertiesA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR, param2 : *const RASENTRYA, param3 : u32, param4 : *const u8, param5 : u32) -> u32);
    unsafe { RasSetEntryPropertiesA(param0.param().abi(), param1.param().abi(), param2, param3, param4.unwrap_or(core::mem::zeroed()) as _, param5) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasSetEntryPropertiesW<P0, P1>(param0: P0, param1: P1, param2: *const RASENTRYW, param3: u32, param4: Option<*const u8>, param5: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasSetEntryPropertiesW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR, param2 : *const RASENTRYW, param3 : u32, param4 : *const u8, param5 : u32) -> u32);
    unsafe { RasSetEntryPropertiesW(param0.param().abi(), param1.param().abi(), param2, param3, param4.unwrap_or(core::mem::zeroed()) as _, param5) }
}
#[inline]
pub unsafe fn RasSetSubEntryPropertiesA<P0, P1>(param0: P0, param1: P1, param2: u32, param3: *const RASSUBENTRYA, param4: u32, param5: Option<*const u8>, param6: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasSetSubEntryPropertiesA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR, param2 : u32, param3 : *const RASSUBENTRYA, param4 : u32, param5 : *const u8, param6 : u32) -> u32);
    unsafe { RasSetSubEntryPropertiesA(param0.param().abi(), param1.param().abi(), param2, param3, param4, param5.unwrap_or(core::mem::zeroed()) as _, param6) }
}
#[inline]
pub unsafe fn RasSetSubEntryPropertiesW<P0, P1>(param0: P0, param1: P1, param2: u32, param3: *const RASSUBENTRYW, param4: u32, param5: Option<*const u8>, param6: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasSetSubEntryPropertiesW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR, param2 : u32, param3 : *const RASSUBENTRYW, param4 : u32, param5 : *const u8, param6 : u32) -> u32);
    unsafe { RasSetSubEntryPropertiesW(param0.param().abi(), param1.param().abi(), param2, param3, param4, param5.unwrap_or(core::mem::zeroed()) as _, param6) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasUpdateConnection(hrasconn: HRASCONN, lprasupdateconn: *const RASUPDATECONN) -> u32 {
    windows_link::link!("rasapi32.dll" "system" fn RasUpdateConnection(hrasconn : HRASCONN, lprasupdateconn : *const RASUPDATECONN) -> u32);
    unsafe { RasUpdateConnection(hrasconn, lprasupdateconn) }
}
#[inline]
pub unsafe fn RasValidateEntryNameA<P0, P1>(param0: P0, param1: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasValidateEntryNameA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR) -> u32);
    unsafe { RasValidateEntryNameA(param0.param().abi(), param1.param().abi()) }
}
#[inline]
pub unsafe fn RasValidateEntryNameW<P0, P1>(param0: P0, param1: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("rasapi32.dll" "system" fn RasValidateEntryNameW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR) -> u32);
    unsafe { RasValidateEntryNameW(param0.param().abi(), param1.param().abi()) }
}
#[inline]
pub unsafe fn RtmAddNextHop(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO, nexthophandle: *mut isize, changeflags: *mut u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmAddNextHop(rtmreghandle : isize, nexthopinfo : *mut RTM_NEXTHOP_INFO, nexthophandle : *mut isize, changeflags : *mut u32) -> u32);
    unsafe { RtmAddNextHop(rtmreghandle, nexthopinfo as _, nexthophandle as _, changeflags as _) }
}
#[inline]
pub unsafe fn RtmAddRouteToDest(rtmreghandle: isize, routehandle: *mut isize, destaddress: *mut RTM_NET_ADDRESS, routeinfo: *mut RTM_ROUTE_INFO, timetolive: u32, routelisthandle: isize, notifytype: u32, notifyhandle: isize, changeflags: *mut u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmAddRouteToDest(rtmreghandle : isize, routehandle : *mut isize, destaddress : *mut RTM_NET_ADDRESS, routeinfo : *mut RTM_ROUTE_INFO, timetolive : u32, routelisthandle : isize, notifytype : u32, notifyhandle : isize, changeflags : *mut u32) -> u32);
    unsafe { RtmAddRouteToDest(rtmreghandle, routehandle as _, destaddress as _, routeinfo as _, timetolive, routelisthandle, notifytype, notifyhandle, changeflags as _) }
}
#[inline]
pub unsafe fn RtmBlockMethods(rtmreghandle: isize, targethandle: super::super::Foundation::HANDLE, targettype: u8, blockingflag: u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmBlockMethods(rtmreghandle : isize, targethandle : super::super::Foundation:: HANDLE, targettype : u8, blockingflag : u32) -> u32);
    unsafe { RtmBlockMethods(rtmreghandle, targethandle, targettype, blockingflag) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RtmConvertIpv6AddressAndLengthToNetAddress(pnetaddress: *mut RTM_NET_ADDRESS, address: super::super::Networking::WinSock::IN6_ADDR, dwlength: u32, dwaddresssize: u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmConvertIpv6AddressAndLengthToNetAddress(pnetaddress : *mut RTM_NET_ADDRESS, address : super::super::Networking::WinSock:: IN6_ADDR, dwlength : u32, dwaddresssize : u32) -> u32);
    unsafe { RtmConvertIpv6AddressAndLengthToNetAddress(pnetaddress as _, core::mem::transmute(address), dwlength, dwaddresssize) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RtmConvertNetAddressToIpv6AddressAndLength(pnetaddress: *mut RTM_NET_ADDRESS, paddress: *mut super::super::Networking::WinSock::IN6_ADDR, plength: *mut u32, dwaddresssize: u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmConvertNetAddressToIpv6AddressAndLength(pnetaddress : *mut RTM_NET_ADDRESS, paddress : *mut super::super::Networking::WinSock:: IN6_ADDR, plength : *mut u32, dwaddresssize : u32) -> u32);
    unsafe { RtmConvertNetAddressToIpv6AddressAndLength(pnetaddress as _, paddress as _, plength as _, dwaddresssize) }
}
#[inline]
pub unsafe fn RtmCreateDestEnum(rtmreghandle: isize, targetviews: u32, enumflags: u32, netaddress: *mut RTM_NET_ADDRESS, protocolid: u32, rtmenumhandle: *mut isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmCreateDestEnum(rtmreghandle : isize, targetviews : u32, enumflags : u32, netaddress : *mut RTM_NET_ADDRESS, protocolid : u32, rtmenumhandle : *mut isize) -> u32);
    unsafe { RtmCreateDestEnum(rtmreghandle, targetviews, enumflags, netaddress as _, protocolid, rtmenumhandle as _) }
}
#[inline]
pub unsafe fn RtmCreateNextHopEnum(rtmreghandle: isize, enumflags: u32, netaddress: *mut RTM_NET_ADDRESS, rtmenumhandle: *mut isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmCreateNextHopEnum(rtmreghandle : isize, enumflags : u32, netaddress : *mut RTM_NET_ADDRESS, rtmenumhandle : *mut isize) -> u32);
    unsafe { RtmCreateNextHopEnum(rtmreghandle, enumflags, netaddress as _, rtmenumhandle as _) }
}
#[inline]
pub unsafe fn RtmCreateRouteEnum(rtmreghandle: isize, desthandle: isize, targetviews: u32, enumflags: u32, startdest: *mut RTM_NET_ADDRESS, matchingflags: u32, criteriaroute: *mut RTM_ROUTE_INFO, criteriainterface: u32, rtmenumhandle: *mut isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmCreateRouteEnum(rtmreghandle : isize, desthandle : isize, targetviews : u32, enumflags : u32, startdest : *mut RTM_NET_ADDRESS, matchingflags : u32, criteriaroute : *mut RTM_ROUTE_INFO, criteriainterface : u32, rtmenumhandle : *mut isize) -> u32);
    unsafe { RtmCreateRouteEnum(rtmreghandle, desthandle, targetviews, enumflags, startdest as _, matchingflags, criteriaroute as _, criteriainterface, rtmenumhandle as _) }
}
#[inline]
pub unsafe fn RtmCreateRouteList(rtmreghandle: isize, routelisthandle: *mut isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmCreateRouteList(rtmreghandle : isize, routelisthandle : *mut isize) -> u32);
    unsafe { RtmCreateRouteList(rtmreghandle, routelisthandle as _) }
}
#[inline]
pub unsafe fn RtmCreateRouteListEnum(rtmreghandle: isize, routelisthandle: isize, rtmenumhandle: *mut isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmCreateRouteListEnum(rtmreghandle : isize, routelisthandle : isize, rtmenumhandle : *mut isize) -> u32);
    unsafe { RtmCreateRouteListEnum(rtmreghandle, routelisthandle, rtmenumhandle as _) }
}
#[inline]
pub unsafe fn RtmDeleteEnumHandle(rtmreghandle: isize, enumhandle: isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmDeleteEnumHandle(rtmreghandle : isize, enumhandle : isize) -> u32);
    unsafe { RtmDeleteEnumHandle(rtmreghandle, enumhandle) }
}
#[inline]
pub unsafe fn RtmDeleteNextHop(rtmreghandle: isize, nexthophandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmDeleteNextHop(rtmreghandle : isize, nexthophandle : isize, nexthopinfo : *mut RTM_NEXTHOP_INFO) -> u32);
    unsafe { RtmDeleteNextHop(rtmreghandle, nexthophandle, nexthopinfo as _) }
}
#[inline]
pub unsafe fn RtmDeleteRouteList(rtmreghandle: isize, routelisthandle: isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmDeleteRouteList(rtmreghandle : isize, routelisthandle : isize) -> u32);
    unsafe { RtmDeleteRouteList(rtmreghandle, routelisthandle) }
}
#[inline]
pub unsafe fn RtmDeleteRouteToDest(rtmreghandle: isize, routehandle: isize, changeflags: *mut u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmDeleteRouteToDest(rtmreghandle : isize, routehandle : isize, changeflags : *mut u32) -> u32);
    unsafe { RtmDeleteRouteToDest(rtmreghandle, routehandle, changeflags as _) }
}
#[inline]
pub unsafe fn RtmDeregisterEntity(rtmreghandle: isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmDeregisterEntity(rtmreghandle : isize) -> u32);
    unsafe { RtmDeregisterEntity(rtmreghandle) }
}
#[inline]
pub unsafe fn RtmDeregisterFromChangeNotification(rtmreghandle: isize, notifyhandle: isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmDeregisterFromChangeNotification(rtmreghandle : isize, notifyhandle : isize) -> u32);
    unsafe { RtmDeregisterFromChangeNotification(rtmreghandle, notifyhandle) }
}
#[inline]
pub unsafe fn RtmFindNextHop(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO, nexthophandle: *mut isize, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmFindNextHop(rtmreghandle : isize, nexthopinfo : *mut RTM_NEXTHOP_INFO, nexthophandle : *mut isize, nexthoppointer : *mut *mut RTM_NEXTHOP_INFO) -> u32);
    unsafe { RtmFindNextHop(rtmreghandle, nexthopinfo as _, nexthophandle as _, nexthoppointer as _) }
}
#[inline]
pub unsafe fn RtmGetChangeStatus(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, changestatus: *mut windows_core::BOOL) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetChangeStatus(rtmreghandle : isize, notifyhandle : isize, desthandle : isize, changestatus : *mut windows_core::BOOL) -> u32);
    unsafe { RtmGetChangeStatus(rtmreghandle, notifyhandle, desthandle, changestatus as _) }
}
#[inline]
pub unsafe fn RtmGetChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: *mut u32, changeddests: *mut RTM_DEST_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetChangedDests(rtmreghandle : isize, notifyhandle : isize, numdests : *mut u32, changeddests : *mut RTM_DEST_INFO) -> u32);
    unsafe { RtmGetChangedDests(rtmreghandle, notifyhandle, numdests as _, changeddests as _) }
}
#[inline]
pub unsafe fn RtmGetDestInfo(rtmreghandle: isize, desthandle: isize, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetDestInfo(rtmreghandle : isize, desthandle : isize, protocolid : u32, targetviews : u32, destinfo : *mut RTM_DEST_INFO) -> u32);
    unsafe { RtmGetDestInfo(rtmreghandle, desthandle, protocolid, targetviews, destinfo as _) }
}
#[inline]
pub unsafe fn RtmGetEntityInfo(rtmreghandle: isize, entityhandle: isize, entityinfo: *mut RTM_ENTITY_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetEntityInfo(rtmreghandle : isize, entityhandle : isize, entityinfo : *mut RTM_ENTITY_INFO) -> u32);
    unsafe { RtmGetEntityInfo(rtmreghandle, entityhandle, entityinfo as _) }
}
#[inline]
pub unsafe fn RtmGetEntityMethods(rtmreghandle: isize, entityhandle: isize, nummethods: *mut u32, exptmethods: *mut RTM_ENTITY_EXPORT_METHOD) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetEntityMethods(rtmreghandle : isize, entityhandle : isize, nummethods : *mut u32, exptmethods : *mut RTM_ENTITY_EXPORT_METHOD) -> u32);
    unsafe { RtmGetEntityMethods(rtmreghandle, entityhandle, nummethods as _, exptmethods as _) }
}
#[inline]
pub unsafe fn RtmGetEnumDests(rtmreghandle: isize, enumhandle: isize, numdests: *mut u32, destinfos: *mut RTM_DEST_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetEnumDests(rtmreghandle : isize, enumhandle : isize, numdests : *mut u32, destinfos : *mut RTM_DEST_INFO) -> u32);
    unsafe { RtmGetEnumDests(rtmreghandle, enumhandle, numdests as _, destinfos as _) }
}
#[inline]
pub unsafe fn RtmGetEnumNextHops(rtmreghandle: isize, enumhandle: isize, numnexthops: *mut u32, nexthophandles: *mut isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetEnumNextHops(rtmreghandle : isize, enumhandle : isize, numnexthops : *mut u32, nexthophandles : *mut isize) -> u32);
    unsafe { RtmGetEnumNextHops(rtmreghandle, enumhandle, numnexthops as _, nexthophandles as _) }
}
#[inline]
pub unsafe fn RtmGetEnumRoutes(rtmreghandle: isize, enumhandle: isize, numroutes: *mut u32, routehandles: *mut isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetEnumRoutes(rtmreghandle : isize, enumhandle : isize, numroutes : *mut u32, routehandles : *mut isize) -> u32);
    unsafe { RtmGetEnumRoutes(rtmreghandle, enumhandle, numroutes as _, routehandles as _) }
}
#[inline]
pub unsafe fn RtmGetExactMatchDestination(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetExactMatchDestination(rtmreghandle : isize, destaddress : *mut RTM_NET_ADDRESS, protocolid : u32, targetviews : u32, destinfo : *mut RTM_DEST_INFO) -> u32);
    unsafe { RtmGetExactMatchDestination(rtmreghandle, destaddress as _, protocolid, targetviews, destinfo as _) }
}
#[inline]
pub unsafe fn RtmGetExactMatchRoute(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, matchingflags: u32, routeinfo: *mut RTM_ROUTE_INFO, interfaceindex: u32, targetviews: u32, routehandle: *mut isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetExactMatchRoute(rtmreghandle : isize, destaddress : *mut RTM_NET_ADDRESS, matchingflags : u32, routeinfo : *mut RTM_ROUTE_INFO, interfaceindex : u32, targetviews : u32, routehandle : *mut isize) -> u32);
    unsafe { RtmGetExactMatchRoute(rtmreghandle, destaddress as _, matchingflags, routeinfo as _, interfaceindex, targetviews, routehandle as _) }
}
#[inline]
pub unsafe fn RtmGetLessSpecificDestination(rtmreghandle: isize, desthandle: isize, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetLessSpecificDestination(rtmreghandle : isize, desthandle : isize, protocolid : u32, targetviews : u32, destinfo : *mut RTM_DEST_INFO) -> u32);
    unsafe { RtmGetLessSpecificDestination(rtmreghandle, desthandle, protocolid, targetviews, destinfo as _) }
}
#[inline]
pub unsafe fn RtmGetListEnumRoutes(rtmreghandle: isize, enumhandle: isize, numroutes: *mut u32, routehandles: *mut isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetListEnumRoutes(rtmreghandle : isize, enumhandle : isize, numroutes : *mut u32, routehandles : *mut isize) -> u32);
    unsafe { RtmGetListEnumRoutes(rtmreghandle, enumhandle, numroutes as _, routehandles as _) }
}
#[inline]
pub unsafe fn RtmGetMostSpecificDestination(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetMostSpecificDestination(rtmreghandle : isize, destaddress : *mut RTM_NET_ADDRESS, protocolid : u32, targetviews : u32, destinfo : *mut RTM_DEST_INFO) -> u32);
    unsafe { RtmGetMostSpecificDestination(rtmreghandle, destaddress as _, protocolid, targetviews, destinfo as _) }
}
#[inline]
pub unsafe fn RtmGetNextHopInfo(rtmreghandle: isize, nexthophandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetNextHopInfo(rtmreghandle : isize, nexthophandle : isize, nexthopinfo : *mut RTM_NEXTHOP_INFO) -> u32);
    unsafe { RtmGetNextHopInfo(rtmreghandle, nexthophandle, nexthopinfo as _) }
}
#[inline]
pub unsafe fn RtmGetNextHopPointer(rtmreghandle: isize, nexthophandle: isize, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetNextHopPointer(rtmreghandle : isize, nexthophandle : isize, nexthoppointer : *mut *mut RTM_NEXTHOP_INFO) -> u32);
    unsafe { RtmGetNextHopPointer(rtmreghandle, nexthophandle, nexthoppointer as _) }
}
#[inline]
pub unsafe fn RtmGetOpaqueInformationPointer(rtmreghandle: isize, desthandle: isize, opaqueinfopointer: *mut *mut core::ffi::c_void) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetOpaqueInformationPointer(rtmreghandle : isize, desthandle : isize, opaqueinfopointer : *mut *mut core::ffi::c_void) -> u32);
    unsafe { RtmGetOpaqueInformationPointer(rtmreghandle, desthandle, opaqueinfopointer as _) }
}
#[inline]
pub unsafe fn RtmGetRegisteredEntities(rtmreghandle: isize, numentities: *mut u32, entityhandles: *mut isize, entityinfos: *mut RTM_ENTITY_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetRegisteredEntities(rtmreghandle : isize, numentities : *mut u32, entityhandles : *mut isize, entityinfos : *mut RTM_ENTITY_INFO) -> u32);
    unsafe { RtmGetRegisteredEntities(rtmreghandle, numentities as _, entityhandles as _, entityinfos as _) }
}
#[inline]
pub unsafe fn RtmGetRouteInfo(rtmreghandle: isize, routehandle: isize, routeinfo: *mut RTM_ROUTE_INFO, destaddress: *mut RTM_NET_ADDRESS) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetRouteInfo(rtmreghandle : isize, routehandle : isize, routeinfo : *mut RTM_ROUTE_INFO, destaddress : *mut RTM_NET_ADDRESS) -> u32);
    unsafe { RtmGetRouteInfo(rtmreghandle, routehandle, routeinfo as _, destaddress as _) }
}
#[inline]
pub unsafe fn RtmGetRoutePointer(rtmreghandle: isize, routehandle: isize, routepointer: *mut *mut RTM_ROUTE_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmGetRoutePointer(rtmreghandle : isize, routehandle : isize, routepointer : *mut *mut RTM_ROUTE_INFO) -> u32);
    unsafe { RtmGetRoutePointer(rtmreghandle, routehandle, routepointer as _) }
}
#[inline]
pub unsafe fn RtmHoldDestination(rtmreghandle: isize, desthandle: isize, targetviews: u32, holdtime: u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmHoldDestination(rtmreghandle : isize, desthandle : isize, targetviews : u32, holdtime : u32) -> u32);
    unsafe { RtmHoldDestination(rtmreghandle, desthandle, targetviews, holdtime) }
}
#[inline]
pub unsafe fn RtmIgnoreChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: u32, changeddests: *mut isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmIgnoreChangedDests(rtmreghandle : isize, notifyhandle : isize, numdests : u32, changeddests : *mut isize) -> u32);
    unsafe { RtmIgnoreChangedDests(rtmreghandle, notifyhandle, numdests, changeddests as _) }
}
#[inline]
pub unsafe fn RtmInsertInRouteList(rtmreghandle: isize, routelisthandle: isize, numroutes: u32, routehandles: *mut isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmInsertInRouteList(rtmreghandle : isize, routelisthandle : isize, numroutes : u32, routehandles : *mut isize) -> u32);
    unsafe { RtmInsertInRouteList(rtmreghandle, routelisthandle, numroutes, routehandles as _) }
}
#[inline]
pub unsafe fn RtmInvokeMethod(rtmreghandle: isize, entityhandle: isize, input: *mut RTM_ENTITY_METHOD_INPUT, outputsize: *mut u32, output: *mut RTM_ENTITY_METHOD_OUTPUT) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmInvokeMethod(rtmreghandle : isize, entityhandle : isize, input : *mut RTM_ENTITY_METHOD_INPUT, outputsize : *mut u32, output : *mut RTM_ENTITY_METHOD_OUTPUT) -> u32);
    unsafe { RtmInvokeMethod(rtmreghandle, entityhandle, input as _, outputsize as _, output as _) }
}
#[inline]
pub unsafe fn RtmIsBestRoute(rtmreghandle: isize, routehandle: isize, bestinviews: *mut u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmIsBestRoute(rtmreghandle : isize, routehandle : isize, bestinviews : *mut u32) -> u32);
    unsafe { RtmIsBestRoute(rtmreghandle, routehandle, bestinviews as _) }
}
#[inline]
pub unsafe fn RtmIsMarkedForChangeNotification(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, destmarked: *mut windows_core::BOOL) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmIsMarkedForChangeNotification(rtmreghandle : isize, notifyhandle : isize, desthandle : isize, destmarked : *mut windows_core::BOOL) -> u32);
    unsafe { RtmIsMarkedForChangeNotification(rtmreghandle, notifyhandle, desthandle, destmarked as _) }
}
#[inline]
pub unsafe fn RtmLockDestination(rtmreghandle: isize, desthandle: isize, exclusive: bool, lockdest: bool) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmLockDestination(rtmreghandle : isize, desthandle : isize, exclusive : windows_core::BOOL, lockdest : windows_core::BOOL) -> u32);
    unsafe { RtmLockDestination(rtmreghandle, desthandle, exclusive.into(), lockdest.into()) }
}
#[inline]
pub unsafe fn RtmLockNextHop(rtmreghandle: isize, nexthophandle: isize, exclusive: bool, locknexthop: bool, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmLockNextHop(rtmreghandle : isize, nexthophandle : isize, exclusive : windows_core::BOOL, locknexthop : windows_core::BOOL, nexthoppointer : *mut *mut RTM_NEXTHOP_INFO) -> u32);
    unsafe { RtmLockNextHop(rtmreghandle, nexthophandle, exclusive.into(), locknexthop.into(), nexthoppointer as _) }
}
#[inline]
pub unsafe fn RtmLockRoute(rtmreghandle: isize, routehandle: isize, exclusive: bool, lockroute: bool, routepointer: *mut *mut RTM_ROUTE_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmLockRoute(rtmreghandle : isize, routehandle : isize, exclusive : windows_core::BOOL, lockroute : windows_core::BOOL, routepointer : *mut *mut RTM_ROUTE_INFO) -> u32);
    unsafe { RtmLockRoute(rtmreghandle, routehandle, exclusive.into(), lockroute.into(), routepointer as _) }
}
#[inline]
pub unsafe fn RtmMarkDestForChangeNotification(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, markdest: bool) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmMarkDestForChangeNotification(rtmreghandle : isize, notifyhandle : isize, desthandle : isize, markdest : windows_core::BOOL) -> u32);
    unsafe { RtmMarkDestForChangeNotification(rtmreghandle, notifyhandle, desthandle, markdest.into()) }
}
#[inline]
pub unsafe fn RtmReferenceHandles(rtmreghandle: isize, numhandles: u32, rtmhandles: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmReferenceHandles(rtmreghandle : isize, numhandles : u32, rtmhandles : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { RtmReferenceHandles(rtmreghandle, numhandles, rtmhandles as _) }
}
#[inline]
pub unsafe fn RtmRegisterEntity(rtmentityinfo: *mut RTM_ENTITY_INFO, exportmethods: *mut RTM_ENTITY_EXPORT_METHODS, eventcallback: RTM_EVENT_CALLBACK, reserveopaquepointer: bool, rtmregprofile: *mut RTM_REGN_PROFILE, rtmreghandle: *mut isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmRegisterEntity(rtmentityinfo : *mut RTM_ENTITY_INFO, exportmethods : *mut RTM_ENTITY_EXPORT_METHODS, eventcallback : RTM_EVENT_CALLBACK, reserveopaquepointer : windows_core::BOOL, rtmregprofile : *mut RTM_REGN_PROFILE, rtmreghandle : *mut isize) -> u32);
    unsafe { RtmRegisterEntity(rtmentityinfo as _, exportmethods as _, eventcallback, reserveopaquepointer.into(), rtmregprofile as _, rtmreghandle as _) }
}
#[inline]
pub unsafe fn RtmRegisterForChangeNotification(rtmreghandle: isize, targetviews: u32, notifyflags: u32, notifycontext: *mut core::ffi::c_void, notifyhandle: *mut isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmRegisterForChangeNotification(rtmreghandle : isize, targetviews : u32, notifyflags : u32, notifycontext : *mut core::ffi::c_void, notifyhandle : *mut isize) -> u32);
    unsafe { RtmRegisterForChangeNotification(rtmreghandle, targetviews, notifyflags, notifycontext as _, notifyhandle as _) }
}
#[inline]
pub unsafe fn RtmReleaseChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: u32, changeddests: *mut RTM_DEST_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmReleaseChangedDests(rtmreghandle : isize, notifyhandle : isize, numdests : u32, changeddests : *mut RTM_DEST_INFO) -> u32);
    unsafe { RtmReleaseChangedDests(rtmreghandle, notifyhandle, numdests, changeddests as _) }
}
#[inline]
pub unsafe fn RtmReleaseDestInfo(rtmreghandle: isize, destinfo: *mut RTM_DEST_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmReleaseDestInfo(rtmreghandle : isize, destinfo : *mut RTM_DEST_INFO) -> u32);
    unsafe { RtmReleaseDestInfo(rtmreghandle, destinfo as _) }
}
#[inline]
pub unsafe fn RtmReleaseDests(rtmreghandle: isize, numdests: u32, destinfos: *mut RTM_DEST_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmReleaseDests(rtmreghandle : isize, numdests : u32, destinfos : *mut RTM_DEST_INFO) -> u32);
    unsafe { RtmReleaseDests(rtmreghandle, numdests, destinfos as _) }
}
#[inline]
pub unsafe fn RtmReleaseEntities(rtmreghandle: isize, numentities: u32, entityhandles: *mut isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmReleaseEntities(rtmreghandle : isize, numentities : u32, entityhandles : *mut isize) -> u32);
    unsafe { RtmReleaseEntities(rtmreghandle, numentities, entityhandles as _) }
}
#[inline]
pub unsafe fn RtmReleaseEntityInfo(rtmreghandle: isize, entityinfo: *mut RTM_ENTITY_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmReleaseEntityInfo(rtmreghandle : isize, entityinfo : *mut RTM_ENTITY_INFO) -> u32);
    unsafe { RtmReleaseEntityInfo(rtmreghandle, entityinfo as _) }
}
#[inline]
pub unsafe fn RtmReleaseNextHopInfo(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmReleaseNextHopInfo(rtmreghandle : isize, nexthopinfo : *mut RTM_NEXTHOP_INFO) -> u32);
    unsafe { RtmReleaseNextHopInfo(rtmreghandle, nexthopinfo as _) }
}
#[inline]
pub unsafe fn RtmReleaseNextHops(rtmreghandle: isize, numnexthops: u32, nexthophandles: *mut isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmReleaseNextHops(rtmreghandle : isize, numnexthops : u32, nexthophandles : *mut isize) -> u32);
    unsafe { RtmReleaseNextHops(rtmreghandle, numnexthops, nexthophandles as _) }
}
#[inline]
pub unsafe fn RtmReleaseRouteInfo(rtmreghandle: isize, routeinfo: *mut RTM_ROUTE_INFO) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmReleaseRouteInfo(rtmreghandle : isize, routeinfo : *mut RTM_ROUTE_INFO) -> u32);
    unsafe { RtmReleaseRouteInfo(rtmreghandle, routeinfo as _) }
}
#[inline]
pub unsafe fn RtmReleaseRoutes(rtmreghandle: isize, numroutes: u32, routehandles: *mut isize) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmReleaseRoutes(rtmreghandle : isize, numroutes : u32, routehandles : *mut isize) -> u32);
    unsafe { RtmReleaseRoutes(rtmreghandle, numroutes, routehandles as _) }
}
#[inline]
pub unsafe fn RtmUpdateAndUnlockRoute(rtmreghandle: isize, routehandle: isize, timetolive: u32, routelisthandle: isize, notifytype: u32, notifyhandle: isize, changeflags: *mut u32) -> u32 {
    windows_link::link!("rtm.dll" "system" fn RtmUpdateAndUnlockRoute(rtmreghandle : isize, routehandle : isize, timetolive : u32, routelisthandle : isize, notifytype : u32, notifyhandle : isize, changeflags : *mut u32) -> u32);
    unsafe { RtmUpdateAndUnlockRoute(rtmreghandle, routehandle, timetolive, routelisthandle, notifytype, notifyhandle, changeflags as _) }
}
pub const ALLOW_NO_AUTH: u32 = 1u32;
pub const ALL_SOURCES: MGM_ENUM_TYPES = MGM_ENUM_TYPES(1i32);
pub const ANY_SOURCE: MGM_ENUM_TYPES = MGM_ENUM_TYPES(0i32);
pub const ATADDRESSLEN: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUTH_VALIDATION_EX {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub hRasConnection: super::super::Foundation::HANDLE,
    pub wszUserName: [u16; 257],
    pub wszLogonDomain: [u16; 16],
    pub AuthInfoSize: u32,
    pub AuthInfo: [u8; 1],
}
impl Default for AUTH_VALIDATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DO_NOT_ALLOW_NO_AUTH: u32 = 0u32;
pub const ERROR_ACCESSING_TCPCFGDLL: u32 = 727u32;
pub const ERROR_ACCT_DISABLED: u32 = 647u32;
pub const ERROR_ACCT_EXPIRED: u32 = 708u32;
pub const ERROR_ACTION_REQUIRED: u32 = 877u32;
pub const ERROR_ALLOCATING_MEMORY: u32 = 664u32;
pub const ERROR_ALREADY_DISCONNECTING: u32 = 617u32;
pub const ERROR_ASYNC_REQUEST_PENDING: u32 = 616u32;
pub const ERROR_AUTHENTICATION_FAILURE: u32 = 691u32;
pub const ERROR_AUTH_INTERNAL: u32 = 645u32;
pub const ERROR_AUTOMATIC_VPN_FAILED: u32 = 800u32;
pub const ERROR_BAD_ADDRESS_SPECIFIED: u32 = 769u32;
pub const ERROR_BAD_CALLBACK_NUMBER: u32 = 704u32;
pub const ERROR_BAD_PHONE_NUMBER: u32 = 749u32;
pub const ERROR_BAD_STRING: u32 = 637u32;
pub const ERROR_BAD_USAGE_IN_INI_FILE: u32 = 669u32;
pub const ERROR_BIPLEX_PORT_NOT_AVAILABLE: u32 = 712u32;
pub const ERROR_BLOCKED: u32 = 775u32;
pub const ERROR_BROADBAND_ACTIVE: u32 = 813u32;
pub const ERROR_BROADBAND_NO_NIC: u32 = 814u32;
pub const ERROR_BROADBAND_TIMEOUT: u32 = 815u32;
pub const ERROR_BUFFER_INVALID: u32 = 610u32;
pub const ERROR_BUFFER_TOO_SMALL: u32 = 603u32;
pub const ERROR_BUNDLE_NOT_FOUND: u32 = 754u32;
pub const ERROR_CANNOT_DELETE: u32 = 817u32;
pub const ERROR_CANNOT_DO_CUSTOMDIAL: u32 = 755u32;
pub const ERROR_CANNOT_FIND_PHONEBOOK_ENTRY: u32 = 623u32;
pub const ERROR_CANNOT_GET_LANA: u32 = 639u32;
pub const ERROR_CANNOT_INITIATE_MOBIKE_UPDATE: u32 = 844u32;
pub const ERROR_CANNOT_LOAD_PHONEBOOK: u32 = 622u32;
pub const ERROR_CANNOT_LOAD_STRING: u32 = 626u32;
pub const ERROR_CANNOT_OPEN_PHONEBOOK: u32 = 621u32;
pub const ERROR_CANNOT_PROJECT_CLIENT: u32 = 634u32;
pub const ERROR_CANNOT_SET_PORT_INFO: u32 = 605u32;
pub const ERROR_CANNOT_SHARE_CONNECTION: u32 = 763u32;
pub const ERROR_CANNOT_USE_LOGON_CREDENTIALS: u32 = 739u32;
pub const ERROR_CANNOT_WRITE_PHONEBOOK: u32 = 624u32;
pub const ERROR_CERT_FOR_ENCRYPTION_NOT_FOUND: u32 = 781u32;
pub const ERROR_CHANGING_PASSWORD: u32 = 709u32;
pub const ERROR_CMD_TOO_LONG: u32 = 700u32;
pub const ERROR_CONGESTION: u32 = 771u32;
pub const ERROR_CONNECTING_DEVICE_NOT_FOUND: u32 = 797u32;
pub const ERROR_CONNECTION_ALREADY_SHARED: u32 = 758u32;
pub const ERROR_CONNECTION_REJECT: u32 = 770u32;
pub const ERROR_CORRUPT_PHONEBOOK: u32 = 625u32;
pub const ERROR_DCB_NOT_FOUND: u32 = 694u32;
pub const ERROR_DEFAULTOFF_MACRO_NOT_FOUND: u32 = 656u32;
pub const ERROR_DEVICENAME_NOT_FOUND: u32 = 659u32;
pub const ERROR_DEVICENAME_TOO_LONG: u32 = 658u32;
pub const ERROR_DEVICETYPE_DOES_NOT_EXIST: u32 = 609u32;
pub const ERROR_DEVICE_COMPLIANCE: u32 = 875u32;
pub const ERROR_DEVICE_DOES_NOT_EXIST: u32 = 608u32;
pub const ERROR_DEVICE_NOT_READY: u32 = 666u32;
pub const ERROR_DIAL_ALREADY_IN_PROGRESS: u32 = 756u32;
pub const ERROR_DISCONNECTION: u32 = 628u32;
pub const ERROR_DNSNAME_NOT_RESOLVABLE: u32 = 868u32;
pub const ERROR_DONOTDISTURB: u32 = 776u32;
pub const ERROR_EAPTLS_CACHE_CREDENTIALS_INVALID: u32 = 826u32;
pub const ERROR_EAPTLS_PASSWD_INVALID: u32 = 869u32;
pub const ERROR_EAPTLS_SCARD_CACHE_CREDENTIALS_INVALID: u32 = 847u32;
pub const ERROR_EAP_METHOD_DOES_NOT_SUPPORT_SSO: u32 = 851u32;
pub const ERROR_EAP_METHOD_NOT_INSTALLED: u32 = 850u32;
pub const ERROR_EAP_METHOD_OPERATION_NOT_SUPPORTED: u32 = 852u32;
pub const ERROR_EAP_SERVER_CERT_EXPIRED: u32 = 858u32;
pub const ERROR_EAP_SERVER_CERT_INVALID: u32 = 857u32;
pub const ERROR_EAP_SERVER_CERT_OTHER_ERROR: u32 = 860u32;
pub const ERROR_EAP_SERVER_CERT_REVOKED: u32 = 859u32;
pub const ERROR_EAP_SERVER_ROOT_CERT_INVALID: u32 = 865u32;
pub const ERROR_EAP_SERVER_ROOT_CERT_NAME_REQUIRED: u32 = 866u32;
pub const ERROR_EAP_SERVER_ROOT_CERT_NOT_FOUND: u32 = 864u32;
pub const ERROR_EAP_USER_CERT_EXPIRED: u32 = 854u32;
pub const ERROR_EAP_USER_CERT_INVALID: u32 = 853u32;
pub const ERROR_EAP_USER_CERT_OTHER_ERROR: u32 = 856u32;
pub const ERROR_EAP_USER_CERT_REVOKED: u32 = 855u32;
pub const ERROR_EAP_USER_ROOT_CERT_EXPIRED: u32 = 863u32;
pub const ERROR_EAP_USER_ROOT_CERT_INVALID: u32 = 862u32;
pub const ERROR_EAP_USER_ROOT_CERT_NOT_FOUND: u32 = 861u32;
pub const ERROR_EMPTY_INI_FILE: u32 = 690u32;
pub const ERROR_EVENT_INVALID: u32 = 607u32;
pub const ERROR_FAILED_CP_REQUIRED: u32 = 841u32;
pub const ERROR_FAILED_TO_ENCRYPT: u32 = 768u32;
pub const ERROR_FAST_USER_SWITCH: u32 = 831u32;
pub const ERROR_FEATURE_DEPRECATED: u32 = 816u32;
pub const ERROR_FILE_COULD_NOT_BE_OPENED: u32 = 657u32;
pub const ERROR_FROM_DEVICE: u32 = 651u32;
pub const ERROR_HANGUP_FAILED: u32 = 753u32;
pub const ERROR_HARDWARE_FAILURE: u32 = 630u32;
pub const ERROR_HIBERNATION: u32 = 832u32;
pub const ERROR_IDLE_TIMEOUT: u32 = 828u32;
pub const ERROR_IKEV2_PSK_INTERFACE_ALREADY_EXISTS: u32 = 870u32;
pub const ERROR_INCOMPATIBLE: u32 = 772u32;
pub const ERROR_INTERACTIVE_MODE: u32 = 703u32;
pub const ERROR_INTERNAL_ADDRESS_FAILURE: u32 = 840u32;
pub const ERROR_INVALID_AUTH_STATE: u32 = 705u32;
pub const ERROR_INVALID_CALLBACK_NUMBER: u32 = 751u32;
pub const ERROR_INVALID_COMPRESSION_SPECIFIED: u32 = 613u32;
pub const ERROR_INVALID_DESTINATION_IP: u32 = 871u32;
pub const ERROR_INVALID_FUNCTION_FOR_ENTRY: u32 = 780u32;
pub const ERROR_INVALID_INTERFACE_CONFIG: u32 = 872u32;
pub const ERROR_INVALID_MSCHAPV2_CONFIG: u32 = 805u32;
pub const ERROR_INVALID_PEAP_COOKIE_ATTRIBUTES: u32 = 849u32;
pub const ERROR_INVALID_PEAP_COOKIE_CONFIG: u32 = 803u32;
pub const ERROR_INVALID_PEAP_COOKIE_USER: u32 = 804u32;
pub const ERROR_INVALID_PORT_HANDLE: u32 = 601u32;
pub const ERROR_INVALID_PREFERENCES: u32 = 846u32;
pub const ERROR_INVALID_SERVER_CERT: u32 = 835u32;
pub const ERROR_INVALID_SIZE: u32 = 632u32;
pub const ERROR_INVALID_SMM: u32 = 745u32;
pub const ERROR_INVALID_TUNNELID: u32 = 837u32;
pub const ERROR_INVALID_VPNSTRATEGY: u32 = 825u32;
pub const ERROR_IN_COMMAND: u32 = 681u32;
pub const ERROR_IPSEC_SERVICE_STOPPED: u32 = 827u32;
pub const ERROR_IPXCP_DIALOUT_ALREADY_ACTIVE: u32 = 726u32;
pub const ERROR_IPXCP_NET_NUMBER_CONFLICT: u32 = 744u32;
pub const ERROR_IPXCP_NO_DIALIN_CONFIGURED: u32 = 725u32;
pub const ERROR_IPXCP_NO_DIALOUT_CONFIGURED: u32 = 724u32;
pub const ERROR_IP_CONFIGURATION: u32 = 716u32;
pub const ERROR_KEY_NOT_FOUND: u32 = 627u32;
pub const ERROR_LINE_BUSY: u32 = 676u32;
pub const ERROR_LINK_FAILURE: u32 = 829u32;
pub const ERROR_MACRO_NOT_DEFINED: u32 = 654u32;
pub const ERROR_MACRO_NOT_FOUND: u32 = 653u32;
pub const ERROR_MESSAGE_MACRO_NOT_FOUND: u32 = 655u32;
pub const ERROR_MOBIKE_DISABLED: u32 = 843u32;
pub const ERROR_NAME_EXISTS_ON_NET: u32 = 642u32;
pub const ERROR_NETBIOS_ERROR: u32 = 640u32;
pub const ERROR_NOT_BINARY_MACRO: u32 = 693u32;
pub const ERROR_NOT_NAP_CAPABLE: u32 = 836u32;
pub const ERROR_NO_ACTIVE_ISDN_LINES: u32 = 713u32;
pub const ERROR_NO_ANSWER: u32 = 678u32;
pub const ERROR_NO_CARRIER: u32 = 679u32;
pub const ERROR_NO_CERTIFICATE: u32 = 766u32;
pub const ERROR_NO_COMMAND_FOUND: u32 = 661u32;
pub const ERROR_NO_CONNECTION: u32 = 668u32;
pub const ERROR_NO_DIALIN_PERMISSION: u32 = 649u32;
pub const ERROR_NO_DIALTONE: u32 = 680u32;
pub const ERROR_NO_DIFF_USER_AT_LOGON: u32 = 784u32;
pub const ERROR_NO_EAPTLS_CERTIFICATE: u32 = 798u32;
pub const ERROR_NO_ENDPOINTS: u32 = 620u32;
pub const ERROR_NO_IP_ADDRESSES: u32 = 717u32;
pub const ERROR_NO_IP_RAS_ADAPTER: u32 = 728u32;
pub const ERROR_NO_ISDN_CHANNELS_AVAILABLE: u32 = 714u32;
pub const ERROR_NO_LOCAL_ENCRYPTION: u32 = 741u32;
pub const ERROR_NO_MAC_FOR_PORT: u32 = 747u32;
pub const ERROR_NO_REG_CERT_AT_LOGON: u32 = 785u32;
pub const ERROR_NO_REMOTE_ENCRYPTION: u32 = 742u32;
pub const ERROR_NO_RESPONSES: u32 = 660u32;
pub const ERROR_NO_SMART_CARD_READER: u32 = 764u32;
pub const ERROR_NUMBERCHANGED: u32 = 773u32;
pub const ERROR_OAKLEY_ATTRIB_FAIL: u32 = 788u32;
pub const ERROR_OAKLEY_AUTH_FAIL: u32 = 787u32;
pub const ERROR_OAKLEY_ERROR: u32 = 793u32;
pub const ERROR_OAKLEY_GENERAL_PROCESSING: u32 = 789u32;
pub const ERROR_OAKLEY_NO_CERT: u32 = 786u32;
pub const ERROR_OAKLEY_NO_PEER_CERT: u32 = 790u32;
pub const ERROR_OAKLEY_NO_POLICY: u32 = 791u32;
pub const ERROR_OAKLEY_TIMED_OUT: u32 = 792u32;
pub const ERROR_OUTOFORDER: u32 = 777u32;
pub const ERROR_OUT_OF_BUFFERS: u32 = 614u32;
pub const ERROR_OVERRUN: u32 = 710u32;
pub const ERROR_PARTIAL_RESPONSE_LOOPING: u32 = 697u32;
pub const ERROR_PASSWD_EXPIRED: u32 = 648u32;
pub const ERROR_PEAP_CRYPTOBINDING_INVALID: u32 = 823u32;
pub const ERROR_PEAP_CRYPTOBINDING_NOTRECEIVED: u32 = 824u32;
pub const ERROR_PEAP_IDENTITY_MISMATCH: u32 = 867u32;
pub const ERROR_PEAP_SERVER_REJECTED_CLIENT_TLV: u32 = 845u32;
pub const ERROR_PHONE_NUMBER_TOO_LONG: u32 = 723u32;
pub const ERROR_PLUGIN_NOT_INSTALLED: u32 = 876u32;
pub const ERROR_PORT_ALREADY_OPEN: u32 = 602u32;
pub const ERROR_PORT_DISCONNECTED: u32 = 619u32;
pub const ERROR_PORT_NOT_AVAILABLE: u32 = 633u32;
pub const ERROR_PORT_NOT_CONFIGURED: u32 = 665u32;
pub const ERROR_PORT_NOT_CONNECTED: u32 = 606u32;
pub const ERROR_PORT_NOT_FOUND: u32 = 615u32;
pub const ERROR_PORT_NOT_OPEN: u32 = 618u32;
pub const ERROR_PORT_OR_DEVICE: u32 = 692u32;
pub const ERROR_PPP_CP_REJECTED: u32 = 733u32;
pub const ERROR_PPP_INVALID_PACKET: u32 = 722u32;
pub const ERROR_PPP_LCP_TERMINATED: u32 = 734u32;
pub const ERROR_PPP_LOOPBACK_DETECTED: u32 = 737u32;
pub const ERROR_PPP_NCP_TERMINATED: u32 = 736u32;
pub const ERROR_PPP_NOT_CONVERGING: u32 = 732u32;
pub const ERROR_PPP_NO_ADDRESS_ASSIGNED: u32 = 738u32;
pub const ERROR_PPP_NO_PROTOCOLS_CONFIGURED: u32 = 720u32;
pub const ERROR_PPP_NO_RESPONSE: u32 = 721u32;
pub const ERROR_PPP_REMOTE_TERMINATED: u32 = 719u32;
pub const ERROR_PPP_REQUIRED_ADDRESS_REJECTED: u32 = 735u32;
pub const ERROR_PPP_TIMEOUT: u32 = 718u32;
pub const ERROR_PROJECTION_NOT_COMPLETE: u32 = 730u32;
pub const ERROR_PROTOCOL_ENGINE_DISABLED: u32 = 839u32;
pub const ERROR_PROTOCOL_NOT_CONFIGURED: u32 = 731u32;
pub const ERROR_RASAUTO_CANNOT_INITIALIZE: u32 = 757u32;
pub const ERROR_RASMAN_CANNOT_INITIALIZE: u32 = 711u32;
pub const ERROR_RASMAN_SERVICE_STOPPED: u32 = 834u32;
pub const ERROR_RASQEC_CONN_DOESNOTEXIST: u32 = 821u32;
pub const ERROR_RASQEC_NAPAGENT_NOT_CONNECTED: u32 = 820u32;
pub const ERROR_RASQEC_NAPAGENT_NOT_ENABLED: u32 = 819u32;
pub const ERROR_RASQEC_RESOURCE_CREATION_FAILED: u32 = 818u32;
pub const ERROR_RASQEC_TIMEOUT: u32 = 822u32;
pub const ERROR_READING_DEFAULTOFF: u32 = 689u32;
pub const ERROR_READING_DEVICENAME: u32 = 672u32;
pub const ERROR_READING_DEVICETYPE: u32 = 671u32;
pub const ERROR_READING_INI_FILE: u32 = 667u32;
pub const ERROR_READING_MAXCARRIERBPS: u32 = 675u32;
pub const ERROR_READING_MAXCONNECTBPS: u32 = 674u32;
pub const ERROR_READING_SCARD: u32 = 802u32;
pub const ERROR_READING_SECTIONNAME: u32 = 670u32;
pub const ERROR_READING_USAGE: u32 = 673u32;
pub const ERROR_RECV_BUF_FULL: u32 = 699u32;
pub const ERROR_REMOTE_DISCONNECTION: u32 = 629u32;
pub const ERROR_REMOTE_REQUIRES_ENCRYPTION: u32 = 743u32;
pub const ERROR_REQUEST_TIMEOUT: u32 = 638u32;
pub const ERROR_RESTRICTED_LOGON_HOURS: u32 = 646u32;
pub const ERROR_ROUTE_NOT_ALLOCATED: u32 = 612u32;
pub const ERROR_ROUTE_NOT_AVAILABLE: u32 = 611u32;
pub const ERROR_SCRIPT_SYNTAX: u32 = 752u32;
pub const ERROR_SERVER_GENERAL_NET_FAILURE: u32 = 643u32;
pub const ERROR_SERVER_NOT_RESPONDING: u32 = 650u32;
pub const ERROR_SERVER_OUT_OF_RESOURCES: u32 = 641u32;
pub const ERROR_SERVER_POLICY: u32 = 812u32;
pub const ERROR_SHARE_CONNECTION_FAILED: u32 = 761u32;
pub const ERROR_SHARING_ADDRESS_EXISTS: u32 = 765u32;
pub const ERROR_SHARING_CHANGE_FAILED: u32 = 759u32;
pub const ERROR_SHARING_HOST_ADDRESS_CONFLICT: u32 = 799u32;
pub const ERROR_SHARING_MULTIPLE_ADDRESSES: u32 = 767u32;
pub const ERROR_SHARING_NO_PRIVATE_LAN: u32 = 783u32;
pub const ERROR_SHARING_PRIVATE_INSTALL: u32 = 762u32;
pub const ERROR_SHARING_ROUTER_INSTALL: u32 = 760u32;
pub const ERROR_SHARING_RRAS_CONFLICT: u32 = 782u32;
pub const ERROR_SLIP_REQUIRES_IP: u32 = 729u32;
pub const ERROR_SMART_CARD_REQUIRED: u32 = 779u32;
pub const ERROR_SMM_TIMEOUT: u32 = 748u32;
pub const ERROR_SMM_UNINITIALIZED: u32 = 746u32;
pub const ERROR_SSO_CERT_MISSING: u32 = 874u32;
pub const ERROR_SSTP_COOKIE_SET_FAILURE: u32 = 848u32;
pub const ERROR_STATE_MACHINES_ALREADY_STARTED: u32 = 696u32;
pub const ERROR_STATE_MACHINES_NOT_STARTED: u32 = 695u32;
pub const ERROR_SYSTEM_SUSPENDED: u32 = 833u32;
pub const ERROR_TAPI_CONFIGURATION: u32 = 740u32;
pub const ERROR_TEMPFAILURE: u32 = 774u32;
pub const ERROR_TOO_MANY_LINE_ERRORS: u32 = 715u32;
pub const ERROR_TS_UNACCEPTABLE: u32 = 842u32;
pub const ERROR_UNABLE_TO_AUTHENTICATE_SERVER: u32 = 778u32;
pub const ERROR_UNEXPECTED_RESPONSE: u32 = 702u32;
pub const ERROR_UNKNOWN: u32 = 635u32;
pub const ERROR_UNKNOWN_DEVICE_TYPE: u32 = 663u32;
pub const ERROR_UNKNOWN_FRAMED_PROTOCOL: u32 = 794u32;
pub const ERROR_UNKNOWN_RESPONSE_KEY: u32 = 698u32;
pub const ERROR_UNKNOWN_SERVICE_TYPE: u32 = 796u32;
pub const ERROR_UNRECOGNIZED_RESPONSE: u32 = 652u32;
pub const ERROR_UNSUPPORTED_BPS: u32 = 701u32;
pub const ERROR_UPDATECONNECTION_REQUEST_IN_PROCESS: u32 = 838u32;
pub const ERROR_USER_DISCONNECTION: u32 = 631u32;
pub const ERROR_USER_LOGOFF: u32 = 830u32;
pub const ERROR_VALIDATING_SERVER_CERT: u32 = 801u32;
pub const ERROR_VOICE_ANSWER: u32 = 677u32;
pub const ERROR_VPN_BAD_CERT: u32 = 810u32;
pub const ERROR_VPN_BAD_PSK: u32 = 811u32;
pub const ERROR_VPN_DISCONNECT: u32 = 807u32;
pub const ERROR_VPN_GRE_BLOCKED: u32 = 806u32;
pub const ERROR_VPN_PLUGIN_GENERIC: u32 = 873u32;
pub const ERROR_VPN_REFUSED: u32 = 808u32;
pub const ERROR_VPN_TIMEOUT: u32 = 809u32;
pub const ERROR_WRITING_DEFAULTOFF: u32 = 688u32;
pub const ERROR_WRITING_DEVICENAME: u32 = 684u32;
pub const ERROR_WRITING_DEVICETYPE: u32 = 683u32;
pub const ERROR_WRITING_INITBPS: u32 = 706u32;
pub const ERROR_WRITING_MAXCARRIERBPS: u32 = 686u32;
pub const ERROR_WRITING_MAXCONNECTBPS: u32 = 685u32;
pub const ERROR_WRITING_SECTIONNAME: u32 = 682u32;
pub const ERROR_WRITING_USAGE: u32 = 687u32;
pub const ERROR_WRONG_DEVICE_ATTACHED: u32 = 636u32;
pub const ERROR_WRONG_INFO_SPECIFIED: u32 = 604u32;
pub const ERROR_WRONG_KEY_SPECIFIED: u32 = 662u32;
pub const ERROR_WRONG_MODULE: u32 = 750u32;
pub const ERROR_WRONG_TUNNEL_TYPE: u32 = 795u32;
pub const ERROR_X25_DIAGNOSTIC: u32 = 707u32;
pub const ET_None: u32 = 0u32;
pub const ET_Optional: u32 = 3u32;
pub const ET_Require: u32 = 1u32;
pub const ET_RequireMax: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GRE_CONFIG_PARAMS0 {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HRASCONN(pub *mut core::ffi::c_void);
impl HRASCONN {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HRASCONN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IKEV2_CONFIG_PARAMS {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
    pub dwTunnelConfigParamFlags: u32,
    pub TunnelConfigParams: IKEV2_TUNNEL_CONFIG_PARAMS4,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEV2_ID_PAYLOAD_TYPE(pub i32);
pub const IKEV2_ID_PAYLOAD_TYPE_DER_ASN1_DN: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(9i32);
pub const IKEV2_ID_PAYLOAD_TYPE_DER_ASN1_GN: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(10i32);
pub const IKEV2_ID_PAYLOAD_TYPE_FQDN: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(2i32);
pub const IKEV2_ID_PAYLOAD_TYPE_ID_IPV6_ADDR: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(5i32);
pub const IKEV2_ID_PAYLOAD_TYPE_INVALID: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(0i32);
pub const IKEV2_ID_PAYLOAD_TYPE_IPV4_ADDR: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(1i32);
pub const IKEV2_ID_PAYLOAD_TYPE_KEY_ID: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(11i32);
pub const IKEV2_ID_PAYLOAD_TYPE_MAX: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(12i32);
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED1: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(4i32);
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED2: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(6i32);
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED3: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(7i32);
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED4: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(8i32);
pub const IKEV2_ID_PAYLOAD_TYPE_RFC822_ADDR: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IKEV2_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub IPv4SubInterfaceIndex: u64,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
    pub IPv6SubInterfaceIndex: u64,
    pub dwOptions: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwEapTypeId: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwEncryptionMethod: u32,
}
impl Default for IKEV2_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IKEV2_PROJECTION_INFO2 {
    pub dwIPv4NegotiationError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub IPv4SubInterfaceIndex: u64,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
    pub IPv6SubInterfaceIndex: u64,
    pub dwOptions: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwEapTypeId: u32,
    pub dwEmbeddedEAPTypeId: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwEncryptionMethod: u32,
}
impl Default for IKEV2_PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IKEV2_TUNNEL_CONFIG_PARAMS2 {
    pub dwIdleTimeout: u32,
    pub dwNetworkBlackoutTime: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub dwConfigOptions: u32,
    pub dwTotalCertificates: u32,
    pub certificateNames: *mut super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub machineCertificateName: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub dwEncryptionType: u32,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for IKEV2_TUNNEL_CONFIG_PARAMS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IKEV2_TUNNEL_CONFIG_PARAMS3 {
    pub dwIdleTimeout: u32,
    pub dwNetworkBlackoutTime: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub dwConfigOptions: u32,
    pub dwTotalCertificates: u32,
    pub certificateNames: *mut super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub machineCertificateName: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub dwEncryptionType: u32,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub dwTotalEkus: u32,
    pub certificateEKUs: *mut MPR_CERT_EKU,
    pub machineCertificateHash: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for IKEV2_TUNNEL_CONFIG_PARAMS3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IKEV2_TUNNEL_CONFIG_PARAMS4 {
    pub dwIdleTimeout: u32,
    pub dwNetworkBlackoutTime: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub dwConfigOptions: u32,
    pub dwTotalCertificates: u32,
    pub certificateNames: *mut super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub machineCertificateName: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub dwEncryptionType: u32,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub dwTotalEkus: u32,
    pub certificateEKUs: *mut MPR_CERT_EKU,
    pub machineCertificateHash: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub dwMmSaLifeTime: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for IKEV2_TUNNEL_CONFIG_PARAMS4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IPADDRESSLEN: u32 = 15u32;
pub const IPV6_ADDRESS_LEN_IN_BYTES: u32 = 16u32;
pub const IPXADDRESSLEN: u32 = 22u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct L2TP_CONFIG_PARAMS0 {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct L2TP_CONFIG_PARAMS1 {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
    pub dwTunnelConfigParamFlags: u32,
    pub TunnelConfigParams: L2TP_TUNNEL_CONFIG_PARAMS2,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct L2TP_TUNNEL_CONFIG_PARAMS1 {
    pub dwIdleTimeout: u32,
    pub dwEncryptionType: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
}
impl Default for L2TP_TUNNEL_CONFIG_PARAMS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct L2TP_TUNNEL_CONFIG_PARAMS2 {
    pub dwIdleTimeout: u32,
    pub dwEncryptionType: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub dwMmSaLifeTime: u32,
}
impl Default for L2TP_TUNNEL_CONFIG_PARAMS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAXIPADRESSLEN: u32 = 64u32;
pub const MAX_SSTP_HASH_SIZE: u32 = 32u32;
pub const METHOD_BGP4_AS_PATH: u32 = 1u32;
pub const METHOD_BGP4_NEXTHOP_ATTR: u32 = 8u32;
pub const METHOD_BGP4_PA_ORIGIN: u32 = 4u32;
pub const METHOD_BGP4_PEER_ID: u32 = 2u32;
pub const METHOD_RIP2_NEIGHBOUR_ADDR: u32 = 1u32;
pub const METHOD_RIP2_OUTBOUND_INTF: u32 = 2u32;
pub const METHOD_RIP2_ROUTE_TAG: u32 = 4u32;
pub const METHOD_RIP2_ROUTE_TIMESTAMP: u32 = 8u32;
pub const METHOD_TYPE_ALL_METHODS: u32 = 4294967295u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MGM_ENUM_TYPES(pub i32);
pub const MGM_FORWARD_STATE_FLAG: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MGM_IF_ENTRY {
    pub dwIfIndex: u32,
    pub dwIfNextHopAddr: u32,
    pub bIGMP: windows_core::BOOL,
    pub bIsEnabled: windows_core::BOOL,
}
pub const MGM_JOIN_STATE_FLAG: u32 = 1u32;
pub const MGM_MFE_STATS_0: u32 = 1u32;
pub const MGM_MFE_STATS_1: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Default)]
pub struct MPRAPI_ADMIN_DLL_CALLBACKS {
    pub revision: u8,
    pub lpfnMprAdminGetIpAddressForUser: PMPRADMINGETIPADDRESSFORUSER,
    pub lpfnMprAdminReleaseIpAddress: PMPRADMINRELEASEIPADRESS,
    pub lpfnMprAdminGetIpv6AddressForUser: PMPRADMINGETIPV6ADDRESSFORUSER,
    pub lpfnMprAdminReleaseIpV6AddressForUser: PMPRADMINRELEASEIPV6ADDRESSFORUSER,
    pub lpfnRasAdminAcceptNewLink: PMPRADMINACCEPTNEWLINK,
    pub lpfnRasAdminLinkHangupNotification: PMPRADMINLINKHANGUPNOTIFICATION,
    pub lpfnRasAdminTerminateDll: PMPRADMINTERMINATEDLL,
    pub lpfnRasAdminAcceptNewConnectionEx: PMPRADMINACCEPTNEWCONNECTIONEX,
    pub lpfnRasAdminAcceptEndpointChangeEx: PMPRADMINACCEPTTUNNELENDPOINTCHANGEEX,
    pub lpfnRasAdminAcceptReauthenticationEx: PMPRADMINACCEPTREAUTHENTICATIONEX,
    pub lpfnRasAdminConnectionHangupNotificationEx: PMPRADMINCONNECTIONHANGUPNOTIFICATIONEX,
    pub lpfnRASValidatePreAuthenticatedConnectionEx: PMPRADMINRASVALIDATEPREAUTHENTICATEDCONNECTIONEX,
}
pub const MPRAPI_ADMIN_DLL_VERSION_1: u32 = 1u32;
pub const MPRAPI_ADMIN_DLL_VERSION_2: u32 = 2u32;
pub const MPRAPI_IF_CUSTOM_CONFIG_FOR_IKEV2: u32 = 1u32;
pub const MPRAPI_IKEV2_AUTH_USING_CERT: u32 = 1u32;
pub const MPRAPI_IKEV2_AUTH_USING_EAP: u32 = 2u32;
pub const MPRAPI_IKEV2_PROJECTION_INFO_TYPE: u32 = 2u32;
pub const MPRAPI_IKEV2_SET_TUNNEL_CONFIG_PARAMS: u32 = 1u32;
pub const MPRAPI_L2TP_SET_TUNNEL_CONFIG_PARAMS: u32 = 1u32;
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_1: u32 = 1u32;
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_2: u32 = 2u32;
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_3: u32 = 3u32;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_1: u32 = 1u32;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_2: u32 = 2u32;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_3: u32 = 3u32;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_4: u32 = 4u32;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_5: u32 = 5u32;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_1: u32 = 1u32;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_2: u32 = 2u32;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_3: u32 = 3u32;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_4: u32 = 4u32;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_5: u32 = 5u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPRAPI_OBJECT_HEADER {
    pub revision: u8,
    pub r#type: u8,
    pub size: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPRAPI_OBJECT_TYPE(pub i32);
pub const MPRAPI_OBJECT_TYPE_AUTH_VALIDATION_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(4i32);
pub const MPRAPI_OBJECT_TYPE_IF_CUSTOM_CONFIG_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(6i32);
pub const MPRAPI_OBJECT_TYPE_MPR_SERVER_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(2i32);
pub const MPRAPI_OBJECT_TYPE_MPR_SERVER_SET_CONFIG_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(3i32);
pub const MPRAPI_OBJECT_TYPE_RAS_CONNECTION_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(1i32);
pub const MPRAPI_OBJECT_TYPE_UPDATE_CONNECTION_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(5i32);
pub const MPRAPI_PPP_PROJECTION_INFO_TYPE: u32 = 1u32;
pub const MPRAPI_RAS_CONNECTION_OBJECT_REVISION_1: u32 = 1u32;
pub const MPRAPI_RAS_UPDATE_CONNECTION_OBJECT_REVISION_1: u32 = 1u32;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_GRE: u32 = 16u32;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_IKEV2: u32 = 8u32;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_L2TP: u32 = 2u32;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_PPTP: u32 = 1u32;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_SSTP: u32 = 4u32;
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    pub IkeConfigParams: IKEV2_CONFIG_PARAMS,
    pub PptpConfigParams: PPTP_CONFIG_PARAMS,
    pub L2tpConfigParams: L2TP_CONFIG_PARAMS1,
    pub SstpConfigParams: SSTP_CONFIG_PARAMS,
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    pub IkeConfigParams: IKEV2_CONFIG_PARAMS,
    pub PptpConfigParams: PPTP_CONFIG_PARAMS,
    pub L2tpConfigParams: L2TP_CONFIG_PARAMS1,
    pub SstpConfigParams: SSTP_CONFIG_PARAMS,
    pub GREConfigParams: GRE_CONFIG_PARAMS0,
}
pub const MPRDM_DialAll: MPR_INTERFACE_DIAL_MODE = MPR_INTERFACE_DIAL_MODE(1u32);
pub const MPRDM_DialAsNeeded: MPR_INTERFACE_DIAL_MODE = MPR_INTERFACE_DIAL_MODE(2u32);
pub const MPRDM_DialFirst: MPR_INTERFACE_DIAL_MODE = MPR_INTERFACE_DIAL_MODE(0u32);
pub const MPRDT_Atm: windows_core::PCWSTR = windows_core::w!("ATM");
pub const MPRDT_FrameRelay: windows_core::PCWSTR = windows_core::w!("FRAMERELAY");
pub const MPRDT_Generic: windows_core::PCWSTR = windows_core::w!("GENERIC");
pub const MPRDT_Irda: windows_core::PCWSTR = windows_core::w!("IRDA");
pub const MPRDT_Isdn: windows_core::PCWSTR = windows_core::w!("isdn");
pub const MPRDT_Modem: windows_core::PCWSTR = windows_core::w!("modem");
pub const MPRDT_Pad: windows_core::PCWSTR = windows_core::w!("pad");
pub const MPRDT_Parallel: windows_core::PCWSTR = windows_core::w!("PARALLEL");
pub const MPRDT_SW56: windows_core::PCWSTR = windows_core::w!("SW56");
pub const MPRDT_Serial: windows_core::PCWSTR = windows_core::w!("SERIAL");
pub const MPRDT_Sonet: windows_core::PCWSTR = windows_core::w!("SONET");
pub const MPRDT_Vpn: windows_core::PCWSTR = windows_core::w!("vpn");
pub const MPRDT_X25: windows_core::PCWSTR = windows_core::w!("x25");
pub const MPRET_Direct: u32 = 3u32;
pub const MPRET_Phone: u32 = 1u32;
pub const MPRET_Vpn: u32 = 2u32;
pub const MPRIDS_Disabled: u32 = 4294967295u32;
pub const MPRIDS_UseGlobalValue: u32 = 0u32;
pub const MPRIO_DisableLcpExtensions: u32 = 32u32;
pub const MPRIO_IpHeaderCompression: u32 = 8u32;
pub const MPRIO_IpSecPreSharedKey: u32 = 2147483648u32;
pub const MPRIO_NetworkLogon: u32 = 8192u32;
pub const MPRIO_PromoteAlternates: u32 = 32768u32;
pub const MPRIO_RemoteDefaultGateway: u32 = 16u32;
pub const MPRIO_RequireCHAP: u32 = 134217728u32;
pub const MPRIO_RequireDataEncryption: u32 = 4096u32;
pub const MPRIO_RequireEAP: u32 = 131072u32;
pub const MPRIO_RequireEncryptedPw: u32 = 1024u32;
pub const MPRIO_RequireMachineCertificates: u32 = 16777216u32;
pub const MPRIO_RequireMsCHAP: u32 = 268435456u32;
pub const MPRIO_RequireMsCHAP2: u32 = 536870912u32;
pub const MPRIO_RequireMsEncryptedPw: u32 = 2048u32;
pub const MPRIO_RequirePAP: u32 = 262144u32;
pub const MPRIO_RequireSPAP: u32 = 524288u32;
pub const MPRIO_SecureLocalFiles: u32 = 65536u32;
pub const MPRIO_SharedPhoneNumbers: u32 = 8388608u32;
pub const MPRIO_SpecificIpAddr: u32 = 2u32;
pub const MPRIO_SpecificNameServers: u32 = 4u32;
pub const MPRIO_SwCompression: u32 = 512u32;
pub const MPRIO_UsePreSharedKeyForIkev2Initiator: u32 = 33554432u32;
pub const MPRIO_UsePreSharedKeyForIkev2Responder: u32 = 67108864u32;
pub const MPRNP_Ip: u32 = 4u32;
pub const MPRNP_Ipv6: u32 = 8u32;
pub const MPRNP_Ipx: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_CERT_EKU {
    pub dwSize: u32,
    pub IsEKUOID: windows_core::BOOL,
    pub pwszEKU: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_CREDENTIALSEX_0 {
    pub dwSize: u32,
    pub lpbCredentialsInfo: *mut u8,
}
impl Default for MPR_CREDENTIALSEX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_CREDENTIALSEX_1 {
    pub dwSize: u32,
    pub lpbCredentialsInfo: *mut u8,
}
impl Default for MPR_CREDENTIALSEX_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_DEVICE_0 {
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
}
impl Default for MPR_DEVICE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_DEVICE_1 {
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szLocalPhoneNumber: [u16; 129],
    pub szAlternates: windows_core::PWSTR,
}
impl Default for MPR_DEVICE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MPR_ENABLE_RAS_ON_DEVICE: u32 = 1u32;
pub const MPR_ENABLE_ROUTING_ON_DEVICE: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_ET(pub u32);
pub const MPR_ET_None: MPR_ET = MPR_ET(0u32);
pub const MPR_ET_Optional: MPR_ET = MPR_ET(3u32);
pub const MPR_ET_Require: MPR_ET = MPR_ET(1u32);
pub const MPR_ET_RequireMax: MPR_ET = MPR_ET(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_FILTER_0 {
    pub fEnable: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_IFTRANSPORT_0 {
    pub dwTransportId: u32,
    pub hIfTransport: super::super::Foundation::HANDLE,
    pub wszIfTransportName: [u16; 41],
}
impl Default for MPR_IFTRANSPORT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_IF_CUSTOMINFOEX0 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: u32,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG0,
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_IF_CUSTOMINFOEX1 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: u32,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG1,
}
#[repr(C)]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_IF_CUSTOMINFOEX2 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: u32,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG2,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_INTERFACE_0 {
    pub wszInterfaceName: [u16; 257],
    pub hInterface: super::super::Foundation::HANDLE,
    pub fEnabled: windows_core::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: u32,
    pub dwLastError: u32,
}
impl Default for MPR_INTERFACE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_INTERFACE_1 {
    pub wszInterfaceName: [u16; 257],
    pub hInterface: super::super::Foundation::HANDLE,
    pub fEnabled: windows_core::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: u32,
    pub dwLastError: u32,
    pub lpwsDialoutHoursRestriction: windows_core::PWSTR,
}
impl Default for MPR_INTERFACE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_INTERFACE_2 {
    pub wszInterfaceName: [u16; 257],
    pub hInterface: super::super::Foundation::HANDLE,
    pub fEnabled: windows_core::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: u32,
    pub dwLastError: u32,
    pub dwfOptions: u32,
    pub szLocalPhoneNumber: [u16; 129],
    pub szAlternates: windows_core::PWSTR,
    pub ipaddr: u32,
    pub ipaddrDns: u32,
    pub ipaddrDnsAlt: u32,
    pub ipaddrWins: u32,
    pub ipaddrWinsAlt: u32,
    pub dwfNetProtocols: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szX25PadType: [u16; 33],
    pub szX25Address: [u16; 201],
    pub szX25Facilities: [u16; 201],
    pub szX25UserData: [u16; 201],
    pub dwChannels: u32,
    pub dwSubEntries: u32,
    pub dwDialMode: MPR_INTERFACE_DIAL_MODE,
    pub dwDialExtraPercent: u32,
    pub dwDialExtraSampleSeconds: u32,
    pub dwHangUpExtraPercent: u32,
    pub dwHangUpExtraSampleSeconds: u32,
    pub dwIdleDisconnectSeconds: u32,
    pub dwType: u32,
    pub dwEncryptionType: MPR_ET,
    pub dwCustomAuthKey: u32,
    pub dwCustomAuthDataSize: u32,
    pub lpbCustomAuthData: *mut u8,
    pub guidId: windows_core::GUID,
    pub dwVpnStrategy: MPR_VS,
}
impl Default for MPR_INTERFACE_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct MPR_INTERFACE_3 {
    pub wszInterfaceName: [u16; 257],
    pub hInterface: super::super::Foundation::HANDLE,
    pub fEnabled: windows_core::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: u32,
    pub dwLastError: u32,
    pub dwfOptions: u32,
    pub szLocalPhoneNumber: [u16; 129],
    pub szAlternates: windows_core::PWSTR,
    pub ipaddr: u32,
    pub ipaddrDns: u32,
    pub ipaddrDnsAlt: u32,
    pub ipaddrWins: u32,
    pub ipaddrWinsAlt: u32,
    pub dwfNetProtocols: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szX25PadType: [u16; 33],
    pub szX25Address: [u16; 201],
    pub szX25Facilities: [u16; 201],
    pub szX25UserData: [u16; 201],
    pub dwChannels: u32,
    pub dwSubEntries: u32,
    pub dwDialMode: MPR_INTERFACE_DIAL_MODE,
    pub dwDialExtraPercent: u32,
    pub dwDialExtraSampleSeconds: u32,
    pub dwHangUpExtraPercent: u32,
    pub dwHangUpExtraSampleSeconds: u32,
    pub dwIdleDisconnectSeconds: u32,
    pub dwType: u32,
    pub dwEncryptionType: MPR_ET,
    pub dwCustomAuthKey: u32,
    pub dwCustomAuthDataSize: u32,
    pub lpbCustomAuthData: *mut u8,
    pub guidId: windows_core::GUID,
    pub dwVpnStrategy: MPR_VS,
    pub AddressCount: u32,
    pub ipv6addrDns: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6addrDnsAlt: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6addr: *mut super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for MPR_INTERFACE_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MPR_INTERFACE_ADMIN_DISABLED: u32 = 2u32;
pub const MPR_INTERFACE_CONNECTION_FAILURE: u32 = 4u32;
pub const MPR_INTERFACE_DIALOUT_HOURS_RESTRICTION: u32 = 16u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_INTERFACE_DIAL_MODE(pub u32);
pub const MPR_INTERFACE_NO_DEVICE: u32 = 64u32;
pub const MPR_INTERFACE_NO_MEDIA_SENSE: u32 = 32u32;
pub const MPR_INTERFACE_OUT_OF_RESOURCES: u32 = 1u32;
pub const MPR_INTERFACE_SERVICE_PAUSED: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_IPINIP_INTERFACE_0 {
    pub wszFriendlyName: [u16; 257],
    pub Guid: windows_core::GUID,
}
impl Default for MPR_IPINIP_INTERFACE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MPR_MaxAreaCode: u32 = 10u32;
pub const MPR_MaxCallbackNumber: u32 = 128u32;
pub const MPR_MaxDeviceName: u32 = 128u32;
pub const MPR_MaxDeviceType: u32 = 16u32;
pub const MPR_MaxEntryName: u32 = 256u32;
pub const MPR_MaxFacilities: u32 = 200u32;
pub const MPR_MaxIpAddress: u32 = 15u32;
pub const MPR_MaxIpxAddress: u32 = 21u32;
pub const MPR_MaxPadType: u32 = 32u32;
pub const MPR_MaxPhoneNumber: u32 = 128u32;
pub const MPR_MaxUserData: u32 = 200u32;
pub const MPR_MaxX25Address: u32 = 200u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_SERVER_0 {
    pub fLanOnlyMode: windows_core::BOOL,
    pub dwUpTime: u32,
    pub dwTotalPorts: u32,
    pub dwPortsInUse: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_SERVER_1 {
    pub dwNumPptpPorts: u32,
    pub dwPptpPortFlags: u32,
    pub dwNumL2tpPorts: u32,
    pub dwL2tpPortFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_SERVER_2 {
    pub dwNumPptpPorts: u32,
    pub dwPptpPortFlags: u32,
    pub dwNumL2tpPorts: u32,
    pub dwL2tpPortFlags: u32,
    pub dwNumSstpPorts: u32,
    pub dwSstpPortFlags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_SERVER_EX0 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub fLanOnlyMode: u32,
    pub dwUpTime: u32,
    pub dwTotalPorts: u32,
    pub dwPortsInUse: u32,
    pub Reserved: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS0,
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_SERVER_EX1 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub fLanOnlyMode: u32,
    pub dwUpTime: u32,
    pub dwTotalPorts: u32,
    pub dwPortsInUse: u32,
    pub Reserved: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS1,
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_SERVER_SET_CONFIG_EX0 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub setConfigForProtocols: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS0,
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_SERVER_SET_CONFIG_EX1 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub setConfigForProtocols: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_TRANSPORT_0 {
    pub dwTransportId: u32,
    pub hTransport: super::super::Foundation::HANDLE,
    pub wszTransportName: [u16; 41],
}
impl Default for MPR_TRANSPORT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct MPR_VPN_TRAFFIC_SELECTOR {
    pub r#type: MPR_VPN_TS_TYPE,
    pub protocolId: u8,
    pub portStart: u16,
    pub portEnd: u16,
    pub tsPayloadId: u16,
    pub addrStart: VPN_TS_IP_ADDRESS,
    pub addrEnd: VPN_TS_IP_ADDRESS,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for MPR_VPN_TRAFFIC_SELECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_VPN_TRAFFIC_SELECTORS {
    pub numTsi: u32,
    pub numTsr: u32,
    pub tsI: *mut MPR_VPN_TRAFFIC_SELECTOR,
    pub tsR: *mut MPR_VPN_TRAFFIC_SELECTOR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for MPR_VPN_TRAFFIC_SELECTORS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MPR_VPN_TS_IPv4_ADDR_RANGE: MPR_VPN_TS_TYPE = MPR_VPN_TS_TYPE(7i32);
pub const MPR_VPN_TS_IPv6_ADDR_RANGE: MPR_VPN_TS_TYPE = MPR_VPN_TS_TYPE(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_VPN_TS_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_VS(pub u32);
pub const MPR_VS_Default: MPR_VS = MPR_VS(0u32);
pub const MPR_VS_Ikev2First: u32 = 8u32;
pub const MPR_VS_Ikev2Only: u32 = 7u32;
pub const MPR_VS_L2tpFirst: MPR_VS = MPR_VS(4u32);
pub const MPR_VS_L2tpOnly: MPR_VS = MPR_VS(3u32);
pub const MPR_VS_PptpFirst: MPR_VS = MPR_VS(2u32);
pub const MPR_VS_PptpOnly: MPR_VS = MPR_VS(1u32);
pub type ORASADFUNC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: windows_core::PCSTR, param2: u32, param3: *mut u32) -> windows_core::BOOL>;
pub const PENDING: u32 = 600u32;
pub type PFNRASFREEBUFFER = Option<unsafe extern "system" fn(pbufer: *mut u8) -> u32>;
pub type PFNRASGETBUFFER = Option<unsafe extern "system" fn(ppbuffer: *mut *mut u8, pdwsize: *mut u32) -> u32>;
pub type PFNRASRECEIVEBUFFER = Option<unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, pbuffer: *mut u8, pdwsize: *mut u32, dwtimeout: u32, hevent: super::super::Foundation::HANDLE) -> u32>;
pub type PFNRASRETRIEVEBUFFER = Option<unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, pbuffer: *mut u8, pdwsize: *mut u32) -> u32>;
pub type PFNRASSENDBUFFER = Option<unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, pbuffer: *mut u8, dwsize: u32) -> u32>;
pub type PFNRASSETCOMMSETTINGS = Option<unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, prascommsettings: *mut RASCOMMSETTINGS, pvreserved: *mut core::ffi::c_void) -> u32>;
pub const PID_ATALK: u32 = 41u32;
pub const PID_IP: u32 = 33u32;
pub const PID_IPV6: u32 = 87u32;
pub const PID_IPX: u32 = 43u32;
pub const PID_NBF: u32 = 63u32;
pub type PMGM_CREATION_ALERT_CALLBACK = Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwinifindex: u32, dwinifnexthopaddr: u32, dwifcount: u32, pmieoutiflist: *mut MGM_IF_ENTRY) -> u32>;
pub type PMGM_DISABLE_IGMP_CALLBACK = Option<unsafe extern "system" fn(dwifindex: u32, dwifnexthopaddr: u32) -> u32>;
pub type PMGM_ENABLE_IGMP_CALLBACK = Option<unsafe extern "system" fn(dwifindex: u32, dwifnexthopaddr: u32) -> u32>;
pub type PMGM_JOIN_ALERT_CALLBACK = Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, bmemberupdate: windows_core::BOOL) -> u32>;
pub type PMGM_LOCAL_JOIN_CALLBACK = Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopaddr: u32) -> u32>;
pub type PMGM_LOCAL_LEAVE_CALLBACK = Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopaddr: u32) -> u32>;
pub type PMGM_PRUNE_ALERT_CALLBACK = Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopaddr: u32, bmemberdelete: windows_core::BOOL, pdwtimeout: *mut u32) -> u32>;
pub type PMGM_RPF_CALLBACK = Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, pdwinifindex: *mut u32, pdwinifnexthopaddr: *mut u32, pdwupstreamnbr: *mut u32, dwhdrsize: u32, pbpackethdr: *mut u8, pbroute: *mut u8) -> u32>;
pub type PMGM_WRONG_IF_CALLBACK = Option<unsafe extern "system" fn(dwsourceaddr: u32, dwgroupaddr: u32, dwifindex: u32, dwifnexthopaddr: u32, dwhdrsize: u32, pbpackethdr: *mut u8) -> u32>;
pub type PMPRADMINACCEPTNEWCONNECTION = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1) -> windows_core::BOOL>;
pub type PMPRADMINACCEPTNEWCONNECTION2 = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2) -> windows_core::BOOL>;
pub type PMPRADMINACCEPTNEWCONNECTION3 = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: *mut RAS_CONNECTION_3) -> windows_core::BOOL>;
pub type PMPRADMINACCEPTNEWCONNECTIONEX = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> windows_core::BOOL>;
pub type PMPRADMINACCEPTNEWLINK = Option<unsafe extern "system" fn(param0: *mut RAS_PORT_0, param1: *mut RAS_PORT_1) -> windows_core::BOOL>;
pub type PMPRADMINACCEPTREAUTHENTICATION = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: *mut RAS_CONNECTION_3) -> windows_core::BOOL>;
pub type PMPRADMINACCEPTREAUTHENTICATIONEX = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> windows_core::BOOL>;
pub type PMPRADMINACCEPTTUNNELENDPOINTCHANGEEX = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> windows_core::BOOL>;
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1)>;
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION2 = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2)>;
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION3 = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: *mut RAS_CONNECTION_3)>;
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATIONEX = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX)>;
pub type PMPRADMINGETIPADDRESSFORUSER = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: *mut u32, param3: *mut windows_core::BOOL) -> u32>;
#[cfg(feature = "Win32_Networking_WinSock")]
pub type PMPRADMINGETIPV6ADDRESSFORUSER = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: *mut super::super::Networking::WinSock::IN6_ADDR, param3: *mut windows_core::BOOL) -> u32>;
pub type PMPRADMINLINKHANGUPNOTIFICATION = Option<unsafe extern "system" fn(param0: *mut RAS_PORT_0, param1: *mut RAS_PORT_1)>;
pub type PMPRADMINRASVALIDATEPREAUTHENTICATEDCONNECTIONEX = Option<unsafe extern "system" fn(param0: *mut AUTH_VALIDATION_EX) -> u32>;
pub type PMPRADMINRELEASEIPADRESS = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: *mut u32)>;
#[cfg(feature = "Win32_Networking_WinSock")]
pub type PMPRADMINRELEASEIPV6ADDRESSFORUSER = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: *mut super::super::Networking::WinSock::IN6_ADDR)>;
pub type PMPRADMINTERMINATEDLL = Option<unsafe extern "system" fn() -> u32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_ATCP_INFO {
    pub dwError: u32,
    pub wszAddress: [u16; 33],
}
impl Default for PPP_ATCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PPP_CCP_COMPRESSION: u32 = 1u32;
pub const PPP_CCP_ENCRYPTION128BIT: u32 = 64u32;
pub const PPP_CCP_ENCRYPTION40BIT: u32 = 32u32;
pub const PPP_CCP_ENCRYPTION40BITOLD: u32 = 16u32;
pub const PPP_CCP_ENCRYPTION56BIT: u32 = 128u32;
pub const PPP_CCP_HISTORYLESS: u32 = 16777216u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPP_CCP_INFO {
    pub dwError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwOptions: u32,
    pub dwRemoteCompressionAlgorithm: u32,
    pub dwRemoteOptions: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPP_INFO {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO,
    pub ipx: PPP_IPXCP_INFO,
    pub at: PPP_ATCP_INFO,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPP_INFO_2 {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO2,
    pub ipx: PPP_IPXCP_INFO,
    pub at: PPP_ATCP_INFO,
    pub ccp: PPP_CCP_INFO,
    pub lcp: PPP_LCP_INFO,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPP_INFO_3 {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO2,
    pub ipv6: PPP_IPV6_CP_INFO,
    pub ccp: PPP_CCP_INFO,
    pub lcp: PPP_LCP_INFO,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_IPCP_INFO {
    pub dwError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
}
impl Default for PPP_IPCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_IPCP_INFO2 {
    pub dwError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub dwOptions: u32,
    pub dwRemoteOptions: u32,
}
impl Default for PPP_IPCP_INFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PPP_IPCP_VJ: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_IPV6_CP_INFO {
    pub dwVersion: u32,
    pub dwSize: u32,
    pub dwError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub dwOptions: u32,
    pub dwRemoteOptions: u32,
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
}
impl Default for PPP_IPV6_CP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_IPXCP_INFO {
    pub dwError: u32,
    pub wszAddress: [u16; 23],
}
impl Default for PPP_IPXCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PPP_LCP(pub u32);
pub const PPP_LCP_3_DES: u32 = 32u32;
pub const PPP_LCP_ACFC: u32 = 4u32;
pub const PPP_LCP_AES_128: u32 = 64u32;
pub const PPP_LCP_AES_192: u32 = 256u32;
pub const PPP_LCP_AES_256: u32 = 128u32;
pub const PPP_LCP_CHAP: PPP_LCP = PPP_LCP(49699u32);
pub const PPP_LCP_CHAP_MD5: PPP_LCP_INFO_AUTH_DATA = PPP_LCP_INFO_AUTH_DATA(5u32);
pub const PPP_LCP_CHAP_MS: PPP_LCP_INFO_AUTH_DATA = PPP_LCP_INFO_AUTH_DATA(128u32);
pub const PPP_LCP_CHAP_MSV2: PPP_LCP_INFO_AUTH_DATA = PPP_LCP_INFO_AUTH_DATA(129u32);
pub const PPP_LCP_DES_56: u32 = 16u32;
pub const PPP_LCP_EAP: PPP_LCP = PPP_LCP(49703u32);
pub const PPP_LCP_GCM_AES_128: u32 = 512u32;
pub const PPP_LCP_GCM_AES_192: u32 = 1024u32;
pub const PPP_LCP_GCM_AES_256: u32 = 2048u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPP_LCP_INFO {
    pub dwError: u32,
    pub dwAuthenticationProtocol: PPP_LCP,
    pub dwAuthenticationData: PPP_LCP_INFO_AUTH_DATA,
    pub dwRemoteAuthenticationProtocol: u32,
    pub dwRemoteAuthenticationData: u32,
    pub dwTerminateReason: u32,
    pub dwRemoteTerminateReason: u32,
    pub dwOptions: u32,
    pub dwRemoteOptions: u32,
    pub dwEapTypeId: u32,
    pub dwRemoteEapTypeId: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PPP_LCP_INFO_AUTH_DATA(pub u32);
pub const PPP_LCP_MULTILINK_FRAMING: u32 = 1u32;
pub const PPP_LCP_PAP: PPP_LCP = PPP_LCP(49187u32);
pub const PPP_LCP_PFC: u32 = 2u32;
pub const PPP_LCP_SPAP: PPP_LCP = PPP_LCP(49191u32);
pub const PPP_LCP_SSHF: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_NBFCP_INFO {
    pub dwError: u32,
    pub wszWksta: [u16; 17],
}
impl Default for PPP_NBFCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub dwIPv4Options: u32,
    pub dwIPv4RemoteOptions: u32,
    pub IPv4SubInterfaceIndex: u64,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
    pub IPv6SubInterfaceIndex: u64,
    pub dwLcpError: u32,
    pub dwAuthenticationProtocol: PPP_LCP,
    pub dwAuthenticationData: PPP_LCP_INFO_AUTH_DATA,
    pub dwRemoteAuthenticationProtocol: PPP_LCP,
    pub dwRemoteAuthenticationData: PPP_LCP_INFO_AUTH_DATA,
    pub dwLcpTerminateReason: u32,
    pub dwLcpRemoteTerminateReason: u32,
    pub dwLcpOptions: u32,
    pub dwLcpRemoteOptions: u32,
    pub dwEapTypeId: u32,
    pub dwRemoteEapTypeId: u32,
    pub dwCcpError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwCcpOptions: u32,
    pub dwRemoteCompressionAlgorithm: u32,
    pub dwCcpRemoteOptions: u32,
}
impl Default for PPP_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_PROJECTION_INFO2 {
    pub dwIPv4NegotiationError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub dwIPv4Options: u32,
    pub dwIPv4RemoteOptions: u32,
    pub IPv4SubInterfaceIndex: u64,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
    pub IPv6SubInterfaceIndex: u64,
    pub dwLcpError: u32,
    pub dwAuthenticationProtocol: PPP_LCP,
    pub dwAuthenticationData: PPP_LCP_INFO_AUTH_DATA,
    pub dwRemoteAuthenticationProtocol: PPP_LCP,
    pub dwRemoteAuthenticationData: PPP_LCP_INFO_AUTH_DATA,
    pub dwLcpTerminateReason: u32,
    pub dwLcpRemoteTerminateReason: u32,
    pub dwLcpOptions: u32,
    pub dwLcpRemoteOptions: u32,
    pub dwEapTypeId: u32,
    pub dwEmbeddedEAPTypeId: u32,
    pub dwRemoteEapTypeId: u32,
    pub dwCcpError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwCcpOptions: u32,
    pub dwRemoteCompressionAlgorithm: u32,
    pub dwCcpRemoteOptions: u32,
}
impl Default for PPP_PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPTP_CONFIG_PARAMS {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROJECTION_INFO {
    pub projectionInfoType: u8,
    pub Anonymous: PROJECTION_INFO_0,
}
impl Default for PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROJECTION_INFO_0 {
    pub PppProjectionInfo: PPP_PROJECTION_INFO,
    pub Ikev2ProjectionInfo: IKEV2_PROJECTION_INFO,
}
impl Default for PROJECTION_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROJECTION_INFO2 {
    pub projectionInfoType: u8,
    pub Anonymous: PROJECTION_INFO2_0,
}
impl Default for PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROJECTION_INFO2_0 {
    pub PppProjectionInfo: PPP_PROJECTION_INFO2,
    pub Ikev2ProjectionInfo: IKEV2_PROJECTION_INFO2,
}
impl Default for PROJECTION_INFO2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROJECTION_INFO_TYPE_IKEv2: RASPROJECTION_INFO_TYPE = RASPROJECTION_INFO_TYPE(2i32);
pub const PROJECTION_INFO_TYPE_PPP: RASPROJECTION_INFO_TYPE = RASPROJECTION_INFO_TYPE(1i32);
pub const RASADFLG_PositionDlg: u32 = 1u32;
pub type RASADFUNCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: windows_core::PCSTR, param2: *mut RASADPARAMS, param3: *mut u32) -> windows_core::BOOL>;
pub type RASADFUNCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: *mut RASADPARAMS, param3: *mut u32) -> windows_core::BOOL>;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct RASADPARAMS {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
}
pub const RASADP_ConnectionQueryTimeout: u32 = 4u32;
pub const RASADP_DisableConnectionQuery: u32 = 0u32;
pub const RASADP_FailedConnectionTimeout: u32 = 3u32;
pub const RASADP_LoginSessionDisable: u32 = 1u32;
pub const RASADP_SavedAddressesLimit: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASAMBA {
    pub dwSize: u32,
    pub dwError: u32,
    pub szNetBiosError: [i8; 17],
    pub bLana: u8,
}
impl Default for RASAMBA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASAMBW {
    pub dwSize: u32,
    pub dwError: u32,
    pub szNetBiosError: [u16; 17],
    pub bLana: u8,
}
impl Default for RASAMBW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RASAPIVERSION(pub i32);
pub const RASAPIVERSION_500: RASAPIVERSION = RASAPIVERSION(1i32);
pub const RASAPIVERSION_501: RASAPIVERSION = RASAPIVERSION(2i32);
pub const RASAPIVERSION_600: RASAPIVERSION = RASAPIVERSION(3i32);
pub const RASAPIVERSION_601: RASAPIVERSION = RASAPIVERSION(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASAUTODIALENTRYA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDialingLocation: u32,
    pub szEntry: [i8; 257],
}
impl Default for RASAUTODIALENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASAUTODIALENTRYW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDialingLocation: u32,
    pub szEntry: [u16; 257],
}
impl Default for RASAUTODIALENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RASBASE: u32 = 600u32;
pub const RASBASEEND: u32 = 877u32;
pub const RASCCPCA_MPPC: u32 = 6u32;
pub const RASCCPCA_STAC: u32 = 5u32;
pub const RASCCPO_Compression: u32 = 1u32;
pub const RASCCPO_Encryption128bit: u32 = 64u32;
pub const RASCCPO_Encryption40bit: u32 = 32u32;
pub const RASCCPO_Encryption56bit: u32 = 16u32;
pub const RASCCPO_HistoryLess: u32 = 2u32;
pub const RASCF_AllUsers: u32 = 1u32;
pub const RASCF_GlobalCreds: u32 = 2u32;
pub const RASCF_OwnerKnown: u32 = 4u32;
pub const RASCF_OwnerMatch: u32 = 8u32;
pub const RASCM_DDMPreSharedKey: u32 = 64u32;
pub const RASCM_DefaultCreds: u32 = 8u32;
pub const RASCM_Domain: u32 = 4u32;
pub const RASCM_Password: u32 = 2u32;
pub const RASCM_PreSharedKey: u32 = 16u32;
pub const RASCM_ServerPreSharedKey: u32 = 32u32;
pub const RASCM_UserName: u32 = 1u32;
pub const RASCN_BandwidthAdded: u32 = 4u32;
pub const RASCN_BandwidthRemoved: u32 = 8u32;
pub const RASCN_Connection: u32 = 1u32;
pub const RASCN_Disconnection: u32 = 2u32;
pub const RASCN_Dormant: u32 = 16u32;
pub const RASCN_EPDGPacketArrival: u32 = 64u32;
pub const RASCN_ReConnection: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RASCOMMSETTINGS {
    pub dwSize: u32,
    pub bParity: u8,
    pub bStop: u8,
    pub bByteSize: u8,
    pub bAlign: u8,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASCONNA {
    pub dwSize: u32,
    pub hrasconn: HRASCONN,
    pub szEntryName: [i8; 257],
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
    pub szPhonebook: [i8; 260],
    pub dwSubEntry: u32,
    pub guidEntry: windows_core::GUID,
    pub dwFlags: u32,
    pub luid: super::super::Foundation::LUID,
    pub guidCorrelationId: windows_core::GUID,
}
#[cfg(target_arch = "x86")]
impl Default for RASCONNA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RASCONNA {
    pub dwSize: u32,
    pub hrasconn: HRASCONN,
    pub szEntryName: [i8; 257],
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
    pub szPhonebook: [i8; 260],
    pub dwSubEntry: u32,
    pub guidEntry: windows_core::GUID,
    pub dwFlags: u32,
    pub luid: super::super::Foundation::LUID,
    pub guidCorrelationId: windows_core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RASCONNA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RASCONNSTATE(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASCONNSTATUSA {
    pub dwSize: u32,
    pub rasconnstate: RASCONNSTATE,
    pub dwError: u32,
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
    pub szPhoneNumber: [i8; 129],
    pub localEndPoint: RASTUNNELENDPOINT,
    pub remoteEndPoint: RASTUNNELENDPOINT,
    pub rasconnsubstate: RASCONNSUBSTATE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASCONNSTATUSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASCONNSTATUSW {
    pub dwSize: u32,
    pub rasconnstate: RASCONNSTATE,
    pub dwError: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szPhoneNumber: [u16; 129],
    pub localEndPoint: RASTUNNELENDPOINT,
    pub remoteEndPoint: RASTUNNELENDPOINT,
    pub rasconnsubstate: RASCONNSUBSTATE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASCONNSTATUSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RASCONNSUBSTATE(pub i32);
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASCONNW {
    pub dwSize: u32,
    pub hrasconn: HRASCONN,
    pub szEntryName: [u16; 257],
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szPhonebook: [u16; 260],
    pub dwSubEntry: u32,
    pub guidEntry: windows_core::GUID,
    pub dwFlags: u32,
    pub luid: super::super::Foundation::LUID,
    pub guidCorrelationId: windows_core::GUID,
}
#[cfg(target_arch = "x86")]
impl Default for RASCONNW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RASCONNW {
    pub dwSize: u32,
    pub hrasconn: HRASCONN,
    pub szEntryName: [u16; 257],
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szPhonebook: [u16; 260],
    pub dwSubEntry: u32,
    pub guidEntry: windows_core::GUID,
    pub dwFlags: u32,
    pub luid: super::super::Foundation::LUID,
    pub guidCorrelationId: windows_core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RASCONNW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASCREDENTIALSA {
    pub dwSize: u32,
    pub dwMask: u32,
    pub szUserName: [i8; 257],
    pub szPassword: [i8; 257],
    pub szDomain: [i8; 16],
}
impl Default for RASCREDENTIALSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASCREDENTIALSW {
    pub dwSize: u32,
    pub dwMask: u32,
    pub szUserName: [u16; 257],
    pub szPassword: [u16; 257],
    pub szDomain: [u16; 16],
}
impl Default for RASCREDENTIALSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RASCSS_DONE: u32 = 8192u32;
pub const RASCSS_Dormant: RASCONNSUBSTATE = RASCONNSUBSTATE(1i32);
pub const RASCSS_None: RASCONNSUBSTATE = RASCONNSUBSTATE(0i32);
pub const RASCSS_Reconnected: RASCONNSUBSTATE = RASCONNSUBSTATE(8192i32);
pub const RASCSS_Reconnecting: RASCONNSUBSTATE = RASCONNSUBSTATE(2i32);
pub const RASCS_AllDevicesConnected: RASCONNSTATE = RASCONNSTATE(4i32);
pub const RASCS_ApplySettings: RASCONNSTATE = RASCONNSTATE(24i32);
pub const RASCS_AuthAck: RASCONNSTATE = RASCONNSTATE(12i32);
pub const RASCS_AuthCallback: RASCONNSTATE = RASCONNSTATE(8i32);
pub const RASCS_AuthChangePassword: RASCONNSTATE = RASCONNSTATE(9i32);
pub const RASCS_AuthLinkSpeed: RASCONNSTATE = RASCONNSTATE(11i32);
pub const RASCS_AuthNotify: RASCONNSTATE = RASCONNSTATE(6i32);
pub const RASCS_AuthProject: RASCONNSTATE = RASCONNSTATE(10i32);
pub const RASCS_AuthRetry: RASCONNSTATE = RASCONNSTATE(7i32);
pub const RASCS_Authenticate: RASCONNSTATE = RASCONNSTATE(5i32);
pub const RASCS_Authenticated: RASCONNSTATE = RASCONNSTATE(14i32);
pub const RASCS_CallbackComplete: RASCONNSTATE = RASCONNSTATE(20i32);
pub const RASCS_CallbackSetByCaller: RASCONNSTATE = RASCONNSTATE(4098i32);
pub const RASCS_ConnectDevice: RASCONNSTATE = RASCONNSTATE(2i32);
pub const RASCS_Connected: RASCONNSTATE = RASCONNSTATE(8192i32);
pub const RASCS_DONE: u32 = 8192u32;
pub const RASCS_DeviceConnected: RASCONNSTATE = RASCONNSTATE(3i32);
pub const RASCS_Disconnected: RASCONNSTATE = RASCONNSTATE(8193i32);
pub const RASCS_Interactive: RASCONNSTATE = RASCONNSTATE(4096i32);
pub const RASCS_InvokeEapUI: RASCONNSTATE = RASCONNSTATE(4100i32);
pub const RASCS_LogonNetwork: RASCONNSTATE = RASCONNSTATE(21i32);
pub const RASCS_OpenPort: RASCONNSTATE = RASCONNSTATE(0i32);
pub const RASCS_PAUSED: u32 = 4096u32;
pub const RASCS_PasswordExpired: RASCONNSTATE = RASCONNSTATE(4099i32);
pub const RASCS_PortOpened: RASCONNSTATE = RASCONNSTATE(1i32);
pub const RASCS_PrepareForCallback: RASCONNSTATE = RASCONNSTATE(15i32);
pub const RASCS_Projected: RASCONNSTATE = RASCONNSTATE(18i32);
pub const RASCS_ReAuthenticate: RASCONNSTATE = RASCONNSTATE(13i32);
pub const RASCS_RetryAuthentication: RASCONNSTATE = RASCONNSTATE(4097i32);
pub const RASCS_StartAuthentication: RASCONNSTATE = RASCONNSTATE(19i32);
pub const RASCS_SubEntryConnected: RASCONNSTATE = RASCONNSTATE(22i32);
pub const RASCS_SubEntryDisconnected: RASCONNSTATE = RASCONNSTATE(23i32);
pub const RASCS_WaitForCallback: RASCONNSTATE = RASCONNSTATE(17i32);
pub const RASCS_WaitForModemReset: RASCONNSTATE = RASCONNSTATE(16i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RASCTRYINFO {
    pub dwSize: u32,
    pub dwCountryID: u32,
    pub dwNextCountryID: u32,
    pub dwCountryCode: u32,
    pub dwCountryNameOffset: u32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct RASCUSTOMSCRIPTEXTENSIONS {
    pub dwSize: u32,
    pub pfnRasSetCommSettings: PFNRASSETCOMMSETTINGS,
}
pub const RASDDFLAG_AoacRedial: u32 = 4u32;
pub const RASDDFLAG_LinkFailure: u32 = 2147483648u32;
pub const RASDDFLAG_NoPrompt: u32 = 2u32;
pub const RASDDFLAG_PositionDlg: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASDEVINFOA {
    pub dwSize: u32,
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
}
impl Default for RASDEVINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASDEVINFOW {
    pub dwSize: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
}
impl Default for RASDEVINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASDEVSPECIFICINFO {
    pub dwSize: u32,
    pub pbDevSpecificInfo: *mut u8,
}
#[cfg(target_arch = "x86")]
impl Default for RASDEVSPECIFICINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RASDEVSPECIFICINFO {
    pub dwSize: u32,
    pub pbDevSpecificInfo: *mut u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RASDEVSPECIFICINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct RASDIALDLG {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwSubEntry: u32,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
pub const RASDIALEVENT: windows_core::PCSTR = windows_core::s!("RasDialEvent");
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct RASDIALEXTENSIONS {
    pub dwSize: u32,
    pub dwfOptions: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub reserved: usize,
    pub reserved1: usize,
    pub RasEapInfo: RASEAPINFO,
    pub fSkipPppAuth: windows_core::BOOL,
    pub RasDevSpecificInfo: RASDEVSPECIFICINFO,
}
pub type RASDIALFUNC = Option<unsafe extern "system" fn(param0: u32, param1: RASCONNSTATE, param2: u32)>;
pub type RASDIALFUNC1 = Option<unsafe extern "system" fn(param0: HRASCONN, param1: u32, param2: RASCONNSTATE, param3: u32, param4: u32)>;
pub type RASDIALFUNC2 = Option<unsafe extern "system" fn(param0: usize, param1: u32, param2: HRASCONN, param3: u32, param4: RASCONNSTATE, param5: u32, param6: u32) -> u32>;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASDIALPARAMSA {
    pub dwSize: u32,
    pub szEntryName: [i8; 257],
    pub szPhoneNumber: [i8; 129],
    pub szCallbackNumber: [i8; 129],
    pub szUserName: [i8; 257],
    pub szPassword: [i8; 257],
    pub szDomain: [i8; 16],
    pub dwSubEntry: u32,
    pub dwCallbackId: usize,
    pub dwIfIndex: u32,
    pub szEncPassword: windows_core::PSTR,
}
#[cfg(target_arch = "x86")]
impl Default for RASDIALPARAMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RASDIALPARAMSA {
    pub dwSize: u32,
    pub szEntryName: [i8; 257],
    pub szPhoneNumber: [i8; 129],
    pub szCallbackNumber: [i8; 129],
    pub szUserName: [i8; 257],
    pub szPassword: [i8; 257],
    pub szDomain: [i8; 16],
    pub dwSubEntry: u32,
    pub dwCallbackId: usize,
    pub dwIfIndex: u32,
    pub szEncPassword: windows_core::PSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RASDIALPARAMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASDIALPARAMSW {
    pub dwSize: u32,
    pub szEntryName: [u16; 257],
    pub szPhoneNumber: [u16; 129],
    pub szCallbackNumber: [u16; 129],
    pub szUserName: [u16; 257],
    pub szPassword: [u16; 257],
    pub szDomain: [u16; 16],
    pub dwSubEntry: u32,
    pub dwCallbackId: usize,
    pub dwIfIndex: u32,
    pub szEncPassword: windows_core::PWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for RASDIALPARAMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RASDIALPARAMSW {
    pub dwSize: u32,
    pub szEntryName: [u16; 257],
    pub szPhoneNumber: [u16; 129],
    pub szCallbackNumber: [u16; 129],
    pub szUserName: [u16; 257],
    pub szPassword: [u16; 257],
    pub szDomain: [u16; 16],
    pub dwSubEntry: u32,
    pub dwCallbackId: usize,
    pub dwIfIndex: u32,
    pub szEncPassword: windows_core::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RASDIALPARAMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RASDT_Atm: windows_core::PCWSTR = windows_core::w!("ATM");
pub const RASDT_FrameRelay: windows_core::PCWSTR = windows_core::w!("FRAMERELAY");
pub const RASDT_Generic: windows_core::PCWSTR = windows_core::w!("GENERIC");
pub const RASDT_Irda: windows_core::PCWSTR = windows_core::w!("IRDA");
pub const RASDT_Isdn: windows_core::PCWSTR = windows_core::w!("isdn");
pub const RASDT_Modem: windows_core::PCWSTR = windows_core::w!("modem");
pub const RASDT_PPPoE: windows_core::PCWSTR = windows_core::w!("PPPoE");
pub const RASDT_Pad: windows_core::PCWSTR = windows_core::w!("pad");
pub const RASDT_Parallel: windows_core::PCWSTR = windows_core::w!("PARALLEL");
pub const RASDT_SW56: windows_core::PCWSTR = windows_core::w!("SW56");
pub const RASDT_Serial: windows_core::PCWSTR = windows_core::w!("SERIAL");
pub const RASDT_Sonet: windows_core::PCWSTR = windows_core::w!("SONET");
pub const RASDT_Vpn: windows_core::PCWSTR = windows_core::w!("vpn");
pub const RASDT_X25: windows_core::PCWSTR = windows_core::w!("x25");
pub const RASEAPF_Logon: u32 = 4u32;
pub const RASEAPF_NonInteractive: u32 = 2u32;
pub const RASEAPF_Preview: u32 = 8u32;
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct RASEAPINFO {
    pub dwSizeofEapInfo: u32,
    pub pbEapInfo: *mut u8,
}
impl Default for RASEAPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASEAPUSERIDENTITYA {
    pub szUserName: [i8; 257],
    pub dwSizeofEapInfo: u32,
    pub pbEapInfo: [u8; 1],
}
impl Default for RASEAPUSERIDENTITYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASEAPUSERIDENTITYW {
    pub szUserName: [u16; 257],
    pub dwSizeofEapInfo: u32,
    pub pbEapInfo: [u8; 1],
}
impl Default for RASEAPUSERIDENTITYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RASEDFLAG_CloneEntry: u32 = 4u32;
pub const RASEDFLAG_IncomingConnection: u32 = 1024u32;
pub const RASEDFLAG_InternetEntry: u32 = 256u32;
pub const RASEDFLAG_NAT: u32 = 512u32;
pub const RASEDFLAG_NewBroadbandEntry: u32 = 128u32;
pub const RASEDFLAG_NewDirectEntry: u32 = 64u32;
pub const RASEDFLAG_NewEntry: u32 = 2u32;
pub const RASEDFLAG_NewPhoneEntry: u32 = 16u32;
pub const RASEDFLAG_NewTunnelEntry: u32 = 32u32;
pub const RASEDFLAG_NoRename: u32 = 8u32;
pub const RASEDFLAG_PositionDlg: u32 = 1u32;
pub const RASEDFLAG_ShellOwned: u32 = 1073741824u32;
pub const RASEDM_DialAll: RASENTRY_DIAL_MODE = RASENTRY_DIAL_MODE(1u32);
pub const RASEDM_DialAsNeeded: RASENTRY_DIAL_MODE = RASENTRY_DIAL_MODE(2u32);
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASENTRYA {
    pub dwSize: u32,
    pub dwfOptions: u32,
    pub dwCountryID: u32,
    pub dwCountryCode: u32,
    pub szAreaCode: [i8; 11],
    pub szLocalPhoneNumber: [i8; 129],
    pub dwAlternateOffset: u32,
    pub ipaddr: RASIPADDR,
    pub ipaddrDns: RASIPADDR,
    pub ipaddrDnsAlt: RASIPADDR,
    pub ipaddrWins: RASIPADDR,
    pub ipaddrWinsAlt: RASIPADDR,
    pub dwFrameSize: u32,
    pub dwfNetProtocols: u32,
    pub dwFramingProtocol: u32,
    pub szScript: [i8; 260],
    pub szAutodialDll: [i8; 260],
    pub szAutodialFunc: [i8; 260],
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
    pub szX25PadType: [i8; 33],
    pub szX25Address: [i8; 201],
    pub szX25Facilities: [i8; 201],
    pub szX25UserData: [i8; 201],
    pub dwChannels: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwSubEntries: u32,
    pub dwDialMode: RASENTRY_DIAL_MODE,
    pub dwDialExtraPercent: u32,
    pub dwDialExtraSampleSeconds: u32,
    pub dwHangUpExtraPercent: u32,
    pub dwHangUpExtraSampleSeconds: u32,
    pub dwIdleDisconnectSeconds: u32,
    pub dwType: u32,
    pub dwEncryptionType: u32,
    pub dwCustomAuthKey: u32,
    pub guidId: windows_core::GUID,
    pub szCustomDialDll: [i8; 260],
    pub dwVpnStrategy: u32,
    pub dwfOptions2: u32,
    pub dwfOptions3: u32,
    pub szDnsSuffix: [i8; 256],
    pub dwTcpWindowSize: u32,
    pub szPrerequisitePbk: [i8; 260],
    pub szPrerequisiteEntry: [i8; 257],
    pub dwRedialCount: u32,
    pub dwRedialPause: u32,
    pub ipv6addrDns: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6addrDnsAlt: super::super::Networking::WinSock::IN6_ADDR,
    pub dwIPv4InterfaceMetric: u32,
    pub dwIPv6InterfaceMetric: u32,
    pub ipv6addr: super::super::Networking::WinSock::IN6_ADDR,
    pub dwIPv6PrefixLength: u32,
    pub dwNetworkOutageTime: u32,
    pub szIDi: [i8; 257],
    pub szIDr: [i8; 257],
    pub fIsImsConfig: windows_core::BOOL,
    pub IdiType: IKEV2_ID_PAYLOAD_TYPE,
    pub IdrType: IKEV2_ID_PAYLOAD_TYPE,
    pub fDisableIKEv2Fragmentation: windows_core::BOOL,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASENTRYDLGA {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub szEntry: [i8; 257],
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(target_arch = "x86")]
impl Default for RASENTRYDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RASENTRYDLGA {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub szEntry: [i8; 257],
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RASENTRYDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASENTRYDLGW {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub szEntry: [u16; 257],
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(target_arch = "x86")]
impl Default for RASENTRYDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RASENTRYDLGW {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub szEntry: [u16; 257],
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RASENTRYDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASENTRYNAMEA {
    pub dwSize: u32,
    pub szEntryName: [i8; 257],
    pub dwFlags: u32,
    pub szPhonebookPath: [i8; 261],
}
impl Default for RASENTRYNAMEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASENTRYNAMEW {
    pub dwSize: u32,
    pub szEntryName: [u16; 257],
    pub dwFlags: u32,
    pub szPhonebookPath: [u16; 261],
}
impl Default for RASENTRYNAMEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASENTRYW {
    pub dwSize: u32,
    pub dwfOptions: u32,
    pub dwCountryID: u32,
    pub dwCountryCode: u32,
    pub szAreaCode: [u16; 11],
    pub szLocalPhoneNumber: [u16; 129],
    pub dwAlternateOffset: u32,
    pub ipaddr: RASIPADDR,
    pub ipaddrDns: RASIPADDR,
    pub ipaddrDnsAlt: RASIPADDR,
    pub ipaddrWins: RASIPADDR,
    pub ipaddrWinsAlt: RASIPADDR,
    pub dwFrameSize: u32,
    pub dwfNetProtocols: u32,
    pub dwFramingProtocol: u32,
    pub szScript: [u16; 260],
    pub szAutodialDll: [u16; 260],
    pub szAutodialFunc: [u16; 260],
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szX25PadType: [u16; 33],
    pub szX25Address: [u16; 201],
    pub szX25Facilities: [u16; 201],
    pub szX25UserData: [u16; 201],
    pub dwChannels: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwSubEntries: u32,
    pub dwDialMode: RASENTRY_DIAL_MODE,
    pub dwDialExtraPercent: u32,
    pub dwDialExtraSampleSeconds: u32,
    pub dwHangUpExtraPercent: u32,
    pub dwHangUpExtraSampleSeconds: u32,
    pub dwIdleDisconnectSeconds: u32,
    pub dwType: u32,
    pub dwEncryptionType: u32,
    pub dwCustomAuthKey: u32,
    pub guidId: windows_core::GUID,
    pub szCustomDialDll: [u16; 260],
    pub dwVpnStrategy: u32,
    pub dwfOptions2: u32,
    pub dwfOptions3: u32,
    pub szDnsSuffix: [u16; 256],
    pub dwTcpWindowSize: u32,
    pub szPrerequisitePbk: [u16; 260],
    pub szPrerequisiteEntry: [u16; 257],
    pub dwRedialCount: u32,
    pub dwRedialPause: u32,
    pub ipv6addrDns: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6addrDnsAlt: super::super::Networking::WinSock::IN6_ADDR,
    pub dwIPv4InterfaceMetric: u32,
    pub dwIPv6InterfaceMetric: u32,
    pub ipv6addr: super::super::Networking::WinSock::IN6_ADDR,
    pub dwIPv6PrefixLength: u32,
    pub dwNetworkOutageTime: u32,
    pub szIDi: [u16; 257],
    pub szIDr: [u16; 257],
    pub fIsImsConfig: windows_core::BOOL,
    pub IdiType: IKEV2_ID_PAYLOAD_TYPE,
    pub IdrType: IKEV2_ID_PAYLOAD_TYPE,
    pub fDisableIKEv2Fragmentation: windows_core::BOOL,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RASENTRY_DIAL_MODE(pub u32);
pub const RASEO2_AuthTypeIsOtp: u32 = 268435456u32;
pub const RASEO2_AutoTriggerCapable: u32 = 67108864u32;
pub const RASEO2_CacheCredentials: u32 = 33554432u32;
pub const RASEO2_DisableClassBasedStaticRoute: u32 = 524288u32;
pub const RASEO2_DisableIKENameEkuCheck: u32 = 262144u32;
pub const RASEO2_DisableMobility: u32 = 2097152u32;
pub const RASEO2_DisableNbtOverIP: u32 = 64u32;
pub const RASEO2_DontNegotiateMultilink: u32 = 4u32;
pub const RASEO2_DontUseRasCredentials: u32 = 8u32;
pub const RASEO2_IPv4ExplicitMetric: u32 = 65536u32;
pub const RASEO2_IPv6ExplicitMetric: u32 = 131072u32;
pub const RASEO2_IPv6RemoteDefaultGateway: u32 = 8192u32;
pub const RASEO2_IPv6SpecificNameServers: u32 = 4096u32;
pub const RASEO2_Internet: u32 = 32u32;
pub const RASEO2_IsAlwaysOn: u32 = 536870912u32;
pub const RASEO2_IsPrivateNetwork: u32 = 1073741824u32;
pub const RASEO2_IsThirdPartyProfile: u32 = 134217728u32;
pub const RASEO2_PlumbIKEv2TSAsRoutes: u32 = 2147483648u32;
pub const RASEO2_ReconnectIfDropped: u32 = 256u32;
pub const RASEO2_RegisterIpWithDNS: u32 = 16384u32;
pub const RASEO2_RequireMachineCertificates: u32 = 4194304u32;
pub const RASEO2_SecureClientForMSNet: u32 = 2u32;
pub const RASEO2_SecureFileAndPrint: u32 = 1u32;
pub const RASEO2_SecureRoutingCompartment: u32 = 1024u32;
pub const RASEO2_SharePhoneNumbers: u32 = 512u32;
pub const RASEO2_SpecificIPv6Addr: u32 = 1048576u32;
pub const RASEO2_UseDNSSuffixForRegistration: u32 = 32768u32;
pub const RASEO2_UseGlobalDeviceSettings: u32 = 128u32;
pub const RASEO2_UsePreSharedKey: u32 = 16u32;
pub const RASEO2_UsePreSharedKeyForIkev2Initiator: u32 = 8388608u32;
pub const RASEO2_UsePreSharedKeyForIkev2Responder: u32 = 16777216u32;
pub const RASEO2_UseTypicalSettings: u32 = 2048u32;
pub const RASEO_Custom: u32 = 1048576u32;
pub const RASEO_CustomScript: u32 = 2147483648u32;
pub const RASEO_DisableLcpExtensions: u32 = 32u32;
pub const RASEO_IpHeaderCompression: u32 = 8u32;
pub const RASEO_ModemLights: u32 = 256u32;
pub const RASEO_NetworkLogon: u32 = 8192u32;
pub const RASEO_PreviewDomain: u32 = 33554432u32;
pub const RASEO_PreviewPhoneNumber: u32 = 2097152u32;
pub const RASEO_PreviewUserPw: u32 = 16777216u32;
pub const RASEO_PromoteAlternates: u32 = 32768u32;
pub const RASEO_RemoteDefaultGateway: u32 = 16u32;
pub const RASEO_RequireCHAP: u32 = 134217728u32;
pub const RASEO_RequireDataEncryption: u32 = 4096u32;
pub const RASEO_RequireEAP: u32 = 131072u32;
pub const RASEO_RequireEncryptedPw: u32 = 1024u32;
pub const RASEO_RequireMsCHAP: u32 = 268435456u32;
pub const RASEO_RequireMsCHAP2: u32 = 536870912u32;
pub const RASEO_RequireMsEncryptedPw: u32 = 2048u32;
pub const RASEO_RequirePAP: u32 = 262144u32;
pub const RASEO_RequireSPAP: u32 = 524288u32;
pub const RASEO_RequireW95MSCHAP: u32 = 1073741824u32;
pub const RASEO_SecureLocalFiles: u32 = 65536u32;
pub const RASEO_SharedPhoneNumbers: u32 = 8388608u32;
pub const RASEO_ShowDialingProgress: u32 = 67108864u32;
pub const RASEO_SpecificIpAddr: u32 = 2u32;
pub const RASEO_SpecificNameServers: u32 = 4u32;
pub const RASEO_SwCompression: u32 = 512u32;
pub const RASEO_TerminalAfterDial: u32 = 128u32;
pub const RASEO_TerminalBeforeDial: u32 = 64u32;
pub const RASEO_UseCountryAndAreaCodes: u32 = 1u32;
pub const RASEO_UseLogonCredentials: u32 = 16384u32;
pub const RASET_Broadband: u32 = 5u32;
pub const RASET_Direct: u32 = 3u32;
pub const RASET_Internet: u32 = 4u32;
pub const RASET_Phone: u32 = 1u32;
pub const RASET_Vpn: u32 = 2u32;
pub const RASFP_Ppp: u32 = 1u32;
pub const RASFP_Ras: u32 = 4u32;
pub const RASFP_Slip: u32 = 2u32;
pub const RASIDS_Disabled: u32 = 4294967295u32;
pub const RASIDS_UseGlobalValue: u32 = 0u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASIKEV2_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub ipv4Address: super::super::Networking::WinSock::IN_ADDR,
    pub ipv4ServerAddress: super::super::Networking::WinSock::IN_ADDR,
    pub dwIPv6NegotiationError: u32,
    pub ipv6Address: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6ServerAddress: super::super::Networking::WinSock::IN6_ADDR,
    pub dwPrefixLength: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwEapTypeId: u32,
    pub dwFlags: RASIKEV_PROJECTION_INFO_FLAGS,
    pub dwEncryptionMethod: u32,
    pub numIPv4ServerAddresses: u32,
    pub ipv4ServerAddresses: *mut super::super::Networking::WinSock::IN_ADDR,
    pub numIPv6ServerAddresses: u32,
    pub ipv6ServerAddresses: *mut super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASIKEV2_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASIKEV2_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub ipv4Address: super::super::Networking::WinSock::IN_ADDR,
    pub ipv4ServerAddress: super::super::Networking::WinSock::IN_ADDR,
    pub dwIPv6NegotiationError: u32,
    pub ipv6Address: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6ServerAddress: super::super::Networking::WinSock::IN6_ADDR,
    pub dwPrefixLength: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwEapTypeId: u32,
    pub dwFlags: RASIKEV_PROJECTION_INFO_FLAGS,
    pub dwEncryptionMethod: u32,
    pub numIPv4ServerAddresses: u32,
    pub ipv4ServerAddresses: *mut super::super::Networking::WinSock::IN_ADDR,
    pub numIPv6ServerAddresses: u32,
    pub ipv6ServerAddresses: *mut super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASIKEV2_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RASIKEV_PROJECTION_INFO_FLAGS(pub u32);
impl RASIKEV_PROJECTION_INFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RASIKEV_PROJECTION_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RASIKEV_PROJECTION_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RASIKEV_PROJECTION_INFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RASIKEV_PROJECTION_INFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RASIKEV_PROJECTION_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RASIKEv2_AUTH_EAP: u32 = 2u32;
pub const RASIKEv2_AUTH_MACHINECERTIFICATES: u32 = 1u32;
pub const RASIKEv2_AUTH_PSK: u32 = 3u32;
pub const RASIKEv2_FLAGS_BEHIND_NAT: RASIKEV_PROJECTION_INFO_FLAGS = RASIKEV_PROJECTION_INFO_FLAGS(2u32);
pub const RASIKEv2_FLAGS_MOBIKESUPPORTED: RASIKEV_PROJECTION_INFO_FLAGS = RASIKEV_PROJECTION_INFO_FLAGS(1u32);
pub const RASIKEv2_FLAGS_SERVERBEHIND_NAT: RASIKEV_PROJECTION_INFO_FLAGS = RASIKEV_PROJECTION_INFO_FLAGS(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RASIPADDR {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
}
pub const RASIPO_VJ: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASIPXW {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpxAddress: [u16; 22],
}
impl Default for RASIPXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RASLCPAD_CHAP_MD5: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA = RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA(5u32);
pub const RASLCPAD_CHAP_MS: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA = RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA(128u32);
pub const RASLCPAD_CHAP_MSV2: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA = RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA(129u32);
pub const RASLCPAP_CHAP: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL = RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(49699u32);
pub const RASLCPAP_EAP: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL = RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(49703u32);
pub const RASLCPAP_PAP: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL = RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(49187u32);
pub const RASLCPAP_SPAP: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL = RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(49191u32);
pub const RASLCPO_3_DES: u32 = 16u32;
pub const RASLCPO_ACFC: u32 = 2u32;
pub const RASLCPO_AES_128: u32 = 32u32;
pub const RASLCPO_AES_192: u32 = 128u32;
pub const RASLCPO_AES_256: u32 = 64u32;
pub const RASLCPO_DES_56: u32 = 8u32;
pub const RASLCPO_GCM_AES_128: u32 = 256u32;
pub const RASLCPO_GCM_AES_192: u32 = 512u32;
pub const RASLCPO_GCM_AES_256: u32 = 1024u32;
pub const RASLCPO_PFC: u32 = 1u32;
pub const RASLCPO_SSHF: u32 = 4u32;
pub const RASNAP_ProbationTime: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASNOUSERA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwTimeoutMs: u32,
    pub szUserName: [i8; 257],
    pub szPassword: [i8; 257],
    pub szDomain: [i8; 16],
}
impl Default for RASNOUSERA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASNOUSERW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwTimeoutMs: u32,
    pub szUserName: [u16; 257],
    pub szPassword: [u16; 257],
    pub szDomain: [u16; 16],
}
impl Default for RASNOUSERW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RASNOUSER_SmartCard: u32 = 1u32;
pub const RASNP_Ip: u32 = 4u32;
pub const RASNP_Ipv6: u32 = 8u32;
pub const RASNP_Ipx: u32 = 2u32;
pub const RASNP_NetBEUI: u32 = 1u32;
pub const RASPBDEVENT_AddEntry: u32 = 1u32;
pub const RASPBDEVENT_DialEntry: u32 = 4u32;
pub const RASPBDEVENT_EditEntry: u32 = 2u32;
pub const RASPBDEVENT_EditGlobals: u32 = 5u32;
pub const RASPBDEVENT_NoUser: u32 = 6u32;
pub const RASPBDEVENT_NoUserEdit: u32 = 7u32;
pub const RASPBDEVENT_RemoveEntry: u32 = 3u32;
pub const RASPBDFLAG_ForceCloseOnDial: u32 = 2u32;
pub const RASPBDFLAG_NoUser: u32 = 16u32;
pub const RASPBDFLAG_PositionDlg: u32 = 1u32;
pub const RASPBDFLAG_UpdateDefaults: u32 = 2147483648u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default)]
pub struct RASPBDLGA {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwCallbackId: usize,
    pub pCallback: RASPBDLGFUNCA,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct RASPBDLGA {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwCallbackId: usize,
    pub pCallback: RASPBDLGFUNCA,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
pub type RASPBDLGFUNCA = Option<unsafe extern "system" fn(param0: usize, param1: u32, param2: windows_core::PCSTR, param3: *mut core::ffi::c_void)>;
pub type RASPBDLGFUNCW = Option<unsafe extern "system" fn(param0: usize, param1: u32, param2: windows_core::PCWSTR, param3: *mut core::ffi::c_void)>;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default)]
pub struct RASPBDLGW {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwCallbackId: usize,
    pub pCallback: RASPBDLGFUNCW,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct RASPBDLGW {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwCallbackId: usize,
    pub pCallback: RASPBDLGFUNCW,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RASPPPCCP {
    pub dwSize: u32,
    pub dwError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwOptions: u32,
    pub dwServerCompressionAlgorithm: u32,
    pub dwServerOptions: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASPPPIPA {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpAddress: [i8; 16],
    pub szServerIpAddress: [i8; 16],
    pub dwOptions: u32,
    pub dwServerOptions: u32,
}
impl Default for RASPPPIPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASPPPIPV6 {
    pub dwSize: u32,
    pub dwError: u32,
    pub bLocalInterfaceIdentifier: [u8; 8],
    pub bPeerInterfaceIdentifier: [u8; 8],
    pub bLocalCompressionProtocol: [u8; 2],
    pub bPeerCompressionProtocol: [u8; 2],
}
impl Default for RASPPPIPV6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASPPPIPW {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpAddress: [u16; 16],
    pub szServerIpAddress: [u16; 16],
    pub dwOptions: u32,
    pub dwServerOptions: u32,
}
impl Default for RASPPPIPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASPPPIPXA {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpxAddress: [i8; 22],
}
impl Default for RASPPPIPXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASPPPLCPA {
    pub dwSize: u32,
    pub fBundled: windows_core::BOOL,
    pub dwError: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwAuthenticationData: u32,
    pub dwEapTypeId: u32,
    pub dwServerAuthenticationProtocol: u32,
    pub dwServerAuthenticationData: u32,
    pub dwServerEapTypeId: u32,
    pub fMultilink: windows_core::BOOL,
    pub dwTerminateReason: u32,
    pub dwServerTerminateReason: u32,
    pub szReplyMessage: [i8; 1024],
    pub dwOptions: u32,
    pub dwServerOptions: u32,
}
impl Default for RASPPPLCPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASPPPLCPW {
    pub dwSize: u32,
    pub fBundled: windows_core::BOOL,
    pub dwError: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwAuthenticationData: u32,
    pub dwEapTypeId: u32,
    pub dwServerAuthenticationProtocol: u32,
    pub dwServerAuthenticationData: u32,
    pub dwServerEapTypeId: u32,
    pub fMultilink: windows_core::BOOL,
    pub dwTerminateReason: u32,
    pub dwServerTerminateReason: u32,
    pub szReplyMessage: [u16; 1024],
    pub dwOptions: u32,
    pub dwServerOptions: u32,
}
impl Default for RASPPPLCPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASPPPNBFA {
    pub dwSize: u32,
    pub dwError: u32,
    pub dwNetBiosError: u32,
    pub szNetBiosError: [i8; 17],
    pub szWorkstationName: [i8; 17],
    pub bLana: u8,
}
impl Default for RASPPPNBFA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASPPPNBFW {
    pub dwSize: u32,
    pub dwError: u32,
    pub dwNetBiosError: u32,
    pub szNetBiosError: [u16; 17],
    pub szWorkstationName: [u16; 17],
    pub bLana: u8,
}
impl Default for RASPPPNBFW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASPPP_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub ipv4Address: super::super::Networking::WinSock::IN_ADDR,
    pub ipv4ServerAddress: super::super::Networking::WinSock::IN_ADDR,
    pub dwIPv4Options: u32,
    pub dwIPv4ServerOptions: u32,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bServerInterfaceIdentifier: [u8; 8],
    pub fBundled: windows_core::BOOL,
    pub fMultilink: windows_core::BOOL,
    pub dwAuthenticationProtocol: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL,
    pub dwAuthenticationData: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA,
    pub dwServerAuthenticationProtocol: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL,
    pub dwServerAuthenticationData: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA,
    pub dwEapTypeId: u32,
    pub dwServerEapTypeId: u32,
    pub dwLcpOptions: u32,
    pub dwLcpServerOptions: u32,
    pub dwCcpError: u32,
    pub dwCcpCompressionAlgorithm: u32,
    pub dwCcpServerCompressionAlgorithm: u32,
    pub dwCcpOptions: u32,
    pub dwCcpServerOptions: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASPPP_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(pub u32);
pub const RASPRIV2_DialinPolicy: u32 = 1u32;
pub const RASPRIV_AdminSetCallback: u32 = 2u32;
pub const RASPRIV_CallerSetCallback: u32 = 4u32;
pub const RASPRIV_DialinPrivilege: u32 = 8u32;
pub const RASPRIV_NoCallback: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RASPROJECTION(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RASPROJECTION_INFO_TYPE(pub i32);
pub const RASP_Amb: RASPROJECTION = RASPROJECTION(65536i32);
pub const RASP_PppCcp: RASPROJECTION = RASPROJECTION(33021i32);
pub const RASP_PppIp: RASPROJECTION = RASPROJECTION(32801i32);
pub const RASP_PppIpv6: RASPROJECTION = RASPROJECTION(32855i32);
pub const RASP_PppIpx: RASPROJECTION = RASPROJECTION(32811i32);
pub const RASP_PppLcp: RASPROJECTION = RASPROJECTION(49185i32);
pub const RASP_PppNbf: RASPROJECTION = RASPROJECTION(32831i32);
pub type RASSECURITYPROC = Option<unsafe extern "system" fn() -> u32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASSUBENTRYA {
    pub dwSize: u32,
    pub dwfFlags: u32,
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
    pub szLocalPhoneNumber: [i8; 129],
    pub dwAlternateOffset: u32,
}
impl Default for RASSUBENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RASSUBENTRYW {
    pub dwSize: u32,
    pub dwfFlags: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szLocalPhoneNumber: [u16; 129],
    pub dwAlternateOffset: u32,
}
impl Default for RASSUBENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASTUNNELENDPOINT {
    pub dwType: u32,
    pub Anonymous: RASTUNNELENDPOINT_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASTUNNELENDPOINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union RASTUNNELENDPOINT_0 {
    pub ipv4: super::super::Networking::WinSock::IN_ADDR,
    pub ipv6: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASTUNNELENDPOINT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RASTUNNELENDPOINT_IPv4: u32 = 1u32;
pub const RASTUNNELENDPOINT_IPv6: u32 = 2u32;
pub const RASTUNNELENDPOINT_UNKNOWN: u32 = 0u32;
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASUPDATECONN {
    pub version: RASAPIVERSION,
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwIfIndex: u32,
    pub localEndPoint: RASTUNNELENDPOINT,
    pub remoteEndPoint: RASTUNNELENDPOINT,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASUPDATECONN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_CONNECTION_0 {
    pub hConnection: super::super::Foundation::HANDLE,
    pub hInterface: super::super::Foundation::HANDLE,
    pub dwConnectDuration: u32,
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionFlags: RAS_FLAGS,
    pub wszInterfaceName: [u16; 257],
    pub wszUserName: [u16; 257],
    pub wszLogonDomain: [u16; 16],
    pub wszRemoteComputer: [u16; 17],
}
impl Default for RAS_CONNECTION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RAS_CONNECTION_1 {
    pub hConnection: super::super::Foundation::HANDLE,
    pub hInterface: super::super::Foundation::HANDLE,
    pub PppInfo: PPP_INFO,
    pub dwBytesXmited: u32,
    pub dwBytesRcved: u32,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_CONNECTION_2 {
    pub hConnection: super::super::Foundation::HANDLE,
    pub wszUserName: [u16; 257],
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub guid: windows_core::GUID,
    pub PppInfo2: PPP_INFO_2,
}
impl Default for RAS_CONNECTION_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_CONNECTION_3 {
    pub dwVersion: u32,
    pub dwSize: u32,
    pub hConnection: super::super::Foundation::HANDLE,
    pub wszUserName: [u16; 257],
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub guid: windows_core::GUID,
    pub PppInfo3: PPP_INFO_3,
    pub rasQuarState: RAS_QUARANTINE_STATE,
    pub timer: super::super::Foundation::FILETIME,
}
impl Default for RAS_CONNECTION_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RAS_CONNECTION_4 {
    pub dwConnectDuration: u32,
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionFlags: RAS_FLAGS,
    pub wszInterfaceName: [u16; 257],
    pub wszUserName: [u16; 257],
    pub wszLogonDomain: [u16; 16],
    pub wszRemoteComputer: [u16; 17],
    pub guid: windows_core::GUID,
    pub rasQuarState: RAS_QUARANTINE_STATE,
    pub probationTime: super::super::Foundation::FILETIME,
    pub connectionStartTime: super::super::Foundation::FILETIME,
    pub ullBytesXmited: u64,
    pub ullBytesRcved: u64,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
    pub dwNumSwitchOvers: u32,
    pub wszRemoteEndpointAddress: [u16; 65],
    pub wszLocalEndpointAddress: [u16; 65],
    pub ProjectionInfo: PROJECTION_INFO2,
    pub hConnection: super::super::Foundation::HANDLE,
    pub hInterface: super::super::Foundation::HANDLE,
    pub dwDeviceType: u32,
}
impl Default for RAS_CONNECTION_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RAS_CONNECTION_EX {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwConnectDuration: u32,
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionFlags: RAS_FLAGS,
    pub wszInterfaceName: [u16; 257],
    pub wszUserName: [u16; 257],
    pub wszLogonDomain: [u16; 16],
    pub wszRemoteComputer: [u16; 17],
    pub guid: windows_core::GUID,
    pub rasQuarState: RAS_QUARANTINE_STATE,
    pub probationTime: super::super::Foundation::FILETIME,
    pub dwBytesXmited: u32,
    pub dwBytesRcved: u32,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
    pub dwNumSwitchOvers: u32,
    pub wszRemoteEndpointAddress: [u16; 65],
    pub wszLocalEndpointAddress: [u16; 65],
    pub ProjectionInfo: PROJECTION_INFO,
    pub hConnection: super::super::Foundation::HANDLE,
    pub hInterface: super::super::Foundation::HANDLE,
}
impl Default for RAS_CONNECTION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RAS_FLAGS(pub u32);
pub const RAS_FLAGS_ARAP_CONNECTION: RAS_FLAGS = RAS_FLAGS(16u32);
pub const RAS_FLAGS_DORMANT: RAS_FLAGS = RAS_FLAGS(32u32);
pub const RAS_FLAGS_IKEV2_CONNECTION: RAS_FLAGS = RAS_FLAGS(16u32);
pub const RAS_FLAGS_MESSENGER_PRESENT: RAS_FLAGS = RAS_FLAGS(2u32);
pub const RAS_FLAGS_PPP_CONNECTION: RAS_FLAGS = RAS_FLAGS(1u32);
pub const RAS_FLAGS_QUARANTINE_PRESENT: RAS_FLAGS = RAS_FLAGS(8u32);
pub const RAS_FLAGS_RAS_CONNECTION: u32 = 4u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RAS_HARDWARE_CONDITION(pub i32);
pub const RAS_HARDWARE_FAILURE: RAS_HARDWARE_CONDITION = RAS_HARDWARE_CONDITION(1i32);
pub const RAS_HARDWARE_OPERATIONAL: RAS_HARDWARE_CONDITION = RAS_HARDWARE_CONDITION(0i32);
pub const RAS_MaxAreaCode: u32 = 10u32;
pub const RAS_MaxCallbackNumber: u32 = 128u32;
pub const RAS_MaxDeviceName: u32 = 128u32;
pub const RAS_MaxDeviceType: u32 = 16u32;
pub const RAS_MaxDnsSuffix: u32 = 256u32;
pub const RAS_MaxEntryName: u32 = 256u32;
pub const RAS_MaxFacilities: u32 = 200u32;
pub const RAS_MaxIDSize: u32 = 256u32;
pub const RAS_MaxIpAddress: u32 = 15u32;
pub const RAS_MaxIpxAddress: u32 = 21u32;
pub const RAS_MaxPadType: u32 = 32u32;
pub const RAS_MaxPhoneNumber: u32 = 128u32;
pub const RAS_MaxReplyMessage: u32 = 1024u32;
pub const RAS_MaxUserData: u32 = 200u32;
pub const RAS_MaxX25Address: u32 = 200u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_PORT_0 {
    pub hPort: super::super::Foundation::HANDLE,
    pub hConnection: super::super::Foundation::HANDLE,
    pub dwPortCondition: RAS_PORT_CONDITION,
    pub dwTotalNumberOfCalls: u32,
    pub dwConnectDuration: u32,
    pub wszPortName: [u16; 17],
    pub wszMediaName: [u16; 17],
    pub wszDeviceName: [u16; 129],
    pub wszDeviceType: [u16; 17],
}
impl Default for RAS_PORT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RAS_PORT_1 {
    pub hPort: super::super::Foundation::HANDLE,
    pub hConnection: super::super::Foundation::HANDLE,
    pub dwHardwareCondition: RAS_HARDWARE_CONDITION,
    pub dwLineSpeed: u32,
    pub dwBytesXmited: u32,
    pub dwBytesRcved: u32,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_PORT_2 {
    pub hPort: super::super::Foundation::HANDLE,
    pub hConnection: super::super::Foundation::HANDLE,
    pub dwConn_State: u32,
    pub wszPortName: [u16; 17],
    pub wszMediaName: [u16; 17],
    pub wszDeviceName: [u16; 129],
    pub wszDeviceType: [u16; 17],
    pub dwHardwareCondition: RAS_HARDWARE_CONDITION,
    pub dwLineSpeed: u32,
    pub dwCrcErr: u32,
    pub dwSerialOverRunErrs: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
    pub dwTotalErrors: u32,
    pub ullBytesXmited: u64,
    pub ullBytesRcved: u64,
    pub ullFramesXmited: u64,
    pub ullFramesRcved: u64,
    pub ullBytesTxUncompressed: u64,
    pub ullBytesTxCompressed: u64,
    pub ullBytesRcvUncompressed: u64,
    pub ullBytesRcvCompressed: u64,
}
impl Default for RAS_PORT_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RAS_PORT_AUTHENTICATED: RAS_PORT_CONDITION = RAS_PORT_CONDITION(5i32);
pub const RAS_PORT_AUTHENTICATING: RAS_PORT_CONDITION = RAS_PORT_CONDITION(4i32);
pub const RAS_PORT_CALLING_BACK: RAS_PORT_CONDITION = RAS_PORT_CONDITION(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RAS_PORT_CONDITION(pub i32);
pub const RAS_PORT_DISCONNECTED: RAS_PORT_CONDITION = RAS_PORT_CONDITION(1i32);
pub const RAS_PORT_INITIALIZING: RAS_PORT_CONDITION = RAS_PORT_CONDITION(6i32);
pub const RAS_PORT_LISTENING: RAS_PORT_CONDITION = RAS_PORT_CONDITION(3i32);
pub const RAS_PORT_NON_OPERATIONAL: RAS_PORT_CONDITION = RAS_PORT_CONDITION(0i32);
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RAS_PROJECTION_INFO {
    pub version: RASAPIVERSION,
    pub r#type: RASPROJECTION_INFO_TYPE,
    pub Anonymous: RAS_PROJECTION_INFO_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RAS_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union RAS_PROJECTION_INFO_0 {
    pub ppp: RASPPP_PROJECTION_INFO,
    pub ikev2: RASIKEV2_PROJECTION_INFO,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RAS_PROJECTION_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RAS_QUARANTINE_STATE(pub i32);
pub const RAS_QUAR_STATE_NORMAL: RAS_QUARANTINE_STATE = RAS_QUARANTINE_STATE(0i32);
pub const RAS_QUAR_STATE_NOT_CAPABLE: RAS_QUARANTINE_STATE = RAS_QUARANTINE_STATE(3i32);
pub const RAS_QUAR_STATE_PROBATION: RAS_QUARANTINE_STATE = RAS_QUARANTINE_STATE(2i32);
pub const RAS_QUAR_STATE_QUARANTINE: RAS_QUARANTINE_STATE = RAS_QUARANTINE_STATE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_SECURITY_INFO {
    pub LastError: u32,
    pub BytesReceived: u32,
    pub DeviceName: [i8; 129],
}
impl Default for RAS_SECURITY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RAS_STATS {
    pub dwSize: u32,
    pub dwBytesXmited: u32,
    pub dwBytesRcved: u32,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
    pub dwBps: u32,
    pub dwConnectDuration: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_UPDATE_CONNECTION {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwIfIndex: u32,
    pub wszLocalEndpointAddress: [u16; 65],
    pub wszRemoteEndpointAddress: [u16; 65],
}
impl Default for RAS_UPDATE_CONNECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_USER_0 {
    pub bfPrivilege: u8,
    pub wszPhoneNumber: [u16; 129],
}
impl Default for RAS_USER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_USER_1 {
    pub bfPrivilege: u8,
    pub wszPhoneNumber: [u16; 129],
    pub bfPrivilege2: u8,
}
impl Default for RAS_USER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RCD_AllUsers: u32 = 1u32;
pub const RCD_Eap: u32 = 2u32;
pub const RCD_Logon: u32 = 4u32;
pub const RCD_SingleUser: u32 = 0u32;
pub const RDEOPT_CustomDial: u32 = 4096u32;
pub const RDEOPT_DisableConnectedUI: u32 = 64u32;
pub const RDEOPT_DisableReconnect: u32 = 256u32;
pub const RDEOPT_DisableReconnectUI: u32 = 128u32;
pub const RDEOPT_EapInfoCryptInCapable: u32 = 32768u32;
pub const RDEOPT_IgnoreModemSpeaker: u32 = 4u32;
pub const RDEOPT_IgnoreSoftwareCompression: u32 = 16u32;
pub const RDEOPT_InvokeAutoTriggerCredentialUI: u32 = 16384u32;
pub const RDEOPT_NoUser: u32 = 512u32;
pub const RDEOPT_PauseOnScript: u32 = 1024u32;
pub const RDEOPT_PausedStates: u32 = 2u32;
pub const RDEOPT_Router: u32 = 2048u32;
pub const RDEOPT_SetModemSpeaker: u32 = 8u32;
pub const RDEOPT_SetSoftwareCompression: u32 = 32u32;
pub const RDEOPT_UseCustomScripting: u32 = 8192u32;
pub const RDEOPT_UsePrefixSuffix: u32 = 1u32;
pub const REN_AllUsers: u32 = 1u32;
pub const REN_User: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ROUTER_CONNECTION_STATE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ROUTER_CUSTOM_IKEv2_POLICY0 {
    pub dwIntegrityMethod: u32,
    pub dwEncryptionMethod: u32,
    pub dwCipherTransformConstant: u32,
    pub dwAuthTransformConstant: u32,
    pub dwPfsGroup: u32,
    pub dwDhGroup: u32,
}
pub const ROUTER_IF_STATE_CONNECTED: ROUTER_CONNECTION_STATE = ROUTER_CONNECTION_STATE(3i32);
pub const ROUTER_IF_STATE_CONNECTING: ROUTER_CONNECTION_STATE = ROUTER_CONNECTION_STATE(2i32);
pub const ROUTER_IF_STATE_DISCONNECTED: ROUTER_CONNECTION_STATE = ROUTER_CONNECTION_STATE(1i32);
pub const ROUTER_IF_STATE_UNREACHABLE: ROUTER_CONNECTION_STATE = ROUTER_CONNECTION_STATE(0i32);
pub const ROUTER_IF_TYPE_CLIENT: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(0i32);
pub const ROUTER_IF_TYPE_DEDICATED: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(3i32);
pub const ROUTER_IF_TYPE_DIALOUT: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(7i32);
pub const ROUTER_IF_TYPE_FULL_ROUTER: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(2i32);
pub const ROUTER_IF_TYPE_HOME_ROUTER: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(1i32);
pub const ROUTER_IF_TYPE_INTERNAL: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(4i32);
pub const ROUTER_IF_TYPE_LOOPBACK: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(5i32);
pub const ROUTER_IF_TYPE_MAX: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(8i32);
pub const ROUTER_IF_TYPE_TUNNEL1: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(6i32);
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    pub dwSaLifeTime: u32,
    pub dwSaDataSize: u32,
    pub certificateName: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    pub dwSaLifeTime: u32,
    pub dwSaDataSize: u32,
    pub certificateName: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub certificateHash: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    pub dwSaLifeTime: u32,
    pub dwSaDataSize: u32,
    pub certificateName: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub certificateHash: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub dwMmSaLifeTime: u32,
    pub vpnTrafficSelectors: MPR_VPN_TRAFFIC_SELECTORS,
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ROUTER_INTERFACE_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ROUTING_PROTOCOL_CONFIG {
    pub dwCallbackFlags: u32,
    pub pfnRpfCallback: PMGM_RPF_CALLBACK,
    pub pfnCreationAlertCallback: PMGM_CREATION_ALERT_CALLBACK,
    pub pfnPruneAlertCallback: PMGM_PRUNE_ALERT_CALLBACK,
    pub pfnJoinAlertCallback: PMGM_JOIN_ALERT_CALLBACK,
    pub pfnWrongIfCallback: PMGM_WRONG_IF_CALLBACK,
    pub pfnLocalJoinCallback: PMGM_LOCAL_JOIN_CALLBACK,
    pub pfnLocalLeaveCallback: PMGM_LOCAL_LEAVE_CALLBACK,
    pub pfnDisableIgmpCallback: PMGM_DISABLE_IGMP_CALLBACK,
    pub pfnEnableIgmpCallback: PMGM_ENABLE_IGMP_CALLBACK,
}
pub const RRAS_SERVICE_NAME: windows_core::PCWSTR = windows_core::w!("RemoteAccess");
pub const RTM_BLOCK_METHODS: u32 = 1u32;
pub const RTM_CHANGE_NOTIFICATION: RTM_EVENT_TYPE = RTM_EVENT_TYPE(3i32);
pub const RTM_CHANGE_TYPE_ALL: u32 = 1u32;
pub const RTM_CHANGE_TYPE_BEST: u32 = 2u32;
pub const RTM_CHANGE_TYPE_FORWARDING: u32 = 4u32;
pub const RTM_DEST_FLAG_DONT_FORWARD: u32 = 4u32;
pub const RTM_DEST_FLAG_FWD_ENGIN_ADD: u32 = 2u32;
pub const RTM_DEST_FLAG_NATURAL_NET: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RTM_DEST_INFO {
    pub DestHandle: isize,
    pub DestAddress: RTM_NET_ADDRESS,
    pub LastChanged: super::super::Foundation::FILETIME,
    pub BelongsToViews: u32,
    pub NumberOfViews: u32,
    pub ViewInfo: [RTM_DEST_INFO_0; 1],
}
impl Default for RTM_DEST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RTM_DEST_INFO_0 {
    pub ViewId: i32,
    pub NumRoutes: u32,
    pub Route: isize,
    pub Owner: isize,
    pub DestFlags: u32,
    pub HoldRoute: isize,
}
pub const RTM_ENTITY_DEREGISTERED: RTM_EVENT_TYPE = RTM_EVENT_TYPE(1i32);
pub type RTM_ENTITY_EXPORT_METHOD = Option<unsafe extern "system" fn(callerhandle: isize, calleehandle: isize, input: *mut RTM_ENTITY_METHOD_INPUT, output: *mut RTM_ENTITY_METHOD_OUTPUT)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RTM_ENTITY_EXPORT_METHODS {
    pub NumMethods: u32,
    pub Methods: [RTM_ENTITY_EXPORT_METHOD; 1],
}
impl Default for RTM_ENTITY_EXPORT_METHODS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTM_ENTITY_ID {
    pub Anonymous: RTM_ENTITY_ID_0,
}
impl Default for RTM_ENTITY_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RTM_ENTITY_ID_0 {
    pub Anonymous: RTM_ENTITY_ID_0_0,
    pub EntityId: u64,
}
impl Default for RTM_ENTITY_ID_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RTM_ENTITY_ID_0_0 {
    pub EntityProtocolId: u32,
    pub EntityInstanceId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTM_ENTITY_INFO {
    pub RtmInstanceId: u16,
    pub AddressFamily: u16,
    pub EntityId: RTM_ENTITY_ID,
}
impl Default for RTM_ENTITY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RTM_ENTITY_METHOD_INPUT {
    pub MethodType: u32,
    pub InputSize: u32,
    pub InputData: [u8; 1],
}
impl Default for RTM_ENTITY_METHOD_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RTM_ENTITY_METHOD_OUTPUT {
    pub MethodType: u32,
    pub MethodStatus: u32,
    pub OutputSize: u32,
    pub OutputData: [u8; 1],
}
impl Default for RTM_ENTITY_METHOD_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RTM_ENTITY_REGISTERED: RTM_EVENT_TYPE = RTM_EVENT_TYPE(0i32);
pub const RTM_ENUM_ALL_DESTS: u32 = 0u32;
pub const RTM_ENUM_ALL_ROUTES: u32 = 0u32;
pub const RTM_ENUM_NEXT: u32 = 1u32;
pub const RTM_ENUM_OWN_DESTS: u32 = 16777216u32;
pub const RTM_ENUM_OWN_ROUTES: u32 = 65536u32;
pub const RTM_ENUM_RANGE: u32 = 2u32;
pub const RTM_ENUM_START: u32 = 0u32;
pub type RTM_EVENT_CALLBACK = Option<unsafe extern "system" fn(rtmreghandle: isize, eventtype: RTM_EVENT_TYPE, context1: *mut core::ffi::c_void, context2: *mut core::ffi::c_void) -> u32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTM_EVENT_TYPE(pub i32);
pub const RTM_MATCH_FULL: u32 = 65535u32;
pub const RTM_MATCH_INTERFACE: u32 = 16u32;
pub const RTM_MATCH_NEIGHBOUR: u32 = 2u32;
pub const RTM_MATCH_NEXTHOP: u32 = 8u32;
pub const RTM_MATCH_NONE: u32 = 0u32;
pub const RTM_MATCH_OWNER: u32 = 1u32;
pub const RTM_MATCH_PREF: u32 = 4u32;
pub const RTM_MAX_ADDRESS_SIZE: u32 = 16u32;
pub const RTM_MAX_VIEWS: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RTM_NET_ADDRESS {
    pub AddressFamily: u16,
    pub NumBits: u16,
    pub AddrBits: [u8; 16],
}
impl Default for RTM_NET_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RTM_NEXTHOP_CHANGE_NEW: u32 = 1u32;
pub const RTM_NEXTHOP_FLAGS_DOWN: u32 = 2u32;
pub const RTM_NEXTHOP_FLAGS_REMOTE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RTM_NEXTHOP_INFO {
    pub NextHopAddress: RTM_NET_ADDRESS,
    pub NextHopOwner: isize,
    pub InterfaceIndex: u32,
    pub State: u16,
    pub Flags: u16,
    pub EntitySpecificInfo: *mut core::ffi::c_void,
    pub RemoteNextHop: isize,
}
impl Default for RTM_NEXTHOP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RTM_NEXTHOP_LIST {
    pub NumNextHops: u16,
    pub NextHops: [isize; 1],
}
impl Default for RTM_NEXTHOP_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RTM_NEXTHOP_STATE_CREATED: u32 = 0u32;
pub const RTM_NEXTHOP_STATE_DELETED: u32 = 1u32;
pub const RTM_NOTIFY_ONLY_MARKED_DESTS: u32 = 65536u32;
pub const RTM_NUM_CHANGE_TYPES: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RTM_PREF_INFO {
    pub Metric: u32,
    pub Preference: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RTM_REGN_PROFILE {
    pub MaxNextHopsInRoute: u32,
    pub MaxHandlesInEnum: u32,
    pub ViewsSupported: u32,
    pub NumberOfViews: u32,
}
pub const RTM_RESUME_METHODS: u32 = 0u32;
pub const RTM_ROUTE_CHANGE_BEST: u32 = 65536u32;
pub const RTM_ROUTE_CHANGE_FIRST: u32 = 1u32;
pub const RTM_ROUTE_CHANGE_NEW: u32 = 2u32;
pub const RTM_ROUTE_EXPIRED: RTM_EVENT_TYPE = RTM_EVENT_TYPE(2i32);
pub const RTM_ROUTE_FLAGS_BLACKHOLE: u32 = 2u32;
pub const RTM_ROUTE_FLAGS_DISCARD: u32 = 4u32;
pub const RTM_ROUTE_FLAGS_INACTIVE: u32 = 8u32;
pub const RTM_ROUTE_FLAGS_LIMITED_BC: u32 = 1024u32;
pub const RTM_ROUTE_FLAGS_LOCAL: u32 = 16u32;
pub const RTM_ROUTE_FLAGS_LOCAL_MCAST: u32 = 512u32;
pub const RTM_ROUTE_FLAGS_LOOPBACK: u32 = 128u32;
pub const RTM_ROUTE_FLAGS_MARTIAN: u32 = 1u32;
pub const RTM_ROUTE_FLAGS_MCAST: u32 = 256u32;
pub const RTM_ROUTE_FLAGS_MYSELF: u32 = 64u32;
pub const RTM_ROUTE_FLAGS_ONES_NETBC: u32 = 16384u32;
pub const RTM_ROUTE_FLAGS_ONES_SUBNETBC: u32 = 32768u32;
pub const RTM_ROUTE_FLAGS_REMOTE: u32 = 32u32;
pub const RTM_ROUTE_FLAGS_ZEROS_NETBC: u32 = 4096u32;
pub const RTM_ROUTE_FLAGS_ZEROS_SUBNETBC: u32 = 8192u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RTM_ROUTE_INFO {
    pub DestHandle: isize,
    pub RouteOwner: isize,
    pub Neighbour: isize,
    pub State: u8,
    pub Flags1: u8,
    pub Flags: u16,
    pub PrefInfo: RTM_PREF_INFO,
    pub BelongsToViews: u32,
    pub EntitySpecificInfo: *mut core::ffi::c_void,
    pub NextHopsList: RTM_NEXTHOP_LIST,
}
impl Default for RTM_ROUTE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RTM_ROUTE_STATE_CREATED: u32 = 0u32;
pub const RTM_ROUTE_STATE_DELETED: u32 = 2u32;
pub const RTM_ROUTE_STATE_DELETING: u32 = 1u32;
pub const RTM_VIEW_ID_MCAST: u32 = 1u32;
pub const RTM_VIEW_ID_UCAST: u32 = 0u32;
pub const RTM_VIEW_MASK_ALL: u32 = 4294967295u32;
pub const RTM_VIEW_MASK_ANY: u32 = 0u32;
pub const RTM_VIEW_MASK_MCAST: u32 = 2u32;
pub const RTM_VIEW_MASK_NONE: u32 = 0u32;
pub const RTM_VIEW_MASK_SIZE: u32 = 32u32;
pub const RTM_VIEW_MASK_UCAST: u32 = 1u32;
pub type RasCustomDeleteEntryNotifyFn = Option<unsafe extern "system" fn(lpszphonebook: windows_core::PCWSTR, lpszentry: windows_core::PCWSTR, dwflags: u32) -> u32>;
pub type RasCustomDialDlgFn = Option<unsafe extern "system" fn(hinstdll: super::super::Foundation::HINSTANCE, dwflags: u32, lpszphonebook: windows_core::PCWSTR, lpszentry: windows_core::PCWSTR, lpszphonenumber: windows_core::PCWSTR, lpinfo: *mut RASDIALDLG, pvinfo: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type RasCustomDialFn = Option<unsafe extern "system" fn(hinstdll: super::super::Foundation::HINSTANCE, lprasdialextensions: *mut RASDIALEXTENSIONS, lpszphonebook: windows_core::PCWSTR, lprasdialparams: *mut RASDIALPARAMSA, dwnotifiertype: u32, lpvnotifier: *mut core::ffi::c_void, lphrasconn: *mut HRASCONN, dwflags: u32) -> u32>;
pub type RasCustomEntryDlgFn = Option<unsafe extern "system" fn(hinstdll: super::super::Foundation::HINSTANCE, lpszphonebook: windows_core::PCWSTR, lpszentry: windows_core::PCWSTR, lpinfo: *mut RASENTRYDLGA, dwflags: u32) -> windows_core::BOOL>;
pub type RasCustomHangUpFn = Option<unsafe extern "system" fn(hrasconn: HRASCONN) -> u32>;
pub type RasCustomScriptExecuteFn = Option<unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, lpszphonebook: windows_core::PCWSTR, lpszentryname: windows_core::PCWSTR, pfnrasgetbuffer: PFNRASGETBUFFER, pfnrasfreebuffer: PFNRASFREEBUFFER, pfnrassendbuffer: PFNRASSENDBUFFER, pfnrasreceivebuffer: PFNRASRECEIVEBUFFER, pfnrasretrievebuffer: PFNRASRETRIEVEBUFFER, hwnd: super::super::Foundation::HWND, prasdialparams: *mut RASDIALPARAMSA, pvreserved: *mut core::ffi::c_void) -> u32>;
pub const SECURITYMSG_ERROR: SECURITY_MESSAGE_MSG_ID = SECURITY_MESSAGE_MSG_ID(3u32);
pub const SECURITYMSG_FAILURE: SECURITY_MESSAGE_MSG_ID = SECURITY_MESSAGE_MSG_ID(2u32);
pub const SECURITYMSG_SUCCESS: SECURITY_MESSAGE_MSG_ID = SECURITY_MESSAGE_MSG_ID(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECURITY_MESSAGE {
    pub dwMsgId: SECURITY_MESSAGE_MSG_ID,
    pub hPort: isize,
    pub dwError: u32,
    pub UserName: [i8; 257],
    pub Domain: [i8; 16],
}
impl Default for SECURITY_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECURITY_MESSAGE_MSG_ID(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SOURCE_GROUP_ENTRY {
    pub dwSourceAddr: u32,
    pub dwSourceMask: u32,
    pub dwGroupAddr: u32,
    pub dwGroupMask: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SSTP_CERT_INFO {
    pub isDefault: windows_core::BOOL,
    pub certBlob: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SSTP_CONFIG_PARAMS {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
    pub isUseHttps: windows_core::BOOL,
    pub certAlgorithm: u32,
    pub sstpCertDetails: SSTP_CERT_INFO,
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct VPN_TS_IP_ADDRESS {
    pub Type: u16,
    pub Anonymous: VPN_TS_IP_ADDRESS_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for VPN_TS_IP_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union VPN_TS_IP_ADDRESS_0 {
    pub v4: super::super::Networking::WinSock::IN_ADDR,
    pub v6: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for VPN_TS_IP_ADDRESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VS_Default: u32 = 0u32;
pub const VS_GREOnly: u32 = 9u32;
pub const VS_Ikev2First: u32 = 8u32;
pub const VS_Ikev2Only: u32 = 7u32;
pub const VS_Ikev2Sstp: u32 = 14u32;
pub const VS_L2tpFirst: u32 = 4u32;
pub const VS_L2tpOnly: u32 = 3u32;
pub const VS_L2tpSstp: u32 = 13u32;
pub const VS_PptpFirst: u32 = 2u32;
pub const VS_PptpOnly: u32 = 1u32;
pub const VS_PptpSstp: u32 = 12u32;
pub const VS_ProtocolList: u32 = 15u32;
pub const VS_SstpFirst: u32 = 6u32;
pub const VS_SstpOnly: u32 = 5u32;
pub const WARNING_MSG_ALIAS_NOT_ADDED: u32 = 644u32;
pub const WM_RASDIALEVENT: u32 = 52429u32;
