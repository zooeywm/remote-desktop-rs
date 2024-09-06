use remote_desk_kernel::Result;

slint::include_modules!();

#[tokio::main(worker_threads = 32)]
async fn main() -> Result<()> { Ok(()) }
