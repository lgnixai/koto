use crate::{prelude::*, Borrow, PtrMut, Result};
use indexmap::{Equivalent, IndexMap};
use rustc_hash::FxHasher;
use std::{
    fmt,
    hash::{BuildHasherDefault, Hash},
    ops,
};
use std::fmt::Debug;
use std::marker::PhantomData;
use smallvec::SmallVec;
use koto_memory::Ptr;
use crate::KValue::List;


use std::borrow::BorrowMut;
use std::ops::Add;



macro_rules! impl_arithmetic_op {
    ($trait:ident, $trait_fn:ident, $op:tt) => {
        impl ops::$trait for KSeries {
            type Output = Self;

            fn $trait_fn(self, other: Self) -> Self {
                let mut new_history = ValueVec::new();
                for (a, b) in self.data().iter().zip(other.data().iter()){
                    new_history.push(a.clone());
                }

                //let new_current = current_val.$func(rhs_current_val);

                KSeries {
                   // current: Ptr::new(new_current),
                  history: PtrMut::new(KCell::from(new_history)),

                }.into()


                // Inner{
                //     color: self.0.color $op other.0.color,
                //     alpha: self.0.alpha
                // }.into()
            }
        }

        impl ops::$trait<KValue> for KSeries {
            type Output = Self;

            fn $trait_fn(self, other: KValue) -> Self {
                let mut new_history = ValueVec::new();
                 for a in self.data().iter() {
                    new_history.push(a.clone());
                }

                //let new_current = current_val.$func(rhs_current_val);

                KSeries {
                   // current: Ptr::new(new_current),
                  history: PtrMut::new(KCell::from(new_history)),

                }.into()
            }
        }
    };
}
#[macro_export]
macro_rules! color_arithmetic_op {
    ($self:ident, $rhs:expr, $op:tt) => {
        {
            match $rhs {
                KValue::Object(rhs) if rhs.is_a::<Self>() => {
                   let mut new_history = ValueVec::new();
                     for a in $self.data().iter() {
                        new_history.push(a.clone());
                    }

                    //let new_current = current_val.$func(rhs_current_val);

                   let a= KSeries {
                       // current: Ptr::new(new_current),
                      history: PtrMut::new(KCell::from(new_history)),

                    };
                    Ok((a.into()))
                }
                KValue::Number(n) => {

                     let mut new_history = ValueVec::new();
                     for a in $self.data().iter() {
                        new_history.push(a.clone());
                    }

                    //let new_current = current_val.$func(rhs_current_val);

                   let a= KSeries {
                       // current: Ptr::new(new_current),
                      history: PtrMut::new(KCell::from(new_history)),

                    };
                    Ok((a.into()))

                },
                unexpected => {
                    unexpected_type(&format!("a {} or Number", Self::type_static()), unexpected)
                }
            }
        }
    }
}
//impl_arithmetic_op!(Add, add, +);
//
// macro_rules! impl_series_kvalue_ops {
//     ($trait:ident, $fn:ident, $op:tt) => {
//         impl ops::$trait for KSeries {
//             type Output = KSeries;
//
//             fn $fn(self, other: KSeries) -> KSeries {
//                 use KSeries::*;
//
//                 // match (self, other) {
//                 //     (F64(a), F64(b)) => F64(a $op b),
//                 //     (F64(a), I64(b)) => F64(a $op b as f64),
//                 //     (I64(a), F64(b)) => F64(a as f64 $op b),
//                 //     (I64(a), I64(b)) => I64(a $op b),
//                 // }
//
//                 let mut new_history = ValueVec::new();
//                // let current_val = self.current();
//                // let rhs_current_val = rhs.current();
//
//
//                 for (a, b) in self.data().iter().zip(other.data().iter()) {
//                     new_history.push(a.clone().$op(b.clone()));
//                 }
//
//                 //let new_current = current_val.$func(rhs_current_val);
//
//                 KSeries {
//                    // current: Ptr::new(new_current),
//                   history: PtrMut::new(KCell::from(new_history)),
//
//                 }
//             }
//         }
//
//         impl $trait<KValue> for KSeries {
//             type Output = KSeries;
//
//             fn $fn(self, other: KSeries) -> KSeries {
//                 use KSeries::*;
//                 let mut new_history = ValueVec::new();
//                 //let current_val = self.current();
//                 match other {
//                   KValue::Number(knumber) => {
//                        for a in self.data().iter() {
//                             new_history.push(a.clone().$op(knumber.into()));
//                        }
//
//                   }, // 将 KNumber 转换为 usize
//                   _ => panic!("KValue does not contain a KNumber"), // 或者处理为默认值
//
//
//                 }
//
//
//
//                 ///let new_current = current_val.$func(rhs);
//
//                 KSeries {
//                    // current: Ptr::new(new_current),
//                     history: PtrMut::new(KCell::from(new_history)),
//                 }
//             }
//         }
//
//         // impl ops::$trait for &KNumber {
//         //     type Output = KNumber;
//         //
//         //     fn $fn(self, other: &KNumber) -> KNumber {
//         //         use KNumber::*;
//         //
//         //         match (*self, *other) {
//         //             (F64(a), F64(b)) => F64(a $op b),
//         //             (F64(a), I64(b)) => F64(a $op b as f64),
//         //             (I64(a), F64(b)) => F64(a as f64 $op b),
//         //             (I64(a), I64(b)) => I64(a $op b),
//         //         }
//         //     }
//         // }
//     };
// }
// 实现四则运算宏
// macro_rules! impl_series_ops {
//     ($trait:ident, $func:ident) => {
//         impl $trait for KSeries {
//             type Output = KSeries;
//
//             fn $func(self, rhs: KSeries) -> KSeries {
//                 let mut new_history = ValueVec::new();
//                // let current_val = self.current();
//                // let rhs_current_val = rhs.current();
//
//
//                 for (a, b) in self.data().iter().zip(rhs.data().iter()) {
//                     new_history.push(a.clone().$func(b.clone()));
//                 }
//
//                 //let new_current = current_val.$func(rhs_current_val);
//
//                 KSeries {
//                    // current: Ptr::new(new_current),
//                   history: PtrMut::new(KCell::from(new_history)),
//
//                 }
//             }
//         }
//          impl $trait<KValue> for KSeries {
//             type Output = KSeries;
//
//             fn $func(self, rhs: KValue) -> KSeries {
//                 let mut new_history = ValueVec::new();
//                 //let current_val = self.current();
//                 match rhs {
//                   KValue::Number(knumber) => {
//                        for a in self.data().iter() {
//                             new_history.push(a.clone().$func(knumber.into()));
//                        }
//
//                   }, // 将 KNumber 转换为 usize
//                   _ => panic!("KValue does not contain a KNumber"), // 或者处理为默认值
//
//
//                 }
//
//
//
//                 ///let new_current = current_val.$func(rhs);
//
//                 KSeries {
//                    // current: Ptr::new(new_current),
//                     history: PtrMut::new(KCell::from(new_history)),
//                 }
//             }
//         }
//
//     }
// }


/// The core hashmap value type used in Koto, containing a [ValueMap] and a [MetaMap]

pub type ValueVec = smallvec::SmallVec<[KValue; 4]>;

#[derive(Debug,Clone)]
pub struct KSeries {
    // current: PtrMut<KValue>,
    history: PtrMut<crate::ValueVec>,
}

impl KSeries {
    pub fn new(history: crate::ValueVec) -> Self {
        let v=(KValue::Number(KNumber::I64(0)));

        KSeries {
            //  current: PtrMut::from(v),
            history:  PtrMut::from(KCell::from(history))
        }



    }


    // pub fn current(&self) -> Borrow<KValue> {
    //     self.current.borrow()
    // }

    pub fn len(&self) -> usize {
        self.data().len()
    }
    pub fn size(&self) -> usize {
        self.data().len()
    }

    /// Returns true if there are no entries in the list
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn data(&self) -> Borrow<crate::ValueVec> {
        self.history.borrow()
    }


    pub fn with_data(data: ValueVec) -> Self {
        // let v=(KValue::Number(KNumber::I64(2)));
        // Self::with_contents(data, v)
        Self::with_contents(data)
    }

    /// Creates a KMap initialized with the provided data and meta map
    pub fn with_contents(data: ValueVec ) -> Self {
        Self {
            history: data.into(),

        }
    }
    // pub fn with_contents(data: ValueVec, current: KValue) -> Self {
    //     Self {
    //         history: data.into(),
    //         current: current.into(),
    //     }
    // }

    // 索引实现
    // pub fn get_series(&self, index: usize) -> KSeries {
    //     let history_len = self.data().len();
    //     let mut new_history = crate::ValueVec::new();
    //
    //     for i in 0..history_len {
    //         if i + index < history_len {
    //             new_history.push(
    //                 self.data()
    //                     .as_ref()
    //                     .values[history_len - 1 - i - index]
    //                     .clone(),
    //             );
    //         } else {
    //             new_history.push(KValue::Number(0)); // 不足的部分用 0 填充
    //         }
    //     }
    //
    //     KSeries {
    //         current: Ptr::new(
    //             self.history
    //                 .as_ref()
    //                 .values
    //                 .get(history_len.saturating_sub(index + 1))
    //                 .unwrap_or(&KValue::Number(0))
    //                 .clone(),
    //         ),
    //         history: PtrMut::new(new_history),
    //     }
    // }
}


impl KotoType for KSeries {
    fn type_static() -> &'static str where Self: Sized {
        "Series"
    }

    fn type_string(&self) -> KString {
        KString::from("Series")
    }
}

impl KotoCopy for KSeries {
    fn copy(&self) -> KObject {
        todo!()
    }
}
//
// impl Add for KValue {
//     type Output = KValue;
//
//     fn add(self, rhs: KValue) -> KValue {
//         match (self, rhs) {
//             (KValue::Number(KNumber::I64(a)), KValue::Number(KNumber::I64(b))) => KValue::Number(KNumber::I64(a + b)),
//             // 处理其他情况...
//             _ => panic!("Unsupported operation"),
//         }
//     }
// }
//
// impl Add for KSeries {
//     type Output = KSeries;
//
//     fn add(self, other: KSeries) -> KSeries {
//         let mut new_history = ValueVec::new();
//
//         for (a, b) in self.data().iter().zip(other.data().iter()) {
//             new_history.push(a.clone() + b.clone());
//         }
//
//         KSeries {
//             history: PtrMut::new(KCell::from(new_history)),
//         }
//     }
// }
//
// impl Add<KValue> for KSeries {
//     type Output = KSeries;
//
//     fn add(self, other: KValue) -> KSeries {
//
//
//         println!("adfadfasdf");
//         let mut new_history = ValueVec::new();
//
//         match other {
//             KValue::Number(knumber) => {
//                 for a in self.data().iter() {
//                     new_history.push(a.clone() + knumber.into());
//                 }
//             },
//             _ => panic!("KValue does not contain a KNumber"),
//         }
//
//         KSeries {
//             history: PtrMut::new(KCell::from(new_history)),
//         }
//     }
// }
//
// impl Add<KNumber> for KSeries {
//     type Output = KSeries;
//
//     fn add(self, other: KNumber) -> KSeries {
//         let mut new_history = ValueVec::new();
//
//         for a in self.data().iter() {
//             new_history.push(a.clone() + other.into());
//         }
//
//         KSeries {
//             history: PtrMut::new(KCell::from(new_history)),
//         }
//     }
// }
impl KotoEntries for KSeries {}



#[macro_export]
macro_rules! geometry_arithmetic_op {
    ($self:ident, $rhs:expr, $op:tt) => {
        {
              let mut new_history = ValueVec::new();

                    for a in $self.data().iter() {
                        new_history.push(a.clone() + KValue::Number(KNumber::I64(1)));
                    }

                     let k=KSeries {
                        history: PtrMut::new(KCell::from(new_history)),
                    };
                     let v=KValue::from(k);


                    Ok(v)
        }
    }
}
impl KotoObject for KSeries {
    fn index(&self, index: &KValue) -> Result<KValue> {

        let _index: usize = match index {
            KValue::Number(knumber) => knumber.into(), // 将 KNumber 转换为 usize
            // 如果 KValue 还有其他变体，你可以选择如何处理这些情况
            _ => panic!("KValue does not contain a KNumber"), // 或者处理为默认值
        };
        let a = self.data()[_index].clone();
        Ok(a)

    }
    // fn add(&self, rhs: &KValue) -> Result<KValue> {
    //     let mut new_history = ValueVec::new();
    //
    //     // for a in self.data().iter() {
    //     //     new_history.push(a.clone() + KValue::Number(KNumber::I64(1)));
    //     // }
    //
    //     new_history.push(KValue::Number(KNumber::I64(1)));
    //
    //     let k=KSeries {
    //         history: PtrMut::new(KCell::from(new_history)),
    //     };
    //     let v=KValue::from(k);
    //
    //
    //     Ok(v)
    // }
    fn add(&self, rhs: &KValue) -> Result<KValue> {
        color_arithmetic_op!(self, rhs, +)
    }


    //impl_series_ops!(Add, add);

    fn display(&self, ctx: &mut DisplayContext) -> Result<()> {

        ctx.append("series==");
        ctx.append('[');

        let id = PtrMut::address(&self.history);
        if ctx.is_in_parents(id) {
            ctx.append("...");
        } else {
            ctx.push_container(id);

            for (i, value) in self.data().iter().enumerate() {
                if i > 0 {
                    ctx.append(", ");
                }
                value.display(ctx)?;
            }

            ctx.pop_container();
        }

        ctx.append(']');

        Ok(())
    }
}



impl From<KList> for KSeries {
    fn from(list: KList) -> Self {
        // Convert from Borrow to Vec and then to SmallVec
        let data_vec: Vec<KValue> = list.data_mut().to_vec();

        //println!("data:{:?}",list.data());

        let data_smallvec: ValueVec = data_vec.into(); // Convert Vec<KValue> to SmallVec<[KValue; 4]>
        //println!("{:?}",data_smallvec);
        KSeries::with_data(data_smallvec)
    }
}
// 使用宏生成 Add, Sub, Mul, Div 的实现
//impl_kvalue_series_ops!(Add, add);
//
// macro_rules! impl_series_kvalue_ops {
//     ($trait:ident, $fn:ident, $op:tt) => {
//         impl ops::$trait for KSeries {
//             type Output = KSeries;
//
//             fn $fn(self, other: KSeries) -> KSeries {
//                 let mut new_history = ValueVec::new();
//
//                 for (a, b) in self.data().iter().zip(other.data().iter()) {
//                     new_history.push(a.clone() $op (b.clone()));
//                 }
//
//                 KSeries {
//                     history: PtrMut::new(KCell::from(new_history)),
//                 }
//             }
//         }
//
//         impl ops::$trait<KValue> for KSeries {
//             type Output = KSeries;
//
//             fn $fn(self, other: KValue) -> KSeries {
//                 let mut new_history = ValueVec::new();
//
//                 match other {
//                     KValue::Number(knumber) => {
//                         for a in self.data().iter() {
//                             new_history.push(a.clone() $op (knumber.into()));
//                         }
//                     },
//                     _ => panic!("KValue does not contain a KNumber"),
//                 }
//
//                 KSeries {
//                     history: PtrMut::new(KCell::from(new_history)),
//                 }
//             }
//         }
//     };
// }

// Example usage
//impl_series_kvalue_ops!(Add, add, +);
// impl_series_kvalue_ops!(Sub, sub, -);
// impl_series_kvalue_ops!(Mul, mul, *);
// impl_series_kvalue_ops!(Div, div, /);



impl Add<KValue> for KSeries {
    type Output = KSeries;


    fn add(self, rhs: KValue) -> Self::Output {
        let mut new_history = ValueVec::new();

        // for a in self.data().iter() {
        //     new_history.push(a.clone() + KValue::Number(KNumber::I64(1)));
        // }

        new_history.push(KValue::Number(KNumber::I64(1)));

        KSeries {
            history: PtrMut::new(KCell::from(new_history)),
        }

    }
}
//
// impl Add<KNumber> for KSeries {
//     type Output = KSeries;
//
//
//     fn add(self, rhs: KNumber) -> Self::Output {
//         let mut new_history = ValueVec::new();
//
//         // for a in self.data().iter() {
//         //     new_history.push(a.clone() + KValue::Number(KNumber::I64(1)));
//         // }
//
//         new_history.push(KValue::Number(KNumber::I64(1)));
//
//          KSeries {
//             history: PtrMut::new(KCell::from(new_history)),
//         }
//
//     }
// }


//
#[test]
fn main() {
    // let s1 = KSeries::new(KValue::Number(KNumber::I64(2)));
    // let s2 = KSeries::new(KValue::Number(KNumber::I64(2)));
    //let s1 = KSeries::new(vec![KValue::Number(KNumber::I64(2))]);
    let values: ValueVec = SmallVec::from_vec(vec![
        KValue::Number(KNumber::I64(1)),
        KValue::Number(KNumber::I64(2)),
        KValue::Number(KNumber::I64(3)),
    ]);

    // 使用 ValueVec 创建 KSeries 实例
    let s1 = KSeries::new(values);

    let result = s1 + KValue::Number(KNumber::I64(3)) ;

    println!("{:?}", result);

    // let series_at_1 = &result[1];
    // println!("Series at 1: {:?}", series_at_1);
}
