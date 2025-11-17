use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

pub struct AvilaAuth;

impl<S, B> Transform<S, ServiceRequest> for AvilaAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AvilaAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AvilaAuthMiddleware { service }))
    }
}

pub struct AvilaAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AvilaAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Injeta o token Avila em todas as requisições
        if let Some(state) = req.app_data::<actix_web::web::Data<crate::state::AppState>>() {
            let token = state.avila_token.clone();
            req.extensions_mut().insert(token);
        }
        
        let fut = self.service.call(req);
        
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
