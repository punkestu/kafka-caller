#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use kafka::{
    consumer::{Consumer, FetchOffset},
    producer::{Producer, Record},
};
use std::{
    str,
    sync::{Arc, Mutex, RwLock},
    thread,
};
use tauri::{generate_handler, Manager, State, Window};

struct MyState {
    producer: Mutex<Producer>,
    hosts: RwLock<Vec<String>>,
    cancel: Arc<RwLock<bool>>,
}

#[tauri::command]
async fn start_consumer(window: Window, state: State<'_, MyState>) -> Result<(), String> {
    let mut consumer = Consumer::from_hosts(state.hosts.read().unwrap().clone())
        .with_group("test-group".to_owned())
        .with_topic("test-topic".to_owned())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(Some(kafka::consumer::GroupOffsetStorage::Kafka))
        .create()
        .expect("error creating consumer");
    let cancel = state.cancel.clone();
    *state.cancel.write().unwrap() = false;
    thread::spawn(move || loop {
        if cancel.read().unwrap().to_owned() {
            break;
        }
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let _ = window.emit(
                    "kafka_message",
                    str::from_utf8(m.value).unwrap().to_string(),
                );
            }

            consumer.consume_messageset(ms).unwrap();
        }

        consumer.commit_consumed().unwrap();
    });
    Ok(())
}

#[tauri::command]
fn stop_consumer(state: State<'_, MyState>) {
    *state.cancel.write().unwrap() = true;
}

#[tauri::command]
fn produce(state: State<'_, MyState>, message: String) {
    let record = Record::from_value("test-topic", message.as_str());
    state.producer.lock().unwrap().send(&record).unwrap();
}

fn main() {
    let hosts = RwLock::from(vec!["localhost:9092".to_owned()]);
    let producer = Mutex::from(
        Producer::from_hosts(hosts.read().unwrap().clone())
            .create()
            .unwrap(),
    );
    tauri::Builder::default()
        .manage(MyState {
            producer,
            hosts,
            cancel: Arc::new(RwLock::new(false)),
        })
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }

            Ok(())
        })
        .invoke_handler(generate_handler![start_consumer, stop_consumer, produce])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
