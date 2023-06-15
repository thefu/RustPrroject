fn main () {
    let a = 12;
    println!("a is {}",  a);
    println!("a is {}, a again is {}", a, a);

    //更好的写法
    println!("a is {0}, a again is {0}", a); //在{}之间可以放一个数字，它将把之后的可变参数当做一个数组来访问，下标从0开始。
}