//介绍几种常用的数据结构.
//vec
//string
//map

//vec 对应cpp的vector. go的slice,
//常用方法, push,pop 啊之类的, 看相关文档就好.

#[test]
fn cb01_vec() {
    let mut v1:Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(100);
    println!("v1: {:?}", v1);

    let mut v2:Vec<String> = Vec::new();
    v2.push(String::from("wzy"));
    v2.push(String::from("wxy"));
    println!("v2: {:?}", v2);

}

//rust 中有比较强大的宏. (这个好), rang vec使用可以简化.
#[test]
fn cb02_vec_macro() {
    let v1 = vec!(1,2,3);
    println!("v1: {:?}",v1);
    let v2 = vec![1,23,4];
    println!("v2: {:?}, size={}",v2,v2.len());
    //上面两种, 竟然都可以?

    //获取元素值.

    println!("v1[2]={}",v1[2]); //v1[2]=3
    println!("v1[2]={:?}",v1.get(2)); //v1[2]=Some(3)

    match v1.get(2)  {
        Some(3) =>{
            println!("normal v");
        },
        None=>{
            println!("not exist");
        }
        _ => {
            println!("ot")
        }
    };



}

//遍历的时候,就是for了.  遍历的时候,顺便获取索引? 这个不知道怎么弄.
// vec 中也可以春风trait(对象),  类似go的结构. 这样里面的内容就多了.
//vec 也可以放枚举. 这个就先不讨论了.
#[test]
fn cb02_vec_bianli() {
    let v1 = vec![1,23,4];
    //遍历, vec
    // for i in v1 { // 这里是取到值了. 而且v1 转移到别的地方去了.
    //     println!("i={}",i);
    // }

    //this function takes ownership of the receiver `self`, which moves `v1`
    for i in  &v1 { // 这里是取到的是引用.
        println!("i={}",i);
    }

    show_vec_item(&v1);
}
//vec 作为参数的, 引用表示.
fn show_vec_item(arr :&[i32]) {
    for i in arr {
        println!("i = {}",i);
    }
}
