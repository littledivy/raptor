//
use deno_core::error::AnyError;
use deno_core::serde_json::json;
use deno_core::FsModuleLoader;
use deno_runtime::permissions::Permissions;
use deno_runtime::web_worker::WebWorker;
use deno_runtime::web_worker::WebWorkerHandle;
use deno_runtime::web_worker::WebWorkerOptions;
use std::convert::Infallible;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World!")))
}

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let module_loader = Rc::new(FsModuleLoader);
    let create_web_worker_cb = Arc::new(|_| {
        todo!("Web workers are not supported in the example");
    });

    let options = WebWorkerOptions {
        apply_source_maps: false,
        args: vec![],
        debug_flag: false,
        unstable: false,
        ca_data: None,
        user_agent: "hello_runtime".to_string(),
        seed: None,
        js_error_create_fn: None,
        create_web_worker_cb,
        attach_inspector: false,
        maybe_inspector_server: None,
        use_deno_namespace: false,
        module_loader,
        runtime_version: "x".to_string(),
        ts_version: "x".to_string(),
        no_color: false,
        get_error_class_fn: None,
    };

    let js_path = Path::new("examples/hello_world.js");

    let main_module = deno_core::resolve_path(&js_path.to_string_lossy())?;
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
    worker.execute_module(&main_module).await?;
    worker.run_event_loop().await?;
 
    // Init request handler
    let make_svc = make_service_fn(move |_conn| {
        let (handle_sender, handle_receiver) =
        std::sync::mpsc::sync_channel::<WebWorkerHandle>(1);

        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        let handle = worker.thread_safe_handle();
        handle_sender.send(handle).unwrap();

        // handle.post_message(Box::new(*b"Startup"));
        async move {
            Ok::<_, Infallible>(service_fn(move |request: Request<Body>| {
                let req = json!({
                    "body": "Sample"
                });
                let h = handle_receiver.recv().unwrap();
                let r = h.post_message(req.to_string().into_boxed_str().into_boxed_bytes());
                assert!(r.is_ok());
                async move {
                    h.get_event().await.unwrap();
                    hello(request).await
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
