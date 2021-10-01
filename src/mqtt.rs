use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use paho_mqtt::{AsyncClient, ConnectOptions, CreateOptions, Message};

use crate::temper::TemperDevice;

pub(crate) struct MqttConnection {
    device: Arc<Mutex<TemperDevice>>,
    connection: AsyncClient,
    topic: String,
    frequency: u64,
}

impl MqttConnection {
    pub(crate) fn new<T>(
        device: Arc<Mutex<TemperDevice>>,
        opts: T,
        topic: String,
        frequency: u64,
    ) -> MqttConnection
    where
        T: Into<CreateOptions>,
    {
        let connection = AsyncClient::new(opts).unwrap();
        connection.connect(ConnectOptions::new()).wait().unwrap();
        MqttConnection {
            device,
            connection,
            topic,
            frequency,
        }
    }

    pub(crate) fn send_mqtt(&self) {
        if !self.connection.is_connected() {
            println!("Disconnected from MQTT server! Reconnecting...");
            self.connection.reconnect();
        }

        self.connection.publish(Message::new(
            self.topic.to_owned(),
            format!("{}", self.device.lock().unwrap().get_temp()),
            0,
        ));
    }

    pub(crate) fn run_forever(&self) {
        loop {
            self.send_mqtt();
            std::thread::sleep(Duration::new(self.frequency, 0));
        }
    }
}
