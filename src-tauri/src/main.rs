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

struct AppState {
    producer: Mutex<Producer>,
    hosts: RwLock<String>,
    topic: RwLock<String>,
    cancel: Arc<RwLock<bool>>,
}

#[tauri::command]
async fn start_consumer(
    window: Window,
    state: State<'_, AppState>,
    topic: String,
) -> Result<(), String> {
    let mut consumer: Consumer;
    match Consumer::from_hosts(vec![state.hosts.read().unwrap().clone()])
        .with_group("test-group".to_owned())
        .with_topic(topic.clone())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(Some(kafka::consumer::GroupOffsetStorage::Kafka))
        .create()
    {
        Ok(c) => consumer = c,
        Err(e) => return Err(e.to_string()),
    }
    let cancel = state.cancel.clone();
    *state.cancel.write().unwrap() = false;
    *state.topic.write().unwrap() = topic;
    thread::spawn(move || {
        loop {
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
        }
        println!("consumer thread finished");
    });
    Ok(())
}

#[tauri::command]
fn stop_consumer(state: State<'_, AppState>) {
    *state.cancel.write().unwrap() = true;
}

#[tauri::command]
fn produce(state: State<'_, AppState>, message: String) {
    let topic = state.topic.read().unwrap().clone();
    let record = Record::from_value(topic.as_str(), message.as_str());
    state.producer.lock().unwrap().send(&record).unwrap();
}

#[tauri::command]
fn set_broker(state: State<'_, AppState>, broker: String) -> Result<(), String> {
    state.hosts.write().unwrap().clone_from(&broker);
    let mut producer = state.producer.lock().unwrap();
    if let Ok(new_producer) = Producer::from_hosts(vec![broker]).create() {
        *producer = new_producer;
        Ok(())
    } else {
        Err("Failed to create producer".to_string())
    }
}

#[tauri::command]
fn get_broker(state: State<'_, AppState>) -> Result<String, String> {
    Ok(state.hosts.read().unwrap().clone())
}

fn main() {
    let hosts = RwLock::from("localhost:9092".to_string());
    let producer = Mutex::from(
        Producer::from_hosts(vec![hosts.read().unwrap().clone()])
            .create()
            .unwrap(),
    );
    tauri::Builder::default()
        .manage(AppState {
            producer,
            hosts,
            topic: RwLock::from("".to_string()),
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
        .invoke_handler(generate_handler![
            start_consumer,
            stop_consumer,
            produce,
            set_broker,
            get_broker
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
