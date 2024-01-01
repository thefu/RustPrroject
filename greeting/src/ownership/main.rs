/**
 * 对于变量来说，这是最复杂的情况了
 * 如果将一个变量当做函数的参数传给其他函数，怎样安全的处理所有权呢？
 * 下面这段程序描述了这种情况下所有权机制的原理：
 */

fn main() {
    let s = String::from("hello");
    //s被声明有效

    takes_ownership(s);
    //s的值被当作参数传入函数，
    //所以可以当作s已经被移动，从这里开始就已经无效

    let x = 5;
    //x被声明有效

    make_copy(x);
    //x的值被当作参数传入函数
    //但x是基本类型，依然有效
    //在这里依然可以使用x却不能使用s
}
//函数结束，x无效，然后是s，但s已经被移动，所以不用被释放

fn takes_ownership(some_string: String) {
    //一个String参数some_string传入，有效
    println("{}", some_string);
}
    //函数结束，参数some_string在这里释放

fn make_copy(some_integer: i32) {
    //一个i32参数的some_integer传入，有效
    println("{}", some_integer);
}
//函数结束，参数some_integer是基本类型，无需释放

/**
 * 如果变量被当作参数传入函数，那么它和移动的效果是一样的。
 */