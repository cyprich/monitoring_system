mod endpoints;
mod metrics;

pub use endpoints::handle_endpoints;
pub use metrics::handle_metrics;

// TODO it's making notifications too often, maybe some cooldown would be nice
