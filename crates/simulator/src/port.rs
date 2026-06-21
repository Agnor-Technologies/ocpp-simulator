use std::net::TcpListener;

pub fn is_port_available(
    port: u16,
) -> bool {
    TcpListener::bind(
        ("127.0.0.1", port)
    )
    .is_ok()
}

pub fn allocate_port(
    requested: u16,
) -> Option<u16> {
    for port in requested..=u16::MAX {
        if is_port_available(port) {
            return Some(port);
        }
    }

    None
}
