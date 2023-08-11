# liteapi

## Overview

liteapi is a powerful and easy to use service of [litestorm](https://github.com/Kleinmarb/litestorm) to create REST APIs.
Designed with performance in mind, liteapi is highly multithreaded and asynchronous, allowing you to build fast and responsive APIs with ease.
With its user-friendly interface and intuitive design, liteapi makes it easy to get started building your own APIs. Whether you’re an experienced developer or just getting started, liteapi provides the tools you need to create powerful and scalable REST APIs.

## Features

liteapi gives you a lot of flexibility by having to return a http-response with the given enum and taking the query-params as an argument,
this sounds redundant but by doing that you can give the handler function way more flexibility than in other frameworks.
Let's make a simple REST API using liteapi that asks for an apikey as a query-param (don't ask for an apikey as a query-param in production):

``` rust
use liteapi::{LiteAPI, http, entry, json, json2string};

entry! {
    LiteAPI::new().await
        .get("/", index).await
        .run().await;
}

// Note that every handler has to take query-pairs as the one and only argument.
// Also note that the handler can't be asynchronous.
fn index(q: http::QueryParams) -> http::Response {
    match q.get("apikey") {
        None => {
            http::Response::Plain(http::StatusCode::Forbidden.detail("Please provide an apikey!"))
        },

        Some(apikey) => {
            let secret_apikey = "1234"; // Please dont do this in production
            if apikey == secret_apikey {
                let json = json!({"Hello, ": "World!"});
                http::Response::Json(json2string(&json).unwrap())
                // Also note that there is a json2string macro with which you can turn anything into json in liteapi
            } else {
                http::Response::Plain(http::StatusCode::Forbidden.detail("The apikey is wrong!"))
            }
        }
    }
}


// That's it this api will wait for a request on the default path and give a Forbidden if the apikey is not provided or wrong
```


Let's add another route to this RESTAPI this time we are going to return some html!

```rust
use liteapi::{LiteAPI, http, entry, html2string, json, json2string};

entry! {
    LiteAPI::new().await
        .get("/", index).await
        .get("/template", template).await // Dont forget to add the route here
        .run().await;
}

// Note that every handler has to take query-pairs as the one and only argument.
// Also note that the handler can't be asynchronous.
fn index(query: http::QueryParams) -> http::Response {
    match query.get("apikey") {
        None => {
            http::Response::Plain(http::StatusCode::Forbidden.detail("Please provide an apikey!"))
        },

        Some(apikey) => {
            let secret_apikey = "1234"; // Please dont do this in production
            if apikey == secret_apikey {
                let json = json!({"Hello, ": "World!"});
                http::Response::Json(json2string(&json).unwrap())
                // Also note that there is a json2string macro with which you can turn anything into json in liteapi
            } else {
                http::Response::Plain(http::StatusCode::Forbidden.detail("The apikey is wrong!"))
            }
        }
    }
}

// Here you can see even though we are not working with the query-pairs we have to take them as an argument
fn template(_: http::QueryParams) -> http::Response {
    let html = html2string(r"path\to\template.html").expect("Error reading the html!");
    http::Response::Html(html)
}

// This was very easy right? Imagine the possibilities combining these two routes
```

Here are some more features, that are going to follow soon:

- liteapi already provides auto OpenAPI documentation just visit localhost:7878/openapi.json/ but it is not customizable yet
- support for http2.0 and http3.0
- support for custom middleware
- support for oauth2 
- support for websockets
- support for positional query params: localhost:7878/{id}/{name}/
