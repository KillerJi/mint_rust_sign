mod app_data;
mod entity;
mod error;
mod handlers;

use std::io::Write;

use actix_web::{middleware::Logger, web, App, HttpServer};
use app_data::AppData;
use chrono::Local;
use env_logger::fmt::Color;
use handlers::Handlers;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志显示格式
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let mut style = buf.style();
            let prefix = style.set_color(Color::Black).set_intense(true).value("[");
            let mut style = buf.style();
            let suffix = style.set_color(Color::Black).set_intense(true).value("]");
            writeln!(
                buf,
                "{}{} {:<5} {}{} {}",
                prefix,
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                buf.default_styled_level(record.level()),
                record.module_path().unwrap_or_default(),
                suffix,
                record.args()
            )
        })
        .init();

    // let private_key = "7efe2ed0866b6b7a91699712e4cfa0cd343d825064e56cc5e3c2bf46bc9c6cc8".parse()?;
    // let private_key = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;
    let private_key = "13962cc606545b8a706ee4fad4ccf6cfd21add41e24f4c9abd667ceeaa0a74aa".parse()?;

    let app_data = AppData::new(private_key);
    let app_data = web::Data::new(app_data);
    // 启动http服务
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(Handlers::app_config)
            .wrap(Logger::default())
    })
    .workers(num_cpus::get())
    .bind("0.0.0.0:8888")?
    .run()
    .await
    .map_err(|e| e.into())
}
