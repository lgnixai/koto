use crate::{prelude::*, Borrow,BorrowMut, PtrMut, Result};
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

use std::ops::Add;


/// The core hashmap value type used in Koto, containing a [ValueMap] and a [MetaMap]

pub type ValueVec = smallvec::SmallVec<[KValue; 4]>;

#[derive(Debug, Clone)]
pub struct KSeries {
    // current: PtrMut<KValue>,
    pub history: PtrMut<crate::ValueVec>,
}

impl KSeries {
    pub fn new(history: crate::ValueVec) -> Self {
        let v = (KValue::Number(KNumber::I64(0)));

        KSeries {
            //  current: PtrMut::from(v),
            history: PtrMut::from(KCell::from(history))
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

    pub fn data(&self) -> Borrow<ValueVec> {
        self.history.borrow()
    }
    pub fn data_mut(&self) -> BorrowMut<ValueVec> {
        self.history.borrow_mut()
    }
    pub fn insert(&self,  value: impl Into<KValue>) {
        self.data_mut().push( value.into());
    }
    pub fn with_data(data: ValueVec) -> Self {
        // let v=(KValue::Number(KNumber::I64(2)));
        // Self::with_contents(data, v)
        Self::with_contents(data)
    }

    /// Creates a KMap initialized with the provided data and meta map
    pub fn with_contents(data: ValueVec) -> Self {
        Self {
            history: data.into(),

        }
    }

    fn index_value(&self, index: &KValue) -> Result<KValue> {
        println!("====asfsdf");
        println!("====asfsdf");
        let _index: usize = match index {
            KValue::Number(knumber) =>{

                knumber.into()
            } , // 将 KNumber 转换为 usize
            // 如果 KValue 还有其他变体，你可以选择如何处理这些情况
            _ => panic!("KValue does not contain a KNumber"), // 或者处理为默认值
        };
        //let a=self.get_series_from(_index);
        let a = self.data()[_index].clone();
        Ok(KValue::from(a))
    }

    // 获取一个新的 Series，其中切片从 start 开始，如果长度不够，则补 Null
    pub fn get_series_from(&self, start: usize) -> KSeries {
        let history = self.data();
        let mut new_history = ValueVec::new();

        // If the start index is out of range, return a Series with Null values
        if start >= history.len() {
            new_history.push(KValue::Null); // You can add more Null values if needed
        } else {
            // Copy elements from the start index
            for value in &history[start..] {
                new_history.push(value.clone());
            }

            // Pad with Nulls to match the original history length
            while new_history.len() < history.len() {
                new_history.push(KValue::Null);
            }
        }

        // 创建新的 Series，current 为新的第一个值，history 为新数组
            KSeries {
                history: PtrMut::new(KCell::from(new_history)),
            }
    }


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

impl KotoEntries for KSeries {}

impl KotoObject for KSeries {
    fn index(&self, index: &KValue) -> Result<KValue> {
        println!("====asfsdf");
        let _index: usize = match index {
            KValue::Number(knumber) =>{

               knumber.into()
            } , // 将 KNumber 转换为 usize
            // 如果 KValue 还有其他变体，你可以选择如何处理这些情况
            _ => panic!("KValue does not contain a KNumber"), // 或者处理为默认值
        };
        let a=self.get_series_from(_index);
        //let a = self.data()[_index].clone();
        Ok(KValue::Series(a))
    }

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

impl From<Vec<KValue>> for KSeries {
    fn from(vec: Vec<KValue>) -> Self {
        // 将 Vec<KValue> 转换为 ValueVec (SmallVec<[KValue; 4]>)
        let data_smallvec: ValueVec = vec.into();

        // 使用转换后的数据创建 KSeries
        KSeries::with_data(data_smallvec)
    }
}

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
#[test]
fn main() {
    let values: ValueVec = SmallVec::from_vec(vec![
        KValue::Number(KNumber::I64(1)),
        KValue::Number(KNumber::I64(2)),
        KValue::Number(KNumber::I64(3)),
    ]);

    // 使用 ValueVec 创建 KSeries 实例
    let s1 = KSeries::new(values);

    let result = s1 + KValue::Number(KNumber::I64(3));

    println!("{:?}", result);

    // let series_at_1 = &result[1];
    // println!("Series at 1: {:?}", series_at_1);
}
