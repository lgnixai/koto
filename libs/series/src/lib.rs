//! A Koto language module for working with colors

mod series;

use std::result;


use koto_runtime::{prelude::*, Result};
use palette::{Hsl, Hsv};
use koto_runtime::derive::{KotoCopy, KotoType};
use koto_runtime::KValue::List;
use crate::series::Series;

pub fn make_module() -> KMap {
    use KValue::{Number, Str};
    //let mut result = KMap::default();
    //

    // result

    let mut result  = KMap::with_type("series");

    result.add_fn("new", {
        |ctx| match ctx.args() {
            [KValue::Str(text)] => {
                let mut poetry = Series::default();
               // poetry.add_source_material(text);

                Ok(KObject::from(KotoSeries(poetry)).into())
            }
            unexpected => unexpected_args("|String|", unexpected),
        }
    });

    let mut meta = MetaMap::default();

    meta.insert(MetaKey::Type, "series".into());
    meta.add_fn(MetaKey::Call, |ctx| match ctx.args() {
        [KValue::Str(text)] => {
            let mut poetry = Series::init(text);
            //poetry.init(text);
            Ok(KObject::from(KotoSeries(poetry)).into())


        }
      //  [List(s)]=>list_to_serise(s.clone()),
        unexpected => unexpected_args(
            "|String|, or |Number, Number, Number|, or |Number, Number, Number, Number|",
            unexpected,
        ),
    });

    // meta.add_fn("leven", |ctx| match ctx.args() {
    //     [KValue::Number(n)] => Ok((n * n).into()),
    //
    //     //  [List(s)]=>list_to_serise(s.clone()),
    //     unexpected => unexpected_args(
    //         "|String|, or |Number, Number, Number|, or |Number, Number, Number, Number|",
    //         unexpected,
    //     ),
    // });


    result.set_meta_map(Some(meta.into()));

    result
}



#[derive(Clone, KotoCopy, KotoType)]
#[koto(type_name = "Series")]
struct KotoSeries(Series);

impl KotoEntries for KotoSeries {}

impl KotoObject for KotoSeries {


    // fn getok(&self) ->Vec<f64> {
    //     self.0.clone().links
    // }
    //
    // fn iterator_next(&mut self, _vm: &mut KotoVm) -> Option<KIteratorOutput> {
    //     self.0
    //         .next_word()
    //         .map(|word| KIteratorOutput::Value(word.as_ref().into()))
    // }
}


// fn list_to_serise(l: KList) -> Result<KValue> {
//     Ok(Series::init(l).into())
//
//
// }
