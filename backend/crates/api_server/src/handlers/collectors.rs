use crate::{db, notifications::handle_endpoints};
use actix_web::{HttpResponse, Responder, delete, get, patch, put};
use serde::Deserialize;

use crate::{
    AppState, WebSocketType,
    handlers::{ResponseBodyType, handle_query_error},
};

use actix_web::{post, web};
use shared::structs::{
    collector_info::CollectorInfo,
    db::{CollectorTable, DriveTable, NetworkInterfaceTable, EndpointInsert},
    endpoints::{Endpoint, EndpointResult},
    metrics::Metrics,
};


// params structs
#[derive(Deserialize, utoipa::IntoParams)]
struct MetricsQueryParams {
    time_limit_hours: Option<i32>,
    resolution: Option<i32>,
}

#[derive(Deserialize, utoipa::ToSchema)]
struct RenameCollectorStruct {
    name: String,
}

// handlers
#[utoipa::path(
    responses(
        (status = 200, description="Collectors from database", body=Vec<CollectorTable>),
        (status = 500, description="Interval Server Error", body=String)
    ), 
)]
#[get("/collectors")]
async fn collectors(state: web::Data<AppState>) -> impl Responder {
    let result = db::get_collectors(&state.pool).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="Collector with ID from database", body=CollectorTable),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector")
    )
)]
#[get("/collector/{id}")]
async fn get_collector(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = db::get_collector_by_id(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="Collector with ID from database", body=CollectorTable),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
        MetricsQueryParams
    )
)]
#[get("/collector/{id}/metrics")]
async fn get_collector_metrics(
    state: web::Data<AppState>,
    id: web::Path<i32>,
    query: web::Query<MetricsQueryParams>,
) -> impl Responder {
    let table =
        db::get_metrics_table(&state.pool, id.into_inner(), query.time_limit_hours, None).await;
    if table.is_err() {
        return handle_query_error(table, ResponseBodyType::Json);
    }

    let metrics = Metrics::from_metrics_table(table.unwrap());
    if query.resolution.is_none() {
        return handle_query_error(metrics, ResponseBodyType::Json);
    }

    let metrics = metrics.unwrap();

    let chunk_size = metrics.len() / (query.resolution.unwrap() as usize);
    if chunk_size == 0 {
        // TODO what do i do here - maybe some 4xx response code and display on frontend?
        return handle_query_error(Ok(metrics), ResponseBodyType::Json);
    }

    let result = metrics
        .chunks(chunk_size)
        .map(Metrics::average)
        .collect::<Vec<Metrics>>();

    handle_query_error(Ok(result), ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="Drives of Collector with ID from database", body=Vec<DriveTable>),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[get("/collector/{id}/drives")]
async fn get_collector_drives(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = db::get_collector_drives(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="Network Interfaces of Collector with ID from database", body=Vec<NetworkInterfaceTable>),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[get("/collector/{id}/network_interfaces")]
async fn get_collector_network_interfaces(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::get_collector_network_interfaces(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="Endpoints of Collector with ID from database", body=Vec<Endpoint>),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[get("/collector/{id}/endpoints")]
async fn get_collector_endpoints(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = db::get_endpoints(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="New endpoint created", body=Endpoint),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
// post = create new
#[post("/collector/{id}/endpoints")]
async fn post_collector_endpoints(
    state: web::Data<AppState>,
    endpoint: web::Json<EndpointInsert>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::insert_endpoint(&state.pool, id.into_inner(), &endpoint).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="Endpoint updated"),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
// put = update existing
#[put("/collector/{id}/endpoints")]
async fn put_collector_endpoints(
    state: web::Data<AppState>,
    endpoint: web::Json<Endpoint>,
    _id: web::Path<i32>,
) -> impl Responder {
    let result = db::update_endpoint(&state.pool, &endpoint).await;
    handle_query_error(result, ResponseBodyType::None)
}

#[utoipa::path(
    responses(
        (status = 200, description="Endpoint Deleted"),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id_collector", Path, description="ID of Collector"),
        ("id_endpoint", Path, description="ID of Endpoint"),
    )
)]
#[delete("/collector/{id_collector}/endpoints/{id_endpoint}")]
async fn delete_collector_endpoints(
    state: web::Data<AppState>,
    id: web::Path<(i32, i32)>,
) -> impl Responder {
    let result = db::delete_endpoint(&state.pool, id.1).await;
    handle_query_error(result, ResponseBodyType::None)
}

#[utoipa::path(
    responses(
        (status = 200, description="Results of Endpoints measurements", body=Vec<EndpointResult>),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[get("/collector/{id}/endpoints_results")]
async fn get_collector_endpoint_results(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::get_endpoints_results(&state.pool, id.into_inner(), None).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="Results of Endpoints measurements from only the last measurements", body=Vec<EndpointResult>),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[get("/collector/{id}/endpoints_results/last")]
async fn get_collector_endpoint_results_last(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::get_collector_endpoints_results_last(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="Sucessfully inserted into database"),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[post("/collector/{id}/endpoints_results")]
async fn post_collector_endpoint_results(
    state: web::Data<AppState>,
    endpoint_results: web::Json<Vec<EndpointResult>>,
    id: web::Path<i32>,
) -> impl Responder {
    if endpoint_results.is_empty() {
        return HttpResponse::NotModified().finish();
    }

    let id = id.into_inner();

    // intsert into db
    let result = db::insert_endpoints_results(&state.pool, endpoint_results.clone()).await;

    if result.is_ok() {
        // send to websocket
        let _ = state.tx.send((
            WebSocketType::EndpointResult(endpoint_results.into_inner()),
            id,
        ));

        // evaluate notifications
        let _ = handle_endpoints(&state, id).await;
    }

    handle_query_error(result, ResponseBodyType::None)
}

#[utoipa::path(
    responses(
        (status = 201, description="Collector registered, returning ID of Collector", body=i32),
        (status = 500, description="Interval Server Error", body=String)
    ), 
)]
#[post("/collector/register")]
async fn collector_register(
    state: web::Data<AppState>,
    new_collector: web::Json<CollectorInfo>,
) -> impl Responder {
    let result = db::register_collector(&state.pool, &new_collector.into_inner()).await;

    match result {
        Ok(val) => HttpResponse::Created().body(val.to_string()),
        Err(val) => HttpResponse::InternalServerError().body(val.to_string()),
    }
}

#[utoipa::path(
    responses(
        (status = 200, description="Collector renamed"),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[patch("/collector/{id}/rename")]
async fn rename_collector(
    state: web::Data<AppState>,
    id: web::Path<i32>,
    body: web::Json<RenameCollectorStruct>,
) -> impl Responder {
    let result = db::rename_collector(&state.pool, id.into_inner(), body.into_inner().name).await;
    handle_query_error(result, ResponseBodyType::None)
}
