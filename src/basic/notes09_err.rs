//rust 中没有异常.
//出错处理 往往被设置  Result<T,E> 这样的形式.

use std::fs::File;
use std::io::ErrorKind;

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

    let f = File.open("n.md").unwrap_or_else(|error| {
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


