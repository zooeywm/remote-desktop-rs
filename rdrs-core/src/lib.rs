pub mod error;
pub mod infra_trait;
/// TODO: move the main logic to the doamin layer.
pub mod model;
pub mod repository;
pub mod service;
/// TODO: implement more service_impl instead of directly implement in
/// infrastructure
pub mod service_impl;
