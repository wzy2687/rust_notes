/*
直接总结一下
	1:最开始, 一个值一个所有者
	2: 一些场景, 借用打破这个说法
		 2.1 借用其实,相当于某个胖指针的指针.
		 2.2 (p->v(x) ;x-> heap mem
	3: 于是 如何保证, 借用 获得比胖指针短.(不出现悬垂指针,野指针)
	4: 编译器需要, 标注生命周期 来做这件事.
		用在多个引用, 相互比较的时候.
		用生命周期,标注引用,意思是引用 活的时间.
	5: 生命周期, 标注, 并不改变什么引用的存活时间. 也不改变 引用指向实体的存活时间.
	6: 知道这个参照各具体的例子, 基本就明白生命周期 怎么回事了.

看看下面的代码

```
fn longest_str(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let str1 = String::from("123");
    let str_longer;
    {
        let str2 = String::from("12345");
        str_longer = longest_str(&str1, &str2);
    }
    println!("{}",str_longer);
}
```

str_longer  可能如果指向 str2.  出了str2, 	str_longer 就成了悬垂指针了.

编译器在处理 fn longest_str(x: &str, y: &str) -> &str  函数的时候.
编译器,就处理不了这种情况了. 即使下面这样也会报错.  编译器似乎, 遇到多个引用的时候.
就会出问题.

fn longest_str(x: &str, y :&str) -> &str {
    "abc"
}

//这个可以.
fn longest_str(x: &str) -> &str {
    "abc"
}

//这样都不行.
fn longest_str(x: &str, y :&i32) -> &str {
    "abc"
}

//编译器目前还是有点偷懒的.

//这个也编译不过. 因为 y 那个分支会报错.  a,b 无法比较.
fn longest_str<'a,'b >(x: & 'a str, y :& 'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

要记住生命周期的目的: 当且仅当"输出值"的生命周期为 "所有输入值的生命周期" "交集的子集"
时，生命周期合法。

*/

fn longest_str<'a >(x: & 'a str, y :& 'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test]
fn cb01_lifetime( ) {
    let v = longest_str("abc","def");
    println!("{}",v);
}