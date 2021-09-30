mod args;
mod http;
mod mqtt;
mod temper;

use args::Args;
use std::{convert::TryInto, sync::{Arc, Mutex}, thread, time::Duration};

use actix_web::{App, HttpServer, rt::System, web::Data};
use futures::executor::block_on;
use sd_notify::NotifyState;

use crate::mqtt::MqttConnection;

#[actix_web::main]
async fn main() {
    let args = Arc::new(Args::parse().unwrap());
    let temper = temper::iterate_usb_devices().unwrap();
    let shared_temper = Arc::new(Mutex::new(temper));
    let shared_temper_clone1 = shared_temper.clone();
    let shared_temper_clone2 = shared_temper.clone();
    let mut ready = false;

    if args.mqtt() {
        let args = args.clone();
        thread::spawn(move || {
            let mqtt_cli = MqttConnection::new(
                shared_temper,
                args.mqtt_server(),
                args.mqtt_topic().to_string(),
                args.mqtt_frequency().try_into().unwrap(),
            );

            mqtt_cli.run_forever();
        });
        ready = true;
    }
    if args.http() {
        let args = args.clone();
        thread::spawn(move || {
            let sys = System::new("http-server");
            block_on(
                HttpServer::new(move || {
                    App::new()
                        .service(http::index)
                        .app_data(Data::new(shared_temper_clone1.clone()).clone())
                })
                .bind(format!("0.0.0.0:{}", args.http_port()))
                .unwrap()
                .run(),
            )
            .unwrap();

            sys.run().unwrap();
        });
        ready = true;
    }

    if ready {
        sd_notify::notify(false, &[NotifyState::Ready]).unwrap();
    }

    //If no requests for MQTT or HTTP, output a value and end
    if !args.http() && !args.mqtt() {
        println!("{}", shared_temper_clone2.lock().unwrap().get_temp());
    } else {
        loop {
            thread::sleep(Duration::new(1, 0));
        }
    }
}
