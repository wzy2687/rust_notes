//map  除了c, 几乎其它语言都有.
use std::collections::HashMap;

#[test]
fn cb01_map_base() {
    //创建
    let mut  insMap :HashMap<&str,i32> = HashMap::new();
    println!("insMap: {:?}", insMap);
    insMap.insert("abc",123);
    insMap.insert("bcd",456);
    println!("insMap: {:?}", insMap);
    insMap.insert("abc",1234); //更新.
    println!("insMap: {:?}", insMap);

    //两个tuple 的对应.
    let v1 = vec![1,2,3];
    let v2 = vec![4,5,6];

    let m :HashMap<_,_> = v1.iter().zip(v2.iter()).collect();
    println!("m: {:?}",m);

    // 从hashmap 中取值,
    match m.get(&1) {  //zh
        Some(i)=> println!("v = {}",i),
        None => println!("no value"),
    }

    //遍历.  一遍遍历的是, 字典的引用 &map, 否则遍历后,字典不能用了.
    for (k,v) in &m {
        println!("k:{},v{}",k,v);
    }

}

//某些情况下, 需要key不存在才创建. 或者key不存在才更新.
//需要用到 entry 函数.
#[test]
fn cb02_map_entry() {
    let mut m = HashMap::new();
    m.insert("a",1);
    m.insert("b",2);

    m.entry("a").or_insert(100);  //不存在才创建.
    m.entry("c").or_insert(3); //m: {"a": 1, "b": 2, "c": 3}
    //or_insert 返回的是 mut &v; 外部可以改变v.

    println!("m: {:?}",m);
}