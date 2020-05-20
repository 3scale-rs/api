use std::error::Error;

mod context;
mod straitjacket;

use ::straitjacket::client::Method;

pub use context::Context;

pub fn parse_call_args<'s>(args: &'s [&str]) -> Result<(Method, &'s str, Option<&'s str>, Option<&'s str>), Box<dyn Error>> {
    let args: &str = args.get(0)
        .ok_or_else::<Box<dyn Error>, _>(|| From::from("not enough parameters"))?;
    let args = args.split_whitespace().collect::<Vec<_>>();
    if args.len() < 2 {
        return Err(From::from("not enough parameters"));
    }

    let method = match args[0] {
        "GET" | "get" => Method::GET,
        "POST" | "post" => Method::POST,
        "PUT" | "put" => Method::PUT,
        "DELETE" | "delete" => Method::DELETE,
        _ => return Err(From::from("unknown method")),
    };
    let path = args[1];
    let qs = args.get(2).and_then(|&i| i.into());
    let body = args.get(3).and_then(|&i| i.into());
    Ok((method, path, qs, body))
}

