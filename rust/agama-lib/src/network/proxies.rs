//! D-Bus interface proxies for: `org.opensuse.Agama*.**.*`
//!
//! This code was generated by `zbus-xmlgen` `3.1.0` from DBus introspection data.`.
use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.opensuse.Agama.Network1.Connections",
    default_service = "org.opensuse.Agama.Network1",
    default_path = "/org/opensuse/Agama/Network1/connections"
)]
trait Connections {
    /// Add a new network connection.
    ///
    /// `name`: connection name.
    /// `ty`: connection type.
    fn add_connection(&self, name: &str, ty: u8) -> zbus::Result<()>;

    /// Apply method
    fn apply(&self) -> zbus::Result<()>;

    /// GetConnections method
    fn get_connections(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// RemoveConnection method
    fn remove_connection(&self, uuid: &str) -> zbus::Result<()>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Network1.Connection.Wireless",
    default_service = "org.opensuse.Agama.Network1",
    default_path = "/org/opensuse/Agama/Network1"
)]
trait Wireless {
    /// Mode property
    #[dbus_proxy(property)]
    fn mode(&self) -> zbus::Result<String>;
    fn set_mode(&self, value: String) -> zbus::Result<()>;

    /// Password property
    #[dbus_proxy(property)]
    fn password(&self) -> zbus::Result<String>;
    fn set_password(&self, value: &str) -> zbus::Result<()>;

    /// SSID property
    #[dbus_proxy(property, name = "SSID")]
    fn ssid(&self) -> zbus::Result<Vec<u8>>;
    fn set_ssid(&self, value: Vec<u8>) -> zbus::Result<()>;

    /// Security property
    #[dbus_proxy(property)]
    fn security(&self) -> zbus::Result<String>;
    fn set_security(&self, value: String) -> zbus::Result<()>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Network1.Connection",
    default_service = "org.opensuse.Agama.Network1",
    default_path = "/org/opensuse/Agama/Network1"
)]
trait Connection {
    /// Id property
    #[dbus_proxy(property)]
    fn id(&self) -> zbus::Result<String>;

    /// UUID property
    #[dbus_proxy(property, name = "UUID")]
    fn uuid(&self) -> zbus::Result<String>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Network1.Connection.IPv4",
    default_service = "org.opensuse.Agama.Network1",
    default_path = "/org/opensuse/Agama/Network1"
)]
trait IPv4 {
    /// Addresses property
    #[dbus_proxy(property)]
    fn addresses(&self) -> zbus::Result<Vec<(String, u32)>>;
    fn set_addresses(&self, value: &[(&str, u32)]) -> zbus::Result<()>;

    /// Gateway property
    #[dbus_proxy(property)]
    fn gateway(&self) -> zbus::Result<String>;
    fn set_gateway(&self, value: &str) -> zbus::Result<()>;

    /// Method property
    #[dbus_proxy(property)]
    fn method(&self) -> zbus::Result<u8>;
    fn set_method(&self, value: u8) -> zbus::Result<()>;

    /// Nameservers property
    #[dbus_proxy(property)]
    fn nameservers(&self) -> zbus::Result<Vec<String>>;
    fn set_nameservers(&self, value: &[&str]) -> zbus::Result<()>;
}
