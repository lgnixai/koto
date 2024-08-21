use rhai::{Dynamic, Scope};
use koto::{derive::*, prelude::*, Result};
use koto_runtime::KSeries;

fn main() {
    let script = "
import ta
let c=ta.sma(close,2)
print type c

print close
print c
#print scope

";

    let mut koto = Koto::default();

    let mut close={
        let mut values = vec![];

        // 使用 for 循环将 0 到 10 的值添加到 values 向量中
        for i in 0..=10 {
            values.push(KValue::Number(KNumber::F64(i as f64)));
        }
        let s=KSeries::from(values);
        // println!("{:?}",s);
         s
    };
//     let mut scope = Scope::new();
// // 将 close 序列添加到 scope 中
//     let mut high: Vec<f64> = vec![100.5, 101.0, 102.3, 103.8];
//
//     // 将 close 序列添加到 scope 中
//     scope.push("high", high.clone());
//     let new_close_value = 104.7;
//     high.push(new_close_value);
//
//     // 更新 scope 中的 close 序列
//     scope.set_value("high", high.clone());
//     let a = scope.get_value::<Dynamic>("high");
//     if let Some(s) = a {
//         // 尝试将 Dynamic 转换为 Vec<f64>
//         if let high_vec = s.clone().try_cast::<Vec<f64>>() {
//             // 遍历并打印 high 序列中的值
//             for value in high_vec.unwrap().iter() {
//                 println!("high: {}", value);
//             }
//         } else {
//             println!("Failed to cast Dynamic to Vec<f64>");
//         }
//     } else {
//         println!("No value found in scope for 'high'");
//     }
    //println!("high:{:?}",scope.get_value::<Dynamic>("high"));
    //close.insert(KValue::Number(KNumber::F64(11.0)));
    koto.prelude().insert("close", close);
    //koto.prelude().insert("scope", KValue::from(scope));

    // Continue to manipulate the close series
    // For example, adding more values
    // if let Some(close_series) = koto.prelude().data_mut("close") {
    //     for i in 12..=15 {
    //         close_series.push(KValue::Number(KNumber::F64(i as f64)));
    //     }
    // }


    //我需要在这里继续操作 close 给 close push  值 如何操作
    koto.prelude().insert("ta", koto_talib::make_module());


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
