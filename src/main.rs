#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod helpers;
mod http;
mod evaluator;
use crate::helpers::point::Point;
use crate::http::{routes };
use crate::evaluator::JavascriptEngine;
// use crate::evaluator::JavascriptEngine;


fn main() {
    let js_engine = JavascriptEngine::new("foo");
    //let script = "$$.core.print('zzz\\n');'Hello ' + $$.foo;";
    let script = "'Hello zzz'";

    let result = js_engine.execute(script).result;
    println!("{}",result);

    let point = Point::new( 1,2 );
    println!("Point :: {}",point.to_string());

    let rocket_instance = rocket::ignite();
    rocket_instance.mount("/", routes![routes::index,
        routes::point,
        routes::hello,
        routes::js_example1
    ]).launch();

}
