/// struct 相关知识.

//struct 多种struct.


//普通struct, pub属性对外可见, 否则不可见
//这样的结构体, 各种语言差不多, c语言例外(都可见), cpp ,go, 都有配置.
#[derive(Debug)]
struct Ins {
    pub Code: String,
    pub PriceTick: f64,
    beta:i32,
}

#[test]
fn cb01_struct_type() {
    //初始化.
    let ins = Ins{
        Code:String::from("000001.SZ"),
        PriceTick:0.01,
        beta:1,
    };
    println!("ins_code= {}",ins.Code);
    println!("beta= {}",ins.beta); //内部模块,beta可见, 外部模块不可见.

    //从一个struct 快速构建另外一个struct 的方法.

    let ins2 = Ins {
        Code: String::from("000002.SZ"),
        ..ins  //注意,这里不用写 "," , 否则会报错.
    };

    println!("ins2 = {:?}",ins2); //ins2 = Ins { Code: "000002.SZ", PriceTick: 0.01, beta: 1 }

}

//struct rust中, 还有其它类型, 匿名类型, 空类型.
struct Color(i32,i32,i32); //感觉就像一个元组, 但是Color 本身,是一种类型.
//元组的内容,查找notes01.
struct Point(i32,i32,i32); //感觉就像一个元组, 但是Color 本身,是一种类型.

#[test]
fn cb02_struct_noname() {
    let c = Color(255,255,0);
    println!("c.R= {}",c.1); // 访问属性的时候, 用下表.  n.0, n.1, ... n.x
}

// 空结构体. 也称为单元结构体. 用于某种类型想实现trait, 但是又没有成员的情况.
#[derive(Debug)]
struct NStruct ;
#[test]
fn cb03_struct_noattr() {
    let c = NStruct;
    println!("c.R= {:?}",c); // 访问属性的时候, 用下表.  n.0, n.1, ... n.x
}

//结构体, 也可以有方法. 实现起来和其它语言有点差别. 大差不差.
//impl 关键字, 用于表示, 结构体下, 有哪些方法, 和关联函数.
// 方法, 就是参数中有(&self),  关联函数, 就是参数中没有 self.
//一个结构体, 可以有多个impl 块.


struct Rect {
    w :i32,
    h:i32,
}

impl Rect{
    fn Area(&self) ->i32 {
        return self.w*self.h;
    }
    fn Squre(s:i32) ->Rect {
        return Rect{w:s,h:s}
    }
}

impl Rect {
    fn zoucang(&self) ->i32 {
        return 2*(self.w+self.h);
    }
}

#[test]
fn cb04_struct_method() {
    let rect = Rect{w:40,h:50};
    println!("area = {}",rect.Area());// 调用方式和其它语言差不多.
}
