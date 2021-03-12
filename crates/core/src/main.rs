//
use deno_core::error::AnyError;
use deno_core::serde_json::json;
use deno_core::FsModuleLoader;
use deno_runtime::permissions::Permissions;
use deno_runtime::web_worker::WebWorker;
use deno_runtime::web_worker::WebWorkerHandle;
use deno_runtime::web_worker::WebWorkerOptions;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::Infallible;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use url::Url;
pub fn block_run<F, R>(future: F) -> R
where
    F: std::future::Future<Output = R>,
{
    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .max_blocking_threads(32)
        .build()
        .unwrap()
        .block_on(future)
}

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    // Init request handler
    let make_svc = make_service_fn(move |_conn| {
        let (handle_sender, handle_receiver) = std::sync::mpsc::sync_channel::<WebWorkerHandle>(1);
        std::thread::spawn(move || {
            let module_loader = Rc::new(FsModuleLoader);
            let create_web_worker_cb = Arc::new(|_| {
                todo!("Not supported at the moment.");
            });

            let options = WebWorkerOptions {
                apply_source_maps: false,
                args: vec![],
                debug_flag: false,
                unstable: false,
                ca_data: None,
                user_agent: "RaptorDeno".to_string(),
                seed: None,
                js_error_create_fn: None,
                create_web_worker_cb,
                attach_inspector: false,
                maybe_inspector_server: None,
                use_deno_namespace: false,
                module_loader,
                runtime_version: "1.8.0".to_string(),
                ts_version: "xxx".to_string(),
                no_color: false,
                get_error_class_fn: None,
            };

            let js_path = Path::new("examples/hello_world.js");

            let main_module = deno_core::resolve_path(&js_path.to_string_lossy()).unwrap();
            let permissions = Permissions::allow_all();
            let mut worker = WebWorker::from_options(
                "worker".to_string(),
                permissions,
                main_module.clone(),
                1,
                &options,
            );

            // Prepare runtime.
            worker.bootstrap(&options);
            block_run(worker.execute_module(&main_module));
            let handle = worker.thread_safe_handle();
            handle_sender.send(handle).unwrap();
            block_run(worker.run_event_loop());
        });

        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function tht
        // returns a Response into a `Service`.
        async move {
            Ok::<_, Infallible>(service_fn(move |request: Request<Body>| {
                let h = handle_receiver.recv().unwrap();
                async move {
                    let query = Url::parse("https://HOST/")
                        .unwrap()
                        .join(&request.uri().to_string())
                        .unwrap()
                        .query_pairs()
                        .into_owned()
                        .collect::<HashMap<String, String>>();
                    let headers = request
                        .headers()
                        .iter()
                        .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap().to_string()))
                        .collect::<HashMap<String, String>>();
                    let body = hyper::body::to_bytes(request.into_body()).await;
                    let req = json!({
                        "body": body.unwrap().to_vec(),
                        "query": query,
                        "headers": headers,
                    });

                    let r = h.post_message(req.to_string().into_boxed_str().into_boxed_bytes());
                    assert!(r.is_ok());

                    h.get_event().await.unwrap();
                    Ok::<Response<Body>, Infallible>(Response::new(Body::from("Hello World!")))
                }
            }))
        }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);

    // Start server
    server.await?;

    Ok(())
}
