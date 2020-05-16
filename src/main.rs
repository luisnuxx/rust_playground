mod helpers;
mod evaluator;
use crate::helpers::point::Point;

fn main() {
    let script = "$$.core.print('zzz\\n');'Hello ' + $$.foo;";

    let result = evaluator::execute(script).result;
    println!("{}",result);

    let point = Point::new( 1,2 );
    println!("Point :: {}",point.to_string());
}
