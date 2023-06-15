fn main() {
    let number = 3;
    if number < 5 {
        println!("条件为true")
    } else {
        println!("条件为false");
    }
    //条件表达式类型必须为bool类型

    //虽然C/C++语言中的条件表达式用整数表示，非0即真，但这个规则在很多注重代码安全性的语言中是被禁止的；

    let a = 3;
    let number = if a > 0 {1} else {-1};
    println!("number为{}", number);

    //在Rust中可以使用if-elseee结构实现类似于三元条件表达式（A?B:C）的效果；
    //注意：两个函数体表达式的类型必须一样！且必须有一个else及其后的表达式块。
}