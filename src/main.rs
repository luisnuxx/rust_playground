
mod evaluator;

fn main() {
    let script = "$$.core.print('zzz\\n');'Hello ' + $$.foo;";

    let result = evaluator::execute(script).result;
    println!("{}",result);

}
