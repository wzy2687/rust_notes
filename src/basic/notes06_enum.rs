//=====================================================================
// 枚举
//=====================================================================
/*
 枚举 : 列举出所有可能的值.
 rust 枚举有两种.
 1 : 类似c (注意, 枚举的使用要加上作用域.)
 2 : 专有的用于模式匹配. 标签关联枚举, 比如把两种结构体,强制定义为一种结构.
*/

#[derive(Debug)]
enum ClientState {
    state_none2 = 1,
    state_ok = 2,
    state_disconnected = 3,
    state_ing = 4,
    state_dis_ing,
    state_init = 100,
}

#[derive(Debug)]
enum Gender { Unspecified = 0, Female = 1, Male = 2 }

#[test]
fn cb0x_enum01() {
    //注意, 枚举的使用要加上作用域. 否则会报错.
    println!("e1 = {:?}", ClientState::state_none2); //e1 = state_none
    println!("e1 = {:?}", ClientState::state_ing); //e1 = state_none
    println!("e1 = {:?}", Gender::Female); //e1 = state_none


}


//专有的用于模式匹配. 标签关联枚举.
//这里需要注意, String 是个结构体.
#[derive(Debug)]
enum IPAddr {
    V4(String),
    V6(String),
}

#[test]
fn cb0x_enum02() {

    println!("ip = {:?}",IPAddr::V4(String::from("127.0.0.1")));
    // println!("ip = {:?}",IPAddr::V4(String::from("127.0.0.1")));
    //输出的是 ip = V4("127.0.0.1"). 如果想输出 127.0.0.1 呢?
}


//枚举内的,标签代表的结构可以不同.
#[derive(Debug)]
enum Message {
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
//枚举内的数据, 类型可以不同,
// Quit,整数类,
// Move 类似结构体.
// Write 包含一个单独的String
// ChangeColor 一个元组.
// 把不同的结构体, 汇聚为一种类型. op 包含,增删改查.

#[test]
fn cb0x_enum03() {
    println!("color = {:?}",Message::ChangeColor(255,0,255));
    //out:color = ChangeColor(255, 0, 255)
}



/*
学习具体取标签对应的类型值之前, 了解系统一个重要的枚举.
还是个泛型枚举.

enum Option<T> {
    None,
    Some(T),
}

这个用于表示什么呢?  比如 Option<i32>, 表示某个i32, 或者
什么都没有.  这个在编程中比较常用. 比如打开文件. Option<fd>

c语言.  往往下面的样.
int  get_value(&value);
int 用来表示有无, value 正常情况下的值.
golang 往往是.

fun get_value()-> (int32,error);

rust 则是另一种手段.
fn get_value()->Option<i32>
返回值, 可以用于自动判断,是否正常.
https://doc.rust-lang.org/std/option/enum.Option.html
*/

#[test]
fn cb0x_enum04() {

    //option 使用.
    let x1 = Some(5);
    let f1 = Some(2.4);
    let s1 = Some("abc");

    //let n = None; //这样2编译会报错.
    let n :Option<i32> = None;

    let i = 5;

    if n.is_none() {
        println!("n = {:?}",n)
    }
    //获取T 值.
    let v2 = x1.unwrap();
    println!("x1 = {}",v2);

}

/*
 rust 一个比较强大的特性, match.
 听着想switch case.  其实远比这个强大.
 和 枚举联合起来用. 更好.
*/


//下面的例子比较简单. 相当于简单的switch case.
#[test]
fn cb0x_match01() {

    #[derive(Debug)]
    enum A {
        a1,
        a2,
    }
    let p = A::a1;

    //match 匹配要用 => 符号.
    match p {
        A::a1 => {
            println!("a1 bc");
        },
        A::a2 => {
            println!("a2 bc");
        }
    };

}

/*
match 可以匹配模式.
但什么是模式, 暂时不理解, 可以想成具体的enum 类型值.

*/

#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

//注意,这里的的Coin中,几个具体值, 类型已经不同了.
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),  //Quarter 和另一个enum UsState 绑定.
}
#[test]
fn value_in_cents()  {
    let coin = Coin::Quarter(UsState::Alaska) ;
    let rt = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            //这里,注意这个state, 直接匹配了. UsState::Alaska
            //里面还可以二次匹配.
            println!("State quarter from {:?}!", state);
            match state {
                UsState::Alaska =>30,
                UsState::Alabama =>40,
            }
        }
    };
    println!("rt = {}",rt);
}

//匹配, rust 中的option 更是常见.
//match 的意思是, v 是 那个枚举类型呢?

fn GetId(v :Option<i32>) ->Option<i32> {
    match v {
        None => None,
        Some(in_val)=>Some(in_val+1),
        //注意,这里的in_val, v中的值,匹配上了.
        //3 =>3+1, 这个会报错. 对接的类型必须是 Option<i32>
        //另外,需要再次明白, enum内可以是不同的类型.
    }
}
#[test]
fn cb0x_match02() {
    let id1 = Some(32);
    let id2 = None;

    println!("rt = {:?}",GetId(id1));
    println!("rt = {:?}",GetId(id2));

}

//匹配中的default. rust 中匹配必须穷尽所有.(支持default)
//思路就是, 写明白你在做什么.
#[test]
fn cb0x_match03() {
    let v = 5;

    let rt = match v {
        7 =>17,
        _ =>27,
    };
    println!("rt = {:?}",rt);

    let rt = match v {
        7 =>17,
        other =>other,
    };
    println!("rt = {:?}",rt);

    //注意 other 和 _ 的差异, other 会绑定到给定的值v._ 不会.
}

//关于 if let
// 注意 if,一个东西, let一个东西.  if let 似乎又是另外一个东西.
// 类似js.  但是这个规定不好. 学习有前抑效应, 后抑效应.
#[test]
fn cb0x_match04() {
    //if let

    let t = 100;
    if let t = 100 {
        println!("100");
    } else {
        println!("other")
    }

}