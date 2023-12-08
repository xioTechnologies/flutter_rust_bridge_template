use flutter_rust_bridge::*;
use ximu3::connection_info::*;

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

pub fn get_info() -> SyncReturn<UsbConnectionInfoF> {

    let info = ximu3::connection_info::UsbConnectionInfo{
        port_name: "my port".to_string(),
    };

    SyncReturn(info.into())
}
