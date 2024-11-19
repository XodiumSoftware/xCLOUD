use actix_service::Service;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error,
};
use futures::future::{ok, Ready};
use std::pin::Pin;

/// Middleware for logging requests.
pub struct RequestLogger;

/// Implementation of the `Transform` trait for the `RequestLogger` struct.
impl<S, B> actix_service::Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestLoggerMiddleware { service })
    }
}

/// Middleware for logging requests.
pub struct RequestLoggerMiddleware<S> {
    service: S,
}

/// Implementation of the `Service` trait for the `RequestLoggerMiddleware` struct.
impl<S, B> Service<ServiceRequest> for RequestLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>>>>;

    /// Polls the service to determine if it is ready to process a request.
    ///
    /// # Parameters
    ///
    /// - `ctx` - The context for the service.
    ///
    /// # Returns
    ///
    /// A `Poll` containing a `Result` with the result of the poll.
    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    /// Calls the service to process a request.
    ///
    /// # Parameters
    ///
    /// - `req` - The request to process.
    ///
    /// # Returns
    ///
    /// A future containing the result of the request processing.
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let peer_addr = req.peer_addr();
        let user_agent = req
            .headers()
            .get("User-Agent")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("Unknown");

        log::info!(
            "Request to {} from {:?} with User-Agent: {}",
            req.path(),
            peer_addr,
            user_agent
        );

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
