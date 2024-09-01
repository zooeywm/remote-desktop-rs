use tokio::runtime::Handle;

pub fn tokio_handle() -> Handle { tokio::runtime::Handle::current() }
