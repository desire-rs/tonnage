use crate::config::{EXEMPT_ROUTES, JWT_SECRET};
use crate::schema::{Claims, TokenData};
use desire::http;
use desire::Error;
use desire::IntoResponse;
use desire::Middleware;
use desire::Request;
use desire::Result;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::time::Instant;

pub struct Logger;

#[async_trait::async_trait]
impl Middleware for Logger {
  async fn handle(&self, req: Request, next: desire::Next<'_>) -> Result {
    let start = Instant::now();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let res = next.run(req).await?;
    println!(
      "{} {} {} {}ms",
      method,
      path,
      res.status().to_string(),
      start.elapsed().as_millis()
    );
    Ok(res)
  }
}

pub struct Auth;

#[async_trait::async_trait]
impl Middleware for Auth {
  async fn handle(&self, mut req: Request, next: desire::Next<'_>) -> Result {
    let headers = req.inner.headers();
    let uri = req.uri().to_string();
    let path = req.path();
    let mut skip = false;
    if EXEMPT_ROUTES.contains(&path) {
      skip = true;
    } else {
      for value in EXEMPT_ROUTES {
        if value == &"/" {
          continue;
        }
        if path.starts_with(value) {
          skip = true;
        }
      }
    }

    if skip {
      next.run(req).await
    } else {
      if let Some(token) = headers.get(http::header::AUTHORIZATION) {
        let token = String::from_utf8_lossy(token.as_bytes());
        let token = token.replace("Bearer ", "");
        let token = decode::<Claims>(
          token.as_str(),
          &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
          &Validation::default(),
        )
        .map_err(|e| Error::Message { msg: e.to_string() })?;
        let payload: TokenData = TokenData {
          uid: token.claims.sub,
        };
        req.inner.extensions_mut().insert(payload);
        next.run(req).await
      } else {
        Error::Message {
          msg: format!("uri {} auth required", uri),
        }
        .into_response()
      }
    }
  }
}
