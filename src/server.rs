#![deny(warnings)]

use std::{
    convert::Infallible,
    io::{Error, ErrorKind},
    collections::HashMap,
    net::SocketAddr,
    // future::Future,
};
extern crate alloc;
use alloc::borrow::Cow;

use hyper_101::dice::DiceSet;
use hyper_101::roll::Roll;
use hyper_101::probability::Distribution;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Server};
use http::{Response, StatusCode};

type Query = HashMap<String,String>;
type Output = Result<Body, std::io::Error>;

async fn roll_dice(query: Query) -> Output {
    println!("{:?}", query);
    let dice: DiceSet = query.get("dice").unwrap_or(&"1d6".to_string()).parse()?;
    let s = format!("Your die roll is:\n{}", dice.roll().into_string());
    Ok(Body::from(s))
}

async fn get_distribution(query: Query) -> Output {
    println!("{:?}", query);
    let dice: DiceSet = query.get("dice").unwrap_or(&"1d6".to_string()).parse()?;
    let dist: Distribution = dice.try_into()?;
    Ok(Body::from(Cow::<'static, str>::Owned(dist.into_string())))
}

async fn ignore_favicon(_query: Query) -> Output {
    Ok(Body::empty())
}

async fn say_hello(_query: Query) -> Output {
    Ok(Body::from("Hello, stranger!"))
}

async fn serve(r: Request<Body>) -> Result<Response<Body>, Infallible> {

    let params: HashMap<String, String> = r
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        }).unwrap_or_else(HashMap::new);

    let res = match r.uri().path() {
        "/" | "/hello/" => say_hello(params).await,
        "/roll/" => roll_dice(params).await,
        "/distribution/" | "/probability/" => get_distribution(params).await,
        "/favicon.ico" => ignore_favicon(params).await,
        _ => Err(Error::new(ErrorKind::NotFound, "")),
    };

    // Error Handling
    let response = match res {
        Ok(body) => Response::new(body),
        Err(e) => match e.kind() {
            ErrorKind::InvalidData => {
                let mut resp = Response::new(Body::from(Cow::<'static, str>::Owned(e.into_inner().unwrap().to_string())));
                *resp.status_mut() = StatusCode::BAD_REQUEST;
                resp
            }
            ErrorKind::NotFound => {
                let mut resp = Response::new(Body::empty());
                *resp.status_mut() = StatusCode::NOT_FOUND;
                resp
            },
            _ => Response::new(Body::empty()),
        }
    };
    Ok(response)
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        // In other words, we have a wrapper that returns the Response
        // inside a Result and we pass the wrapper to make_service_fn
        async { Ok::<_, Infallible>(service_fn(serve)) }
    });

    // let addr = ([127, 0, 0, 1], 3000).into();
    // let addr = ([0, 0, 0, 0], 3000).into();
    // let addr = ([10, 28, 4, 121], 8080).into();
    let ip = local_ip_address::local_ip().unwrap();
    let addr = SocketAddr::new(ip, 80);

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
