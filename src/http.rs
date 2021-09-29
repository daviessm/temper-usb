use crate::temper::TemperDevice;
use actix_web::{get, web, Responder};
use std::sync::{Arc, Mutex};

#[get("/")]
pub(crate) async fn index(shared_temper: web::Data<Arc<Mutex<TemperDevice>>>) -> impl Responder {
    format!("{}", shared_temper.lock().unwrap().get_temp())
}
