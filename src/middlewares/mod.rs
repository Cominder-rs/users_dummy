use tonic::body::BoxBody;
use hyper::Body;
use std::task::{Context, Poll};

use tower::{Service, Layer};



#[derive(Debug, Clone)]
pub struct MiddlewareLayer {}

impl <S> Layer<S> for MiddlewareLayer {
    type Service = Middleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        Middleware { inner: service }
    }
}


#[derive(Clone)]
pub struct Middleware<S> {
    inner: S,
}


impl <S> Service<hyper::Request<Body>> for Middleware<S>
where 
    S: Service<hyper::Request<Body>, Response = hyper::Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: hyper::Request<Body>) -> Self::Future {
        let new_uri = req.uri().to_string();
        let new_uri = new_uri.strip_prefix("/api/users_dummy").unwrap();
        *req.uri_mut() = new_uri.parse().unwrap();
        // This is necessary because tonic internally uses `tower::buffer::Buffer`.
        // See https://github.com/tower-rs/tower/issues/547#issuecomment-767629149
        // for details on why this is necessary
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            // Do extra async work here...
            let response = inner.call(req).await?;

            Ok(response)
        })
    }    
}
