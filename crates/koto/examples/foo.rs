use std::ops::Add;
use koto::{derive::*, prelude::*, Result};
use koto_runtime::KValue::Str;



#[macro_export]
macro_rules! series_arithmetic_op {
    ($self:ident, $rhs:expr, $op:tt) => {
        {
            match $rhs {
                KValue::Object(rhs) if rhs.is_a::<Self>() => {
                    let rhs = rhs.cast::<Self>().unwrap();
                    Ok((*$self $op *rhs).into())
                }
                KValue::Number(n) => {
                    Ok((*$self $op f64::from(n)).into())
                }
                unexpected => {
                    unexpected_type(&format!("a {} or Number", Self::type_static()), unexpected)
                }
            }
        }
    }
}
fn main() {
    let script = "


close = series([2,3,4])
high = series([5,6,7])
print close+high


";
    let mut koto = Koto::default();
   // let a: &mut KMap=koto.prelude();
    koto.prelude()
        .add_fn("series", |ctx| match ctx.args() {
            [KValue::List(n)] => Ok(Series::new(n.clone()).into()),
            unexpected => unexpected_args("|Number|", unexpected),
        });
    // let mut meta = MetaMap::default();
    //
    // meta.insert(MetaKey::Type, "series".into());
    // meta.add_fn(MetaKey::Call, |ctx| match ctx.args() {
    //     [KValue::Number(n)] => Ok(MyType::make_koto_object(*n).into()),
    //     unexpected => unexpected_args("|Number|", unexpected),
    // });
    // a.set_meta_map(Some(meta.into()));


    koto.compile_and_run(script).unwrap();
}

// MyType is a type that we want to use in Koto
//
// The KotoCopy and KotoType traits are automatically derived.
#[derive(Clone, KotoCopy, KotoType)]
pub struct Series {
    current: KValue,
    history:KList,
}

// The KotoEntries trait is implemented by the koto_impl macro,
// generating Koto functions for any impl function tagged with #[koto_method],
// and inserting them into a cached KMap.
#[koto_impl]
impl Series {
    fn new(n: KList) -> KObject {
        // From is available for any type that implements KotoObject
        let my_type = Self{
            current:n.data()[0].clone(),
            history:n.clone()
            //history:vec![2.0,3.0,4.0,5.0]
        };


        KObject::from(my_type)
    }

    // A simple getter function
    #[koto_method]
    fn get(&self) -> Result<KValue> {
        let b=self;
        Ok(KValue::from(b.history.clone()))
    }



    #[koto_method]
    fn leven(&self) -> Result<KValue> {
        let a=KValue::from(self.current.clone());
        Ok(a)
    }

    // A function that returns the object instance as the result
    // #[koto_method]
    // fn set(ctx: MethodContext<Self>) -> Result<KValue> {
    //     match ctx.args {
    //         [KValue::List(n)] => {
    //             ctx.instance_mut()?.history = n.into();
    //             ctx.instance_result()
    //         }
    //         unexpected => unexpected_args("|Number|", unexpected),
    //     }
    // }
}


// impl From<KValue> for usize {
//     fn from(value: KValue) -> Self {
//         match value {
//             KValue::Number(n) => n.into(),
//             _ => panic!("Expected a KValue::Number"),
//         }
//     }
// }


// impl From<KValue> for usize {
//     fn from(value: KValue) -> Self {
//         match value {
//             KValue::Number(knumber) => knumber.into(), // 将 KNumber 转换为 usize
//             // 如果 KValue 还有其他变体，你可以选择如何处理这些情况
//             _ => panic!("KValue does not contain a KNumber"), // 或者处理为默认值
//         }
//     }
// }

impl From<Series> for KValue {
    fn from(value: Series) -> Self {
        let m=value.history;
        Self::List(KList::from(m))

    }
}
impl KotoObject for Series {
    // KotoObject::Display allows mytype to be used with Koto's print function
    fn display(&self, ctx: &mut DisplayContext) -> Result<()> {
        ctx.append(format!("Series({})", "self.current.into()"));
        Ok(())
    }

    fn index(&self, _index: &KValue) -> Result<KValue> {
        //let knumber2: KNumber = _index.into();
        let index:usize=match _index {
            KValue::Number(knumber) => knumber.into(), // 将 KNumber 转换为 usize
            // 如果 KValue 还有其他变体，你可以选择如何处理这些情况
            _ => panic!("KValue does not contain a KNumber"), // 或者处理为默认值
        };

        // match self.history.data().get::<usize>(_index.into()) {
        //     Some(value) => Ok(value.clone()),
        //     None => Ok(self.history.clone()),
        // }
        // let N=_index.into();
         let a= self.history.data()[index].clone();
         Ok(a)
        //unimplemented_error("@index", self.type_string())
    }

    fn add( &self, other: &KValue) -> Result<KValue> {
        let mut rs =self.clone();
        match other {
            Series => {
                let current=match rs.current {
                    KValue::Number(knumber) => knumber, // 将 KNumber 转换为 usize
                    // 如果 KValue 还有其他变体，你可以选择如何处理这些情况
                    _ => panic!("KValue does not contain a KNumber"), // 或者处理为默认值
                };
                let mut result:ValueVec= rs.history
                    .data()
                    .iter()
                    .map(|v|  v.deep_copy())
                    .collect::<Result<_>>()?;
              //  KList::with_data(result).into()

                //let current=KValue::Number(rs.current.into());
                // rs.current = KValue::from(current.add(KNumber::F64(2.0)));
                result.push(KValue::Number(KNumber::F64(3.0)));
                //result.extend(0..4);
                rs.history= KList::with_data(result).into();
                Ok(KValue::from(rs))
            }

        }
        // println!("{:?}",other);
        // Ok(KValue::from(KString::from("adf".to_string())))
        // let index:usize=match other {
        //     KValue::Number(knumber) => {
        //
        //     },
        //     KValue::Series(knumber) => {
        //         self.current = self.current.add(other.current);
        //         self
        //     },
        //     _ => panic!("KValue does not contain a KNumber"), // 或者处理为默认值
        // };
        // self.current = self.current.add(other.current);
        // self
        // series_arithmetic_op!(self, rhs, +)
    }


}
//
//
// impl KotoType for Series {
//     fn type_static() -> &'static str where Self: Sized {
//         todo!()
//     }
//
//     fn type_string(&self) -> KString {
//         todo!()
//     }
// }
//
// impl KotoCopy for Series {
//     fn copy(&self) -> KObject {
//         todo!()
//     }
// }
//
// impl KotoEntries for Series {}
//
// impl KotoObject for Series {
//     // fn get(&self) -> koto_runtime::Result<()> {
//     //
//     //     print!("asdfsadf");
//     //     Ok(())
//     // }
//
//
// }