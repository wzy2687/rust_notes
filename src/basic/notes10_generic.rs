
// 泛型这东西, 之前用过的也就是cpp, stl.
// 这个东西, 相当于在函数(方法) 上的一种抽象.
// 型, 指的是类型.  泛型 就是广泛的类型.(很多类型都可以)
// 看下面代码
/*
fn sum(a:i32,b:i32 )->i32 {}
fn sum(a:f32,b:f32 )->f32 {}
上面两个, 可以简化为一个.

fn sum<T> (a:T,b:T ) ->T {...}

记得一个go大佬说过, 通常情况下, 写普通函数就行了.
当需要是, 再把函数的类型抽象出来.
*/
// fn sum(a:i32,b i32 )->i32 和 fn sum(a:f32,b f32 )->f32 是具体具体
//

//rust 中的泛型, 类似cpp的泛型, <>语法.
//有一个需要注意的地方, 如何对T 做限制.
// 下面的例子, 如果 T不能执行 +. 怎么办.

// fn Sum<T>(a:T,b:T)->T {
//     a+b
// }

#[test]
fn cb01_generic() {
    //下面函数编译不过, 因为没有对T做限制. 但是这里先不考虑.
    // 知道大概这个样子就行. (这也是一种教育方式,)
    // 不一定要走通, 把目前想说的,说清楚就ok.
    // println!("v1 = {}",Sum(3,4));
    // println!("v2 = {}",Sum(3.5,4.5));
}

//出了函数泛型, 还有结构体泛型.
#[derive(Debug)]
struct Point<T> {
    x :T,
    y :T,
}
#[test]
fn cb02_generic_struct() {
    println!("s :{:?}",Point{x:5,y:6}) ;//s :Point { x: 5, y: 6 }
    //能推断出来,类型, 就不用指明.
}

// 还有枚举中的泛型
//最常用的两个
/*
enum Option<T> {
    Some(T),
    None,
}

enum Result<T,E> {
    Ok(T),
    Err(E),
}
*/


// 方法的枚举, 类似函数, 主要是需要注意语法.

// 注意 impl 后面那个 <T>
impl <T> Point<T> {
    fn str(self) ->String {
        //....
        return String::from("pp");
    }
}

// 泛型类似cpp模板, c中 宏.  编译器会在编译器, 生成具体 代码.
// 编译慢点, 执行的时候,没有性能损失.