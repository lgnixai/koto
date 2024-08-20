use crate::{prelude::*, Borrow, BorrowMut, Error, PtrMut, Result};
use indexmap::{Equivalent, IndexMap};
use rustc_hash::FxHasher;
use std::{
    hash::{BuildHasherDefault, Hash},
    ops::{Deref, DerefMut, RangeBounds},
};
use std::fmt::Debug;
use std::marker::PhantomData;
use smallvec::SmallVec;
use crate::KValue::List;


/// The core hashmap value type used in Koto, containing a [ValueMap] and a [MetaMap]

pub type ValueVec = smallvec::SmallVec<[KValue; 4]>;

#[derive(Clone, Debug)]
pub struct KSeries {
    current:String,
    history: PtrMut<ValueVec>,
}

impl  Default for KSeries {
    fn default() -> Self {
        KSeries {
            current: String::from(""),
            history: ValueVec::new().into(),  // `Vec` 默认是空的
        }
    }
}
impl  KSeries  {

    /// Creates an empty KMap
    pub fn new() -> Self {
        Self::default()
    }
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
    pub fn current(&self) -> String {
        self.current.to_string()
    }

    /// Creates a KMap initialized with the provided data
    pub fn with_data(data: ValueVec) -> Self {
        Self::with_contents(data, "".to_string())
    }

    /// Creates a KMap initialized with the provided data and meta map
    pub fn with_contents(data: ValueVec, current: String) -> Self {
        Self {
            history: data.into(),
            current: current,
        }
    }
    // fn index(&self, _index: &KValue) -> Result<KValue> {
    //     //let knumber2: KNumber = _index.into();
    //     let index: usize = match _index {
    //         KValue::Number(knumber) => knumber.into(), // 将 KNumber 转换为 usize
    //         // 如果 KValue 还有其他变体，你可以选择如何处理这些情况
    //         _ => panic!("KValue does not contain a KNumber"), // 或者处理为默认值
    //     };
    //
    //     // match self.history.data().get::<usize>(_index.into()) {
    //     //     Some(value) => Ok(value.clone()),
    //     //     None => Ok(self.history.clone()),
    //     // }
    //     // let N=_index.into();
    //     let a = self.data()[index].clone();
    //     Ok(a)
    //     //unimplemented_error("@index", self.type_string())
    // }
    fn index(&self, _index: &KValue) -> Result<KValue> {
        todo!("adsfsdf")
    }
    // fn add(&self, _index: &KValue) -> Result<KValue> {
    //
    //     crate::types::object::unimplemented_error("@index", self.type_string())
    // }


}

impl KotoType for KSeries {
    fn type_static() -> &'static str where Self: Sized {
        todo!()
    }

    fn type_string(&self) -> KString {
        todo!()
    }
}

impl KotoCopy for KSeries {
    fn copy(&self) -> KObject {
        todo!()
    }
}

impl KotoEntries for KSeries {}

impl KotoObject for KSeries {

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

impl From<Vec<KValue>> for KSeries {
    fn from(vec: Vec<KValue>) -> Self {
        KSeries::with_data(vec.into())
    }
}

// impl From<KList> for KSeries {
//     fn from(vec: KList) -> Self {
//
//         let value=vec.iter().cloned().collect::<crate::ValueVec>().into();
//         KSeries::with_data(value)
//     }
// }


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
// impl From<Vec> for KSeries {
//     fn from(mut vec: Vec) -> Self {
//         let current = vec.len().to_string();
//         let small_vec: SmallVec<[KValue; 4]> = SmallVec::from_vec(vec);
//         let history = small_vec;
//         //let history:PtrMut<ValueVec>=PtrMut::from(data);
//         KSeries {
//             current,
//             history,
//
//         }
//     }
// }
// impl From<KList> for KSeries{
//     fn from(mut vec: KList) -> Self {
//         let current = vec.data_mut().last();
//         let history = vec;
//
//         KSeries {
//             current,
//             history,
//         }
//     }
// }

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn get_and_remove_with_string() {
//         let m = KSeries::default();
//
//         //pub type ValueVec = smallvec::SmallVec<[KValue; 4]>;
//         //
//         // let mut small_vec:ValueVec = SmallVec::new();
//         //
//
//         let small_vec: Vec<KValue> =vec![
//
//             KValue::Number(KNumber::I64(2)).into()
//
//
//         ];
//         println!("{:?}",KValue::Bool(true).into());
//
//
//         let list = KList::with_data(crate::ValueVec::from_vec(small_vec));
//
//         let data_vec: Vec<KValue> = list.data_mut().to_vec();
//
//        // let data_smallvec: ValueVec = data_vec.into(); // Convert Vec<KValue> to SmallVec<[KValue; 4]>
//         println!("{:?}",data_vec);
//         //println!("list data {:?}",list);
//         let b=KSeries::from(list);
//         println!("{:#?}",b);
//         //
//         // assert!(m.get("test").is_none());
//         // m.insert("test", KValue::Null);
//         // assert!(m.get("test").is_some());
//         // assert!(matches!(
//         //     m.data_mut().shift_remove("test"),
//         //     Some(KValue::Null)
//         // ));
//         // assert!(m.get("test").is_none());
//     }
// }
