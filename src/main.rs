// use wasmer::FunctionEnv;
use wasmer::{imports, Instance, Module, Store, Value};

fn eval_program(wast: &str) -> anyhow::Result<i32> {
    let mut store = Store::default();
    let module = Module::new(&store, &wast)?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let add_one = instance.exports.get_function("add_one")?;
    let result = add_one.call(&mut store, &[Value::I32(42)])?;
    assert_eq!(result[0], Value::I32(43));
    match result[0].i32() {
        Some(r) => Ok(r),
        None => Err(anyhow::anyhow!("empty result")),
    }
}

fn main() -> anyhow::Result<()> {
    let wast = r#"
    (module
      (type $t0 (func (param i32) (result i32)))
      (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
        get_local $p0
        i32.const 1
        i32.add))
    "#;

    let res = eval_program(wast);
    match res {
        Ok(val) => println!("{}", val),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
