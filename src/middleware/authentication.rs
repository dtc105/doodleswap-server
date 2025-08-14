use actix_web::{
    Error, HttpMessage, HttpResponse,
    body::{BoxBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    http::header,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{Algorithm::HS256, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::future::{Ready, ready};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i32,
    pub iat: i64,
    pub exp: i64,
}

pub struct AuthenticationMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthenticationMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static + MessageBody,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddlewareService { service }))
    }
}

pub struct AuthenticationMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static + MessageBody,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let secret: String =
            std::env::var("JWT_SECRET").expect("`JWT_SECRET` must be defined in `.env`");
        let token: Option<String> = req.cookie("Authorization").map(|c| c.value().to_owned());

        let claims = match token {
            Some(t) => match decode::<Claims>(
                t.as_str(),
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::new(HS256),
            ) {
                Ok(data) => data.claims,
                Err(_) => {
                    return Box::pin(async move {
                        Ok(req.into_response(
                            HttpResponse::Unauthorized()
                                .content_type(header::ContentType::json())
                                .json(json!({"error": "Unauthorized", "message": "Must login."}))
                                .map_into_boxed_body(),
                        ))
                    });
                }
            },
            None => {
                return Box::pin(async {
                    Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .content_type(header::ContentType::json())
                            .json(json!({"error": "Unauthorized", "message": "Must login."}))
                            .map_into_boxed_body(),
                    ))
                });
            }
        };

        req.extensions_mut().insert(claims);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?.map_into_boxed_body();
            Ok(res)
        })
    }
}
