use flutter_rust_bridge::*;
use ximu3::connection_info::*;
use ximu3::charging_status::*;
use std::net::Ipv4Addr;
use std::fmt;

// -------------------------------------------------------------------------------------------------
// Example enum wrapping

pub enum ChargingStatusF {
    NotConnected,
    Charging,
    ChargingComplete,
}

impl From<ChargingStatusF> for ChargingStatus {
    fn from(charging_status: ChargingStatusF) -> Self {
        match charging_status {
            ChargingStatusF::NotConnected => ChargingStatus::NotConnected,
            ChargingStatusF::Charging => ChargingStatus::Charging,
            ChargingStatusF::ChargingComplete => ChargingStatus::ChargingComplete,
        }
    }
}

impl From<ChargingStatus> for ChargingStatusF {
    fn from(charging_status: ChargingStatus) -> Self {
        match charging_status {
            ChargingStatus::NotConnected => ChargingStatusF::NotConnected,
            ChargingStatus::Charging => ChargingStatusF::Charging,
            ChargingStatus::ChargingComplete => ChargingStatusF::ChargingComplete,
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Example struct wrapping

pub struct UsbConnectionInfoF {
    pub port_name: String,
}

impl From<UsbConnectionInfo> for UsbConnectionInfoF {
    fn from(connection_info: UsbConnectionInfo) -> Self {
        UsbConnectionInfoF {
            port_name: connection_info.port_name,
        }
    }
}

impl From<UsbConnectionInfoF> for UsbConnectionInfo {
    fn from(connection_info: UsbConnectionInfoF) -> Self {
        UsbConnectionInfo {
            port_name: connection_info.port_name,
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Example struct wrapping

pub struct TcpConnectionInfoF {
    pub ip_address: String,
    pub port: u16,
}

impl From<&TcpConnectionInfo> for TcpConnectionInfoF {
    fn from(connection_info: &TcpConnectionInfo) -> Self {
        TcpConnectionInfoF {
            ip_address: connection_info.ip_address.to_string(),
            port: connection_info.port,
        }
    }
}

impl From<&TcpConnectionInfoF> for TcpConnectionInfo {
    fn from(connection_info: &TcpConnectionInfoF) -> Self {
        TcpConnectionInfo {
            ip_address: connection_info.ip_address.parse().ok().unwrap_or_else(|| { Ipv4Addr::new(0, 0, 0, 0) }),
            port: connection_info.port,
        }
    }
}

impl fmt::Display for TcpConnectionInfoF {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x: TcpConnectionInfo = self.into();
        write!(formatter, "{}", x.to_string())
    }
}

// impl TcpConnectionInfoF {
//     pub fn new() -> TcpConnectionInfoF {
//         let info = TcpConnectionInfo {
//             ip_address: Ipv4Addr::new(192, 167, 1, 1),
//             port: 7000,
//         };
//         (&info).into()
//     }
// }

// -------------------------------------------------------------------------------------------------
// Misc

pub fn get_usb_info() -> SyncReturn<UsbConnectionInfoF> {
    let info = ximu3::connection_info::UsbConnectionInfo {
        port_name: "my port".to_string(),
    };

    SyncReturn(info.into())
}

pub fn get_tcp_info() -> SyncReturn<TcpConnectionInfoF> {
    let info = TcpConnectionInfo {
        ip_address: Ipv4Addr::new(192, 167, 1, 1),
        port: 7000,
    };

    SyncReturn((&info).into())
}