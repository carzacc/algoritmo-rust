extern crate router;
mod algoritmo;
#[macro_use] extern crate serde_json;
extern crate iron;
extern crate hyper;

use router::Router;
use iron::{Iron, Request, Response, IronResult,status};
use algoritmo::algoritmo::{arrotonda,calcola_algoritmo};
use serde_json::Value;
use std::env;
use hyper::header::{ AccessControlAllowOrigin, ContentType};
use hyper::mime::Mime;
use iron::modifiers::Header;
use iron::mime::{TopLevel,SubLevel};

fn gen_json(_: &mut Request) -> IronResult<Response> {
    let squadre = calcola_algoritmo(20);
    let mut arr: [String; 20] = Default::default();
    let mut i: usize = 0;
    for squadra in squadre.iter()   {
        arr[i] = json!({
                "Squadra": squadra.nomesquadra,
                "Alternativa": arrotonda(squadra.punti),
                "Tradizionale": squadra.punti_trad,
                "Somma": arrotonda(squadra.somma()),
                "Gol Fatti": squadra.golfatti,
                "Gol Subiti": squadra.golsubiti,
                "Vittorie": squadra.vittorie,
                "Pareggi": squadra.pareggi,
                "Sconfitte": squadra.sconfitte
        }).to_string();
        if(i!=19)   {
            arr[i].push_str(", ");
        }
        i+=1;
    }
    let mut out: String = " ".to_owned();
    out.push_str("[");
    for entry in arr.iter()    {
        out.push_str(entry);
    }
    out.push_str("]");
    let content_type = Header(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])));
    let cors = Header(AccessControlAllowOrigin::Any);
    let res = Response::with((status::Ok, out, content_type, cors));
    Ok(res)
}

fn get_server_port() -> u16 {
    env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(3000)
}

fn main() {
    let mut router: Router = Router::new();

    router.get("/", gen_json, "index");
    let port = get_server_port();
    println!("Server avviato sulla porta {}",port);
    Iron::new(router).http(("0.0.0.0", port)).unwrap();
}
