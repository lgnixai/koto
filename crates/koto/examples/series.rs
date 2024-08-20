use koto::{derive::*, prelude::*, Result};

fn main() {
    let script = "
import color

let c=color.named 'yellow'
print type c

";

    let script = "
import series

generator = series('fuck you')
print type generator

print generator.leven()
";
    let mut koto = Koto::default();

    // koto.prelude()
    //     .add_fn("series", |ctx| match ctx.args() {
    //         [KValue::Number(n)] => Ok(MyType::make_koto_object(*n).into()),
    //         [KValue::List(n)] => Ok(MyType::make_series(*n).into()),
    //         unexpected => unexpected_args("|Number|", unexpected),
    //     });
    koto.prelude().insert("series", koto_series::make_module());
    koto.prelude().insert("color", koto_color::make_module());

    koto.compile_and_run(script).unwrap();
}
//
// // MyType is a type that we want to use in Koto
// //
// // The KotoCopy and KotoType traits are automatically derived.
// #[derive(Clone, Copy, KotoCopy, KotoType)]
// struct MyType(KList);
//
// // The KotoEntries trait is implemented by the koto_impl macro,
// // generating Koto functions for any impl function tagged with #[koto_method],
// // and inserting them into a cached KMap.
// #[koto_impl]
// impl MyType {
//     fn make_koto_object(n: KNumber) -> KObject {
//         // From is available for any type that implements KotoObject
//         let my_type = Self(n.into());
//         KObject::from(my_type)
//     }
//
//     fn make_series(n: KList) -> KObject {
//         // From is available for any type that implements KotoObject
//         let my_type = Self(n);
//         KObject::from(my_type)
//     }
//
//
//
//     // A simple getter function
//     #[koto_method]
//     fn get(&self) -> Result<KValue> {
//         Ok(self.0.into())
//     }
//
//     // A function that returns the object instance as the result
//     #[koto_method]
//     fn set(ctx: MethodContext<Self>) -> Result<KValue> {
//         match ctx.args {
//             [KValue::Number(n)] => {
//                 ctx.instance_mut()?.0 = n.into();
//                 ctx.instance_result()
//             }
//             unexpected => unexpected_args("|Number|", unexpected),
//         }
//     }
// }
//
// impl KotoObject for MyType {
//     // KotoObject::Display allows mytype to be used with Koto's print function
//     fn display(&self, ctx: &mut DisplayContext) -> Result<()> {
//         ctx.append(format!("MyType({})", self.0));
//         Ok(())
//     }
// }
