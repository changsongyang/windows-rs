windows_core::imp::define_interface!(IPhoneCallBlockedTriggerDetails, IPhoneCallBlockedTriggerDetails_Vtbl, 0xa4a690a2_e4c1_427f_864e_e470477ddb67);
impl windows_core::RuntimeType for IPhoneCallBlockedTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallBlockedTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PhoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LineId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CallBlockedReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallBlockedReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneCallOriginDataRequestTriggerDetails, IPhoneCallOriginDataRequestTriggerDetails_Vtbl, 0x6e9b5b3f_c54b_4e82_4cc9_e329a4184592);
impl windows_core::RuntimeType for IPhoneCallOriginDataRequestTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOriginDataRequestTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneIncomingCallDismissedTriggerDetails, IPhoneIncomingCallDismissedTriggerDetails_Vtbl, 0xbad30276_83b6_5732_9c38_0c206546196a);
impl windows_core::RuntimeType for IPhoneIncomingCallDismissedTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneIncomingCallDismissedTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DismissalTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub TextReplyMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneIncomingCallDismissedReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneIncomingCallNotificationTriggerDetails, IPhoneIncomingCallNotificationTriggerDetails_Vtbl, 0x2b0e6044_9b32_5d42_8222_d2812e39fb21);
impl windows_core::RuntimeType for IPhoneIncomingCallNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneIncomingCallNotificationTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CallId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneLineChangedTriggerDetails, IPhoneLineChangedTriggerDetails_Vtbl, 0xc6d321e7_d11d_40d8_b2b7_e40a01d66249);
impl windows_core::RuntimeType for IPhoneLineChangedTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineChangedTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ChangeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneLineChangeKind) -> windows_core::HRESULT,
    pub HasLinePropertyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, PhoneLineProperties, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneNewVoicemailMessageTriggerDetails, IPhoneNewVoicemailMessageTriggerDetails_Vtbl, 0x13a8c01b_b831_48d3_8ba9_8d22a6580dcf);
impl windows_core::RuntimeType for IPhoneNewVoicemailMessageTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNewVoicemailMessageTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub VoicemailCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub OperatorMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PhoneCallBlockedReason(pub i32);
impl PhoneCallBlockedReason {
    pub const InCallBlockingList: Self = Self(0i32);
    pub const PrivateNumber: Self = Self(1i32);
    pub const UnknownNumber: Self = Self(2i32);
}
impl windows_core::TypeKind for PhoneCallBlockedReason {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PhoneCallBlockedReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneCallBlockedReason;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhoneCallBlockedTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneCallBlockedTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneCallBlockedTriggerDetails {
    pub fn PhoneNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn LineId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CallBlockedReason(&self) -> windows_core::Result<PhoneCallBlockedReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallBlockedReason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PhoneCallBlockedTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneCallBlockedTriggerDetails>();
}
unsafe impl windows_core::Interface for PhoneCallBlockedTriggerDetails {
    type Vtable = <IPhoneCallBlockedTriggerDetails as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPhoneCallBlockedTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneCallBlockedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneCallBlockedTriggerDetails";
}
unsafe impl Send for PhoneCallBlockedTriggerDetails {}
unsafe impl Sync for PhoneCallBlockedTriggerDetails {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhoneCallOriginDataRequestTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneCallOriginDataRequestTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneCallOriginDataRequestTriggerDetails {
    pub fn RequestId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PhoneNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for PhoneCallOriginDataRequestTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneCallOriginDataRequestTriggerDetails>();
}
unsafe impl windows_core::Interface for PhoneCallOriginDataRequestTriggerDetails {
    type Vtable = <IPhoneCallOriginDataRequestTriggerDetails as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPhoneCallOriginDataRequestTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneCallOriginDataRequestTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneCallOriginDataRequestTriggerDetails";
}
unsafe impl Send for PhoneCallOriginDataRequestTriggerDetails {}
unsafe impl Sync for PhoneCallOriginDataRequestTriggerDetails {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PhoneIncomingCallDismissedReason(pub i32);
impl PhoneIncomingCallDismissedReason {
    pub const Unknown: Self = Self(0i32);
    pub const CallRejected: Self = Self(1i32);
    pub const TextReply: Self = Self(2i32);
    pub const ConnectionLost: Self = Self(3i32);
}
impl windows_core::TypeKind for PhoneIncomingCallDismissedReason {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PhoneIncomingCallDismissedReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedReason;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhoneIncomingCallDismissedTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneIncomingCallDismissedTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneIncomingCallDismissedTriggerDetails {
    pub fn LineId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PhoneNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DismissalTime(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DismissalTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TextReplyMessage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextReplyMessage)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Reason(&self) -> windows_core::Result<PhoneIncomingCallDismissedReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PhoneIncomingCallDismissedTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneIncomingCallDismissedTriggerDetails>();
}
unsafe impl windows_core::Interface for PhoneIncomingCallDismissedTriggerDetails {
    type Vtable = <IPhoneIncomingCallDismissedTriggerDetails as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPhoneIncomingCallDismissedTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneIncomingCallDismissedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedTriggerDetails";
}
unsafe impl Send for PhoneIncomingCallDismissedTriggerDetails {}
unsafe impl Sync for PhoneIncomingCallDismissedTriggerDetails {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhoneIncomingCallNotificationTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneIncomingCallNotificationTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneIncomingCallNotificationTriggerDetails {
    pub fn LineId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CallId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for PhoneIncomingCallNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneIncomingCallNotificationTriggerDetails>();
}
unsafe impl windows_core::Interface for PhoneIncomingCallNotificationTriggerDetails {
    type Vtable = <IPhoneIncomingCallNotificationTriggerDetails as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPhoneIncomingCallNotificationTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneIncomingCallNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneIncomingCallNotificationTriggerDetails";
}
unsafe impl Send for PhoneIncomingCallNotificationTriggerDetails {}
unsafe impl Sync for PhoneIncomingCallNotificationTriggerDetails {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PhoneLineChangeKind(pub i32);
impl PhoneLineChangeKind {
    pub const Added: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
    pub const PropertiesChanged: Self = Self(2i32);
}
impl windows_core::TypeKind for PhoneLineChangeKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PhoneLineChangeKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneLineChangeKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhoneLineChangedTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneLineChangedTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneLineChangedTriggerDetails {
    pub fn LineId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ChangeType(&self) -> windows_core::Result<PhoneLineChangeKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChangeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasLinePropertyChanged(&self, lineproperty: PhoneLineProperties) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasLinePropertyChanged)(windows_core::Interface::as_raw(this), lineproperty, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PhoneLineChangedTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneLineChangedTriggerDetails>();
}
unsafe impl windows_core::Interface for PhoneLineChangedTriggerDetails {
    type Vtable = <IPhoneLineChangedTriggerDetails as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPhoneLineChangedTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneLineChangedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneLineChangedTriggerDetails";
}
unsafe impl Send for PhoneLineChangedTriggerDetails {}
unsafe impl Sync for PhoneLineChangedTriggerDetails {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PhoneLineProperties(pub u32);
impl PhoneLineProperties {
    pub const None: Self = Self(0u32);
    pub const BrandingOptions: Self = Self(1u32);
    pub const CanDial: Self = Self(2u32);
    pub const CellularDetails: Self = Self(4u32);
    pub const DisplayColor: Self = Self(8u32);
    pub const DisplayName: Self = Self(16u32);
    pub const NetworkName: Self = Self(32u32);
    pub const NetworkState: Self = Self(64u32);
    pub const Transport: Self = Self(128u32);
    pub const Voicemail: Self = Self(256u32);
}
impl windows_core::TypeKind for PhoneLineProperties {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PhoneLineProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneLineProperties;u4)");
}
impl PhoneLineProperties {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PhoneLineProperties {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PhoneLineProperties {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PhoneLineProperties {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PhoneLineProperties {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PhoneLineProperties {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhoneNewVoicemailMessageTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneNewVoicemailMessageTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneNewVoicemailMessageTriggerDetails {
    pub fn LineId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn VoicemailCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VoicemailCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OperatorMessage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OperatorMessage)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for PhoneNewVoicemailMessageTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneNewVoicemailMessageTriggerDetails>();
}
unsafe impl windows_core::Interface for PhoneNewVoicemailMessageTriggerDetails {
    type Vtable = <IPhoneNewVoicemailMessageTriggerDetails as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPhoneNewVoicemailMessageTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneNewVoicemailMessageTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneNewVoicemailMessageTriggerDetails";
}
unsafe impl Send for PhoneNewVoicemailMessageTriggerDetails {}
unsafe impl Sync for PhoneNewVoicemailMessageTriggerDetails {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PhoneTriggerType(pub i32);
impl PhoneTriggerType {
    pub const NewVoicemailMessage: Self = Self(0i32);
    pub const CallHistoryChanged: Self = Self(1i32);
    pub const LineChanged: Self = Self(2i32);
    pub const AirplaneModeDisabledForEmergencyCall: Self = Self(3i32);
    pub const CallOriginDataRequest: Self = Self(4i32);
    pub const CallBlocked: Self = Self(5i32);
    pub const IncomingCallDismissed: Self = Self(6i32);
    pub const IncomingCallNotification: Self = Self(7i32);
}
impl windows_core::TypeKind for PhoneTriggerType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PhoneTriggerType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneTriggerType;i4)");
}
