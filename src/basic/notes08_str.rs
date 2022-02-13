//string::
/*
rust 中的str 用utf-8 存储,  比想象的中的复杂. 当然字符串涉及到编码,本来也不简单.
字符串 就是 vec<u8>

String, str 有啥差别.

str 是切片,  字面量 也是切片的方式存在.

rust 核心,只有一个 str. 相当于一个 切片.
多以 &str 的方式出现.

String 是rust标准库, 提供的. 类似cpp的string?
可修改,可增长, 可拥有. 其实,就是个结构体.

Rust 内还包含了其它str
OsString, OsStr, String, CStr

 String 可拥有. str 可借用.(引用)

 */


#[test]
fn cb01_string() {
    //创建.
    let a = String::from("hello");
    let b = String::new();//空串.
    let  mut c = "wzy".to_string();

    //push_str  很明显表示的是, 参数是&str.
    c.push_str("wxy"); //字符串连接.
    c.push_str(&a); //参数是 &str, 所以可以用String, 也可以用字面量.
    println!("c = {:?}",c);

    // c.push_str(&c);
    // 这样编译会不能通过. c作为参数,已经借给外面了. 但是c.push_str之后又改变了c
    // 如果要实现这个功能,要向下面这样.
    c.push_str(&c.clone());

    c.push('a'); //增加字符.
}

// 字符串拼接, 编程中很常用. 出了 c 基本都支持, a + b 的方式.
// rust 中使用了 a + &b 的方式.
// 相当于运算符重载.   fn add(item1 mut String, item2 &str)
#[test]
fn cb02_str_add() {
    let s1 = String::from("hello");
    let s2 = String::from("world");

    //let s = s1 + s2; //这样不行.
    let s = s1 + &s2;
    println!("s:{}",s);
    //println!("s1:{}",s1);  //这句会报错.
                             //因为 s1+&s2. s1已经移动到函数的某个参数了.
                             //s1 失效.
    println!("s2:{}",s2); //s2 可以使用.

    let s3 = String::from("hello");
    let s4 = String::from("world");
    let s = format!("{} {}",s3,s4);
    println!("s:{}",s);

    println!("s1:{};s2:{}",s3,s4);
    // format 后还可以用. 这是为啥?
    //宏 format! 生成的代码使用索引并且不会获取任何参数的所有权.
}


//遍历字符串
// 遍历直接 bytes
// 遍历unicode chars
// 遍历 字形簇.  //这个要第三方库.

#[test]
fn cb03_str_iter() {
    let s1 = String::from("你好a");
    println!("s1.len = {}",s1.len()); //7

    for i in s1.bytes() {
        println!("i={}",i);
    }
    //println!("s1.len2 = {}",s1.chars());
    for i in s1.chars() {
        println!("i={}",i);
    }
}


#[test]
fn cb03_str_slice() {
    let s1 = String::from("你好a中国.");
    println!("{:?}",&s1[0..3]); //你, utf-8 的编码.
    //println!("{:?}",&s1[0..2]); //你, utf-8 的编码. 这个会挂.  因为前两个字节不是正确编码.
}