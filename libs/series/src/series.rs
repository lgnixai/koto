use indexmap::IndexMap;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::sync::Arc;
use koto_runtime::derive::{koto_impl, koto_method};
use koto_runtime::{DisplayContext, KObject, KotoCopy, KotoEntries, KotoObject, KotoType, KString, KValue};
use koto_runtime::{derive::*, prelude::*, Result};

/// A basic Markov chain,
#[derive(Clone, Debug, Default)]
pub struct Series {
    links: Vec<f64>,
}

impl Series {
    pub fn init(source: &str) -> Self {
        println!("iniy");
        let mut links: Vec<f64> = vec![2.0, 3.0, 4.0];
        let a = Self {
            links: links,
        };
        return a
    }

    // pub fn get(&self)->Vec<f64> {
    //
    //    return self.links
    // }


}

impl KotoType for Series {
    fn type_static() -> &'static str where Self: Sized {
        todo!()
    }

    fn type_string(&self) -> KString {
        todo!()
    }
}

impl KotoCopy for Series {
    fn copy(&self) -> KObject {
        todo!()
    }
}

impl KotoEntries for Series {}

impl KotoObject for Series {
    // fn get(&self) -> koto_runtime::Result<()> {
    //
    //     print!("asdfsadf");
    //     Ok(())
    // }


}