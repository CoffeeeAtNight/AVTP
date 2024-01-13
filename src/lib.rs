mod header;
mod tcp_communication;

pub use header::Header;
pub use tcp_communication::{send_data, receive_data};
