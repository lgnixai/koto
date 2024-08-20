use koto::prelude::*;
use koto_runtime::KSeries;

fn main() {
    let script = "
let close=[2,3,4]
let b=ta(close)

print type b
print b[0]
";
    let mut koto = Koto::default();
    let prelude = koto.prelude();

    // Standalone functions can be inserted directly
    prelude.insert("ta", ta);

    // The add_fn helper avoids the need for type annotations
    prelude.add_fn("plus", |ctx| match ctx.args() {
        [KValue::Number(a), KValue::Number(b)] => Ok((a + b).into()),
        unexpected => unexpected_args("|Number, Number|", unexpected),
    });

    koto.compile_and_run(script).unwrap();
}

fn ta(ctx: &mut CallContext) -> koto::Result<KValue> {
    match ctx.args() {
        [] => println!("Hello?"),
        [KValue::List(l)] => {
            //println!("{:?}",l);
            let s=KSeries::from(l.clone());
           // println!("{:?}",s);
           return  Ok(s.into())
            //Ok(KSeries::from(l.clone()));
        },
        unexpected => return unexpected_args("||, or |String|", unexpected),
    }

    Ok(KValue::Null)
}
