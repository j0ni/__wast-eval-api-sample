// use wasmer::FunctionEnv;
use actix_web::{error, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::collections::HashMap;
use wasmer::{imports, Instance, Module, Store, Value};

#[derive(Deserialize)]
struct EvalBody {
    wast: String,
}

async fn http_coeffect(_input: i32) -> i32 {
    let resp = reqwest::get("https://httpbin.org/ip").await;
    match resp {
        Ok(res) => match res.json::<HashMap<String, String>>().await {
            Ok(json_res) => {
                println!("{:?}", json_res);
                1
            }
            Err(e) => {
                println!("{}", e);
                -1
            }
        },
        Err(e) => {
            println!("{}", e);
            -1
        }
    }
}
fn eval_program(wast: &String) -> anyhow::Result<i32> {
    let mut store = Store::default();
    let module = Module::new(&store, &wast)?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let add_one = instance.exports.get_function("add_one")?;
    let result = add_one.call(&mut store, &[Value::I32(42)])?;
    assert_eq!(result[0], Value::I32(43));
    match result[0].i32() {
        Some(r) => Ok(r),
        None => Err(anyhow::anyhow!("empty result")),
    }
}

async fn eval_handler(eval_body: web::Json<EvalBody>) -> impl Responder {
    let res = eval_program(&eval_body.wast);
    match res {
        Ok(val) => format!("{}", val),
        Err(e) => {
            println!("{}", e);
            format!("Error.")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    http_coeffect(11).await;
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default().error_handler(|err, _req| {
            // create custom error response
            println!("{}", err.to_string());
            error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
        });

        App::new().service(
            web::resource("/eval")
                // change json extractor configuration
                .app_data(json_config)
                .route(web::post().to(eval_handler)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
