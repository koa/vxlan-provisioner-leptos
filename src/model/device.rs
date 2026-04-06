#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub struct DeviceListEntry {
    pub id: u32,
    pub name: String,
    pub device_type_name: String,
    pub role: Option<DeviceRole>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Copy)]
pub enum DeviceRole {
    Router,
    Switch,
    AccessPoint,
}
