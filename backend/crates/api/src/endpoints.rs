use actix_web::{HttpResponse, Responder, get};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/data")]
async fn get_data() -> impl Responder {
    HttpResponse::NotImplemented().finish()
}

#[get("/sample_data")]
async fn get_sample_data() -> impl Responder {
    log::info!("/sample_data");

    let data: Vec<f64> = vec![
        0.0, 5.1282053, 3.809524, 8.653847, 11.428572, 3.846154, 13.333334, 7.692308, 6.542056,
        6.666667, 5.882353, 6.1855664, 2.9126215,
    ];

    HttpResponse::Ok().json(data)
}
