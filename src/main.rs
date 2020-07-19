use iron::{Iron, Request, Response, IronResult, status};
use iron::prelude::Chain;
use router::{Router};
use router::router;
use mount::Mount;

fn main() {
    println!("Hello, world!");    

    let chain = Chain::new(health_routes());

    let mut mount = Mount::new();
    mount.mount("/", chain);


    // create the handler and bind it it to a port    
    println!("Run :: http://localhost:3000/healthz");

    // Much like with other rust stuff when accessing outside
    // bind to 0.0.0.0
    Iron::new(mount).http("0.0.0.0:3000");
    
}

pub fn health(_: &mut Request) -> IronResult<Response> {
    println!("Health Check ...");
    Ok(Response::with((status::Ok, "OK")))
}

fn health_routes() -> Router {
    router!(
        health: get "/healthz" => health
    )
}