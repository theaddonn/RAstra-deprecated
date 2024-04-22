use crate::error::RastraError;
use crate::error::RastraError::DeviceOsUnknown;

#[derive(Debug)]
enum DeviceOS {
    Android,
    IOS,
    OSX,
    // MacOS
    FireOS,
    GearVR,
    Hololens,
    Win10,
    Win32,
    Dedicated,
    #[deprecated(since = "1.20.10", note = "TVOS is not supported anymore!")]
    TVOS,
    Orbis,
    // PlayStation
    NX,
    // Nintendo Switch
    XBOX,
    #[deprecated(since = "1.20.10", note = "Windows phone is not supported anymore!")]
    WP, // Windows Phone
}

impl DeviceOS {
    #[allow(deprecated)]
    fn get_id(&self) -> i32 {
        match self {
            DeviceOS::Android => 1,
            DeviceOS::IOS => 2,
            DeviceOS::OSX => 3,
            DeviceOS::FireOS => 4,
            DeviceOS::GearVR => 5,
            DeviceOS::Hololens => 6,
            DeviceOS::Win10 => 7,
            DeviceOS::Win32 => 8,
            DeviceOS::Dedicated => 9,
            DeviceOS::TVOS => 10,
            DeviceOS::Orbis => 11,
            DeviceOS::NX => 12,
            DeviceOS::XBOX => 13,
            DeviceOS::WP => 14,
        }
    }

    #[allow(deprecated)]
    fn from_id(id: &u8) -> Result<DeviceOS, RastraError> {
        match id {
            1 => Ok(DeviceOS::Android),
            2 => Ok(DeviceOS::Android),
            3 => Ok(DeviceOS::Android),
            4 => Ok(DeviceOS::Android),
            5 => Ok(DeviceOS::Android),
            6 => Ok(DeviceOS::Android),
            7 => Ok(DeviceOS::Android),
            8 => Ok(DeviceOS::Android),
            9 => Ok(DeviceOS::Android),
            10 => Ok(DeviceOS::Android),
            11 => Ok(DeviceOS::Android),
            12 => Ok(DeviceOS::Android),
            13 => Ok(DeviceOS::Android),
            14 => Ok(DeviceOS::Android),
            _ => Err(DeviceOsUnknown),
        }
    }
}

#[derive(Debug)]
pub struct Device {
    os: DeviceOS,
    model: String,
    id: uuid::Uuid,
}
