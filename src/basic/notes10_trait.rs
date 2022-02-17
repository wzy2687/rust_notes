/*
trait 类似go的interface
trait 就是方法的几何.  注意是方法, 不是函数.

*/

use crate::show;

pub trait Des{
    fn Str(&self) -> String;
}

pub struct Ins {
    Code :String,
    Price :f64,
}

impl Des for Ins {
    fn Str(&self) -> String {
        return self.Code.clone()
    }
}


// 这里结合 go 中接口作为参数有点想了.
// 这个有个问题, 就是 trait 里面一定是方法吗. 不能是函数吗?

fn Show(v: &impl Des) {
    println!("v= {}",v.Str());
}


#[test]
fn use_trait() {
    let v = Ins{
        Code : String::from("000002.SZ"),
        Price:10.0,
    };
    println!("ins={}, {}",v.Str(),v.Code);
    Show(&v);
}