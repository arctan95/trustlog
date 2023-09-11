#[cfg(feature = "opentelemetry")]
pub mod opentelemetry {
    pub use opentelemetry_proto::proto;
}
pub use trustlog_common::{Error, Result};
