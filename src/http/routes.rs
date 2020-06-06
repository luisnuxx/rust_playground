use crate::helpers::point::Point;
//use crate::evaluator::JavascriptEngine;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/point/<x>/<y>")]
pub fn point(x: i64, y: i64 ) -> String {
    Point::new(x,y).to_string()
}

#[get("/hello?wave&<name>")]
pub fn hello(name: Option<String>) -> String {
    name.map(|name| format!("Hi, {}!", name))
        .unwrap_or_else(|| "Hello!".into())
}


#[get("/js/<txt>")]
pub fn js_example1(txt: String) -> String {
    assert!(txt.len() > 0);
    //let script = format!("'Hello {}';",txt);
    //println!("Script :: {}", script);
    //JavascriptEngine::new().execute(script.as_str() ).result
    "js".to_string()
}
