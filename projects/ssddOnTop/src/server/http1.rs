/*use hyper::service::service_fn;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::oneshot;

pub async fn run(
    sc: Arc<ServerConfig>,
) -> anyhow::Result<()> {
    let addr = sc.addr();
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Listening on: http://{}", addr);
    loop {
        let stream_result = listener.accept().await;
        match stream_result {
            Ok((stream, _)) => {
                let io = hyper_util::rt::TokioIo::new(stream);
                let sc = sc.clone();
                tokio::spawn(async move {
                    let server = hyper::server::conn::http1::Builder::new()
                        .serve_connection(
                            io,
                            service_fn(move |req| {
                                todo!()
                            }),
                        )
                        .await;
                    if let Err(e) = server {
                        tracing::error!("An error occurred while handling a request: {e}");
                    }
                });
            }
            Err(e) => tracing::error!("An error occurred while handling request: {e}"),
        }
    }
}*/