#![allow(dead_code)]

use rusty_v8 as v8;




pub struct JavascriptEngine {
    name: String
}
pub struct ExecutionResult {
    pub result: String
}


impl JavascriptEngine {
    pub fn new(name_p : &str) -> Self {

        Self {
           name:name_p.to_string()
        }
    }

    pub fn execute(&self, script_to_execute: &str) -> ExecutionResult {
        let platform =  v8::new_default_platform().unwrap();
        v8::V8::initialize_platform( platform );
        v8::V8::initialize();
        let mut isolate = v8::Isolate::new(Default::default());
        println!("Evaluator::execute::script_to_execute:: [{}]", script_to_execute);
        let mut handle_scope = v8::HandleScope::new( &mut isolate);
        let scope = handle_scope.enter();

        let context = v8::Context::new(scope);
        let mut context_scope = v8::ContextScope::new(scope, context);
        let scope = context_scope.enter();

        let global: v8::Local<v8::Object> = context.global(scope);

        let double_dollar = v8::Object::new(scope);
        global.set(
            context,
            v8::String::new(scope, "$$").unwrap().into(),
            double_dollar.into(),
        );

        let foo = v8::String::new(scope,"world!!");
        double_dollar.set(
            context,
            v8::String::new(scope, "foo").unwrap().into(),
            foo.unwrap().into(),
        );

        let foo = v8::String::new(scope,"world!!");
        double_dollar.set(
            context,
            v8::String::new(scope, "foo").unwrap().into(),
            foo.unwrap().into(),
        );


        let core_val = v8::Object::new(scope);
        double_dollar.set(
            context,
            v8::String::new(scope, "core").unwrap().into(),
            core_val.into(),
        );

        let mut print_tmpl = v8::FunctionTemplate::new(scope, print);
        let print_val = print_tmpl.get_function(scope, context).unwrap();

        core_val.set(
            context,
            v8::String::new(scope, "print").unwrap().into(),
            print_val.into(),
        );

        //"'Hello ' + foo; "
        let code = v8::String::new(scope,script_to_execute ).unwrap();
        println!("javascript code: {}", code.to_rust_string_lossy(scope));

        let mut script = v8::Script::compile(scope, context, code, None).unwrap();
        let raw_result = script.run(scope, context);
        match raw_result {
            Some(result) => {
                let result = result.to_string(scope).unwrap();
                // println!("result: {}", result.to_rust_string_lossy(scope));


                ExecutionResult {
                    result: result.to_rust_string_lossy(scope)
                }
            }
            None => {
                ExecutionResult {
                    result: "ERROR".to_string()
                }
            }
        }

    }


}



fn print(
    scope: v8::FunctionCallbackScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let arg_len = args.length();
    assert!(arg_len >= 0 && arg_len <= 2);

    let obj = args.get(0);
    let is_err_arg = args.get(1);

    let mut hs = v8::HandleScope::new(scope);
    let scope = hs.enter();

    let mut is_err = false;
    if arg_len == 2 {
        let int_val = is_err_arg
            .integer_value(scope)
            .expect("Unable to convert to integer");
        is_err = int_val != 0;
    };
    let mut try_catch = v8::TryCatch::new(scope);
    let _tc = try_catch.enter();
    let str_ = match obj.to_string(scope) {
        Some(s) => s,
        None => v8::String::new(scope, "").unwrap(),
    };
    if is_err {
        eprint!("{}", str_.to_rust_string_lossy(scope));
    } else {
        print!("{}", str_.to_rust_string_lossy(scope));
    }
}

fn fortytwo_callback(
    scope: v8::FunctionCallbackScope,
    _: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    rv.set(v8::Integer::new(scope, 42).into());
}