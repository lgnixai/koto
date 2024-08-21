use koto::prelude::*;

fn main() {
    let script = "
import ta

print ta.echo( 'Hello')

";
    let mut koto = Koto::default();
    koto.prelude().insert("ta", make_module());
    koto.compile_and_run(script).unwrap();
}

fn make_module() -> KMap {
    // The `KMap::with_type` initializer sets up an empty map with a `@type` entry.
    let module = KMap::with_type("ta");

    module.add_fn("echo", |ctx| match ctx.args() {
        [KValue::Str(s)] => Ok(format!("{s}!").into()),
        unexpected => unexpected_args("|String|", unexpected),
    });

    module.add_fn("square", |ctx| match ctx.args() {
        [KValue::Number(n)] => Ok((n * n).into()),
        unexpected => unexpected_args("|Number|", unexpected),
    });

    module
}
