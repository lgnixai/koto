use std::thread::sleep;
use std::time::Duration;
use rhai::{Dynamic, Scope};
use koto::{derive::*, prelude::*, Result};
use koto_runtime::KSeries;

fn main() {
    let script = "
import ta
let c=ta.sma(close,2)
#print type c

#print close.get(0)
while true
    print c
    sleep(2000)

#print scope

";

    let mut koto = Koto::default();

    let  mut close={
        let mut values = vec![];

        // 使用 for 循环将 0 到 10 的值添加到 values 向量中
        for i in 0..=3 {
            values.push(KValue::Number(KNumber::F64(i as f64)));
        }
        let s=KSeries::from(values);
        // println!("{:?}",s);
         s
    };

    koto.prelude().insert("close", close.clone());
    //koto.prelude().insert("scope", KValue::from(scope));

    koto.prelude()
        .add_fn("sleep", |ctx| match ctx.args() {
            [KValue::Number(n)] => {
                //close.insert(KValue::Number(KNumber::F64(10 as f64)));
                sleep(Duration::from_millis(3000));
                Ok(().into())
            },
            unexpected => unexpected_args("|Number|", unexpected),
        });

    //我需要在这里继续操作 close 给 close push  值 如何操作
    koto.prelude().insert("ta", koto_talib::make_module());


    koto.compile_and_run(script).unwrap();

    for i in 3..=50 {
        close.insert(KValue::Number(KNumber::F64(i as f64)));
        sleep(Duration::from_millis(1000));
        print!("234234");
       // koto.compile_and_run(script).unwrap();

    }
    // Continue to manipulate the close series
    // For example, adding more values
   // koto.prelude().update_series("close") ;


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
