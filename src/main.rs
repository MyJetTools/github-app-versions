use app::AppContext;
use background::DataScannerTimer;
use rust_extensions::MyTimer;
use std::{sync::Arc, time::Duration};
mod app;
mod background;
mod db;
mod flows;
mod github;
mod http_server;
mod scripts;
mod settings;

#[tokio::main]
async fn main() {
    let settings = settings::SettingsReader::new(".app-versions").await;

    let app = Arc::new(AppContext::new(settings).await);

    http_server::start_up::setup_server(&app);

    let mut data_reader_timer =
        MyTimer::new(Duration::from_secs(60 * 3)).set_first_tick_before_delay();

    data_reader_timer.register_timer(
        "ReadDataToCache",
        Arc::new(DataScannerTimer::new(app.clone())),
    );

    data_reader_timer.start(app.states.clone(), my_logger::LOGGER.clone());

    app.states.wait_until_shutdown().await;

    // crate::get_last_release::get_last_release(api_key).await;

    /*
    let slice_iterator = SliceIterator::new(result);

    let mut json_reader: JsonArrayIterator<SliceIterator> = JsonArrayIterator::new(slice_iterator);

    while let Some(item) = json_reader.get_next() {
        let item = item.unwrap();
        println!("----");

        let mut object = item.unwrap_as_object(&json_reader).unwrap();

        while let Some(itm) = object.get_next() {
            let itm = itm.unwrap();

            println!(
                "{}: {}",
                itm.name.as_str(&object).unwrap().as_str(),
                itm.value.as_raw_str(&object).unwrap()
            );
        }
    }
     */
}
