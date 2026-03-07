use cynic::impl_scalar;
use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use std::ops::Deref;
use std::str::FromStr;

pub mod id;

#[cynic::schema("netbox")]
mod schema {}

id::create_id!(VxlanId);
id::create_id!(DeviceId);
id::create_id!(InterfaceId);
id::create_id!(VlanId);
id::create_id!(TenantId);
id::create_id!(IpAddressId);
id::create_id!(WlanGroupId);
id::create_id!(WlanId);
id::create_id!(VlanGroupId);

id::create_from_str!(IpNetWrapper, IpNet, "an ip address including netmask");
id::create_from_str!(IpWrapper, IpAddr, "an ip address without netmask");

#[derive(cynic::QueryFragment, Debug, PartialEq, Eq, Default, Clone)]
#[cynic(graphql_type = "Query")]
pub struct Query {
    #[cynic(rename = "l2vpn_list")]
    l2vpn_list: Vec<VxLanData>,
    #[cynic(rename = "device_list")]
    #[arguments(filters:{platform:{name:{exact:"routeros"}}})]
    device_list: Vec<DeviceData>,
    #[cynic(rename = "wireless_lan_group_list")]
    wireless_lan_group_list: Vec<WlanGroupData>,
    #[cynic(rename = "vlan_group_list")]
    vlan_group_list: Vec<VlanGroupData>,
}
pub struct TopologyData {
    pub vxlan: HashMap<VxlanId, VxLanData>,
    pub device: HashMap<DeviceId, DeviceData>,
    pub wlan_group: HashMap<WlanGroupId, WlanGroupData>,
    pub vlan_group: HashMap<VlanGroupId, VlanGroupData>,
}
impl From<Query> for TopologyData {
    fn from(value: Query) -> Self {
        TopologyData {
            vxlan: HashMap::from_iter(value.l2vpn_list.into_iter().map(|v| (v.id, v))),
            device: HashMap::from_iter(value.device_list.into_iter().map(|v| (v.id, v))),
            wlan_group: HashMap::from_iter(
                value.wireless_lan_group_list.into_iter().map(|v| (v.id, v)),
            ),
            vlan_group: HashMap::from_iter(value.vlan_group_list.into_iter().map(|v| (v.id, v))),
        }
    }
}
#[derive(cynic::QueryFragment, Debug, PartialEq, Eq, Clone)]
#[cynic(graphql_type = "DeviceType")]
pub struct DeviceData {
    pub id: DeviceId,
    pub name: Option<String>,
    #[cynic(rename = "primary_ip4")]
    pub primary_ip4: Option<IpAddressData>,
    #[cynic(rename = "primary_ip6")]
    pub primary_ip6: Option<IpAddressData>,
    pub tenant: Option<TenantData>,
    #[cynic(rename = "custom_field_data")]
    pub custom_field_data: DeviceCustomFields,
    #[arguments(filters:{untagged_vlan:{vid:{filter_lookup:{is_null:false}}}})]
    pub interfaces: Vec<InterfaceData>,
}
impl DeviceData {
    pub fn primary_address(&self) -> Option<IpAddressData> {
        self.primary_ip6.or(self.primary_ip4)
    }
}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct DeviceCustomFields {
    wlan_group: Option<u32>,
}
impl_scalar!(DeviceCustomFields, schema::JSON);
impl DeviceCustomFields {
    pub fn wlan_group(&self) -> Option<WlanGroupId> {
        self.wlan_group.map(WlanGroupId)
    }
}
#[derive(cynic::QueryFragment, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cynic(graphql_type = "IPAddressType")]
pub struct IpAddressData {
    pub id: IpAddressId,
    pub address: IpNetWrapper,
}
#[derive(cynic::QueryFragment, Debug, Clone, PartialEq, Eq, Hash)]
#[cynic(graphql_type = "L2VPNType")]
pub struct VxLanData {
    pub id: VxlanId,
    pub name: String,
    #[cynic(rename = "type")]
    pub type_: String,
    pub identifier: Option<U32>,
    pub terminations: Vec<VxLanTermination>,
}
#[derive(cynic::QueryFragment, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cynic(graphql_type = "L2VPNTerminationType")]
pub struct VxLanTermination {
    #[cynic(rename = "assigned_object")]
    pub assigned_object: L2VpnassignmentType,
}
#[derive(cynic::InlineFragments, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cynic(graphql_type = "L2VPNAssignmentType")]
pub enum L2VpnassignmentType {
    InterfaceType(InterfaceReference),
    Vlantype(VlanReference),
    #[cynic(fallback)]
    Unknown,
}
#[derive(cynic::QueryFragment, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cynic(graphql_type = "InterfaceType")]
pub struct InterfaceReference {
    pub id: InterfaceId,
}
#[derive(cynic::QueryFragment, Debug, Clone, PartialEq, Eq, Hash)]
#[cynic(graphql_type = "InterfaceType")]
pub struct InterfaceData {
    pub id: InterfaceId,
    pub name: String,
    #[cynic(rename = "untagged_vlan")]
    pub untagged_vlan: Option<VlanReference>,
}

#[derive(cynic::QueryFragment, Debug, Clone, PartialEq, Eq, Hash)]
#[cynic(graphql_type = "WirelessLANGroupType")]
pub struct WlanGroupData {
    pub id: WlanGroupId,
    #[cynic(rename = "custom_fields")]
    pub custom_fields: WlanCustomFields,
    pub name: String,
    #[cynic(rename = "wireless_lans")]
    pub wireless_lans: Vec<WlanData>,
}

#[derive(cynic::QueryFragment, Debug, Clone, PartialEq, Eq, Hash)]
#[cynic(graphql_type = "VLANGroupType")]
pub struct VlanGroupData {
    pub id: VlanGroupId,
    pub name: String,
    pub vlans: Vec<VlanData>,
}
#[derive(cynic::QueryFragment, Debug, Clone, PartialEq, Eq, Hash)]
#[cynic(graphql_type = "VLANType")]
pub struct VlanData {
    pub id: VlanId,
    pub name: String,
    pub vid: U16,
    #[cynic(rename = "interfaces_as_untagged")]
    pub interfaces_as_untagged: Vec<InterfaceReference>,
}
#[derive(Debug, Deserialize, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WlanCustomFields {
    wlan_mgmt: Option<u32>,
    controller: Option<u32>,
}
impl_scalar!(WlanCustomFields, schema::JSON);
impl WlanCustomFields {
    pub fn controller(&self) -> Option<DeviceId> {
        self.controller.map(DeviceId)
    }
    pub fn mgmt_vlan(&self) -> Option<VlanId> {
        self.wlan_mgmt.map(VlanId)
    }
}
#[derive(cynic::QueryFragment, Debug, Clone, PartialEq, Eq, Hash)]
#[cynic(graphql_type = "WirelessLANType")]
pub struct WlanReference {
    pub id: WlanId,
}
#[derive(cynic::QueryFragment, Debug, Clone, PartialEq, Eq, Hash)]
#[cynic(graphql_type = "WirelessLANType")]
pub struct WlanData {
    pub id: WlanId,
    pub ssid: String,
    #[cynic(rename = "auth_type")]
    pub auth_type: Option<String>,
    #[cynic(rename = "auth_psk")]
    pub auth_psk: String,
    pub vlan: Option<VlanReference>,
}

#[derive(cynic::QueryFragment, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cynic(graphql_type = "VLANType")]
pub struct VlanReference {
    pub id: VlanId,
}

#[derive(cynic::QueryFragment, Debug, Clone, PartialEq, Eq, Hash)]
#[cynic(graphql_type = "TenantType")]
pub struct TenantData {
    pub id: TenantId,
    #[cynic(rename = "custom_field_data")]
    pub custom_field_data: TenancyCustomFields,
}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct TenancyCustomFields {
    pub mikrotik_credentials: Option<Box<str>>,
}
impl_scalar!(TenancyCustomFields, schema::JSON);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Ord, PartialOrd)]
pub struct U16(u16);
impl From<U16> for u16 {
    fn from(value: U16) -> Self {
        value.0
    }
}
impl From<u16> for U16 {
    fn from(value: u16) -> Self {
        U16(value)
    }
}

impl Display for U16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Deref for U16 {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl_scalar!(U16, schema::Int);
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Ord, PartialOrd)]
pub struct U32(pub u32);
impl From<U32> for u32 {
    fn from(value: U32) -> Self {
        value.0
    }
}
impl From<u32> for U32 {
    fn from(value: u32) -> Self {
        U32(value)
    }
}

impl Display for U32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Deref for U32 {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl_scalar!(U32, schema::Int);
