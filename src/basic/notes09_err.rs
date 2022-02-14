//rust 中没有异常.
//出错处理 往往被设置  Result<T,E> 这样的形式.

use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;

#[test]
fn cb01_err_panic() {
    panic!("err hanppen; please check"); //程序崩溃, 会输出调用堆栈.
}

#[test]
fn cb02_err_panic2() {
    let v = vec![1, 2, 3];
    println!("{}", v[100]); //也会崩溃, 输出调用堆栈.
    //环境变量 `RUST_BACKTRACE=full` 设置后, 也可以看到更多详细信息.
}

/*
rust 的错误处理, 类似之前的讲的Option. rust还有一种枚举
enum Result<T,E> {
    OK(T),
    ERR(E)
}
*/

#[test]
fn cb03_err_result() {
    let f = File::open("hello.md");
    let f = match f {
        Ok(f) => f,
        Err(e) => {
            panic!("open err {:?}", e)
        }
    };
    //上面处理代码, 和 c, cpp, go 比起来, 理解也不难. 参照之前的option
}

#[test]
fn cb04_err_result_match() {
    let f = File::open("abc.md"); //运行之前把 abc.md 删除
    let f = match f {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("abc.md") {
                Ok(fd) => fd,
                Err(e) => panic!("new err {:?}",e),
            },
            other_error => panic!("open err {:?}", e),
        }
    };
    //更具体的错误处理, 感觉写起来也不简单.
    //上面一连3个match. 难道不应该有更好的方法吗?

    //一种用闭包的简单方法, 可以是这样.

    let f = File::open("n.md").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("n.md").unwrap_or_else(|error|{
                panic!("new err {:?}",error);
            })
        } else {
            panic!("err {:?}",error);
        }
    } );

}

//unwrap 函数

fn cb05_err_unwrap_expect() {
    let f = File::open("hello.md");
    let f = match f {
        Ok(f) => f,
        Err(e) => {
            panic!("open err {:?}", e)
        }
    };
    //上面代码.等价于
    let f = File::open("hello.md").unwrap();
    //这一句,等价于上面的一段. 这样感觉挺简单.
    //unwrap相当于, 成功就正常返回, 否则挂掉, panic.
    //unwrap相当于 的错误信息不能定制. 但是 expect 可以. 作用类似unwrap.

    let f = File::open("hello.md").expect("open hello.md error");

}

//上面都是决定了. 很多情况下, 需要做错误回传.
//进一步操作交给调用者.

fn read_username_from_file() ->Result<String,io::Error> {
    let f = File::open("user.db");
    let mut f = match f {
        Ok(file)=>file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(& mut s) {   //注意可变引用. 传参数的写法.  相当于c中取地址的符号, 其实也就是那个意思.
        Ok(_)=>Ok(s),
        Err(e)=>Err(e),
    }
}

#[test]
fn cb06_err_to_caller() {
    println!("rt:{:?}",read_username_from_file());
}

//按照上面的写法, 错误处理比较痛苦. golang 因为错误处理,骂声不断.
//rust 有相对简单的做法.
// rust 维持, 还发明了符号: ?  用于回传错误.
// 针对上面的例子.看看下面的简写.

//下面这个版本,就清爽多了.
fn read_username_from_file_v2() ->Result<String,io::Error> {
    let mut f = File::open("user.db")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

//还能链式调用,更剪短.
fn read_username_from_file_v3() ->Result<String,io::Error> {
    let mut s = String::new();
    File::open("user.db")?.read_to_string(&mut s)?;
    Ok(s)
}

//何处可以使用 "?" 呢
//感觉返回 Option 和Result 的时候可用.
/*

fn f1() ->Atype {
    f2()?
}
//f2返回到 类型,和f1 返回的类型Atype 兼容.
 */

//什么时候panic!
//有害状态: 当一些假设、保证、协议或不可变性被打破的状态.



