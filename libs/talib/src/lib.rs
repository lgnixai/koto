//! A Koto language module for working with colors

mod color;

pub use color::Color;

use koto_runtime::{KSeries, prelude::*, PtrMut, Result};
use palette::{Hsl, Hsv};
use yata::prelude::*;
use yata::methods::SMA;
use koto_runtime::KNumber::F64;
use koto_runtime::KValue::Series;

// SMA of length=3

pub fn make_module() -> KMap {
    use KValue::{Number, Str};
    let mut result = KMap::default();

    result.add_fn("sma", |ctx| match ctx.args() {
        [Series(src), Number(len)] => {
            let mut rs = ValueVec::new();

            println!("{:?}",f32::from(len));
            let mut sma = SMA::new(f32::from(len) as u8, &0.0).unwrap();
            for (n) in src.data().iter() {
                let value: f64 = match n {
                    KValue::Number(knumber) =>{

                        knumber.into()
                    } , // 将 KNumber 转换为 usize
                    // 如果 KValue 还有其他变体，你可以选择如何处理这些情况
                    _ => panic!("KValue does not contain a KNumber"), // 或者处理为默认值
                };

               // println!("{:?}",value);

                rs.push(KValue::from(sma.next(&value)))
            }
            let rest=KSeries {
                history: PtrMut::new(KCell::from(rs)),
            };
            Ok(KValue::Series(rest))
        }
        unexpected => unexpected_args("|Number, Number, Number|", unexpected),
    });

    result.add_fn("hsv", |ctx| match ctx.args() {
        [Number(h), Number(s), Number(v)] => {
            let hsv = Hsv::new(f32::from(h), f32::from(s), f32::from(v));
            Ok(Color::from(hsv).into())
        }
        unexpected => unexpected_args("|Number, Number, Number|", unexpected),
    });

    result.add_fn("named", |ctx| match ctx.args() {
        [Str(s)] => named(s),
        unexpected => unexpected_args("|String|", unexpected),
    });

    result.add_fn("rgb", |ctx| match ctx.args() {
        [Number(r), Number(g), Number(b)] => rgb(r, g, b),
        unexpected => unexpected_args("|Number, Number, Number|", unexpected),
    });

    result.add_fn("rgba", |ctx| match ctx.args() {
        [Number(r), Number(g), Number(b), Number(a)] => rgba(r, g, b, a),
        unexpected => unexpected_args("|Number, Number, Number, Number|", unexpected),
    });

    let mut meta = MetaMap::default();

    meta.insert(MetaKey::Type, "ta".into());
    meta.add_fn(MetaKey::Call, |ctx| match ctx.args() {
        [Str(s)] => named(s),
        [Number(r), Number(g), Number(b)] => rgb(r, g, b),
        [Number(r), Number(g), Number(b), Number(a)] => rgba(r, g, b, a),
        unexpected => unexpected_args(
            "|String|, or |Number, Number, Number|, or |Number, Number, Number, Number|",
            unexpected,
        ),
    });

    result.set_meta_map(Some(meta.into()));
    result
}

fn named(name: &str) -> Result<KValue> {
    match Color::named(name) {
        Some(c) => Ok(c.into()),
        None => Ok(KValue::Null),
    }
}

fn rgb(r: &KNumber, g: &KNumber, b: &KNumber) -> Result<KValue> {
    Ok(Color::rgb(r.into(), g.into(), b.into()).into())
}

fn rgba(r: &KNumber, g: &KNumber, b: &KNumber, a: &KNumber) -> Result<KValue> {
    Ok(Color::rgba(r.into(), g.into(), b.into(), a.into()).into())
}
