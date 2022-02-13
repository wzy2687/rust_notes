

#[test]
pub fn cb01_hello_world() {
    println!("hello world");
}

#[test]
pub fn cb02_var_mut() {
    //==简单声明
    let i = 4;
    let f = 6.01;
    println!("{}, {}",i,f);
    // i = 43; //这句会报错, 因为 i 是不可变的.

    //==可变声明==== mut
    //
    let mut mi = 45; //可变的变量必须加 mut.
    println!("mi = {}",mi);
    mi = 100;
    println!("mi = {}",mi);

    //==常量的声明方式  const NAME : TYPE  = VALUE;
    const PI:f64 = 3.14;
    println!("PI = {}",PI);

    //==类似c 也有作用域的概念.
    let x = 5;
    let x = x + 1; //注意这一句. 其它语言不一样.  x+1 转译到新的x上.
    //类似 let y = x + 1;
    //很多语言不能这样用.会提示变量声明冲突. 这里x是个新的物种.
    //这里也让不可变的x可变了. 其实是一死一生.
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    //==再来一个 覆盖; 不太理解为什么可以这样搞.
    let name = "wzy";
    let name = name.len();
    println!("name =  {}", name);
    // 下面这样会出错.
    // let mut name = "wzy";
    // name = name.len();
    // println!("name =  {}", name);
}

#[test]
pub fn cb03_datatype() {
    //======================================================================
    // 基本类型: 整型、浮点型、布尔类型和字符类型
    //======================================================================

    //整形 很简洁.
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize

    let i1 :i32 = 1_234_567; //支持 x_yyy_zzz的写法.
    let c :u8 = b'A'; //一种写法.

    let ff :f32 = 0.78;
    let flag = false;

    //字符类型
    let x = 'w';
    let p = 'z';

    //===================================
    //复合类型
    //===================================
    //元组, 数组.
    let idx = (1,2,3,4);
    let tup = (2,3.0,4.9);
    let t3 = (2,8,true);
    println!("tup = {}",tup.1); // 注意索引的使用方式.

    let (a,b,c) = t3; //换可以这样.
    println!("tup = {} ,{}, {}",a,b,c);

    //数组. 数组的元素类型必须相同. 而且长度是固定的.
    let arr = [1,2,3,4];
    let px = ["wzy","wzy"];
    println!("arr[1] = {}",arr[1]); //这个类似c.
}
