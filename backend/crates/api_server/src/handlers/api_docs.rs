use crate::handlers::*;

#[derive(utoipa::OpenApi)]
#[openapi(paths(
    hello,
    metrics_post,
    collector_register,
    collectors,
    get_collector,
    get_collector_metrics,
    get_collector_drives,
    get_collector_network_interfaces,
    get_collector_endpoints,
    post_collector_endpoints,
    put_collector_endpoints,
    delete_collector_endpoints,
    get_collector_endpoint_results,
    get_collector_endpoint_results_last,
    post_collector_endpoint_results,
    get_collector_notifications,
    delete_collector_notifications,
    delete_collector_notifications_all,
    rename_collector,
    get_collector_metrics_thresholds,
    get_collector_metrics_thresholds_available_metric_types,
    get_collector_metrics_thresholds_available_drives,
    get_collector_metrics_thresholds_available_networks_upload,
    get_collector_metrics_thresholds_available_networks_download,
    get_collector_endpoints_thresholds,
    get_collector_endpoints_thresholds_join,
    get_collector_endpoints_available_endpoints,
    post_metrics_thresholds,
    post_endpoints_thresholds,
    delete_metrics_thresholds,
    delete_endpoints_thresholds
))]
pub struct ApiDoc;
