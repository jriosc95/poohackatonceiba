use actix_web::HttpResponse;

pub async fn status() -> HttpResponse {
    HttpResponse::Ok().body("Prueba de concepto hackaton - corriendo!")
}