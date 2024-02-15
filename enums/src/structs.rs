use crate::enums::IpAddrKind;

pub struct IpAddr {
    pub kind: IpAddrKind,
    pub address: String,
}
