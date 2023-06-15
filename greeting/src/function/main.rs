fn main() {
    println!("Hello, world!");
    another_function();
    another_function2(5, 6);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
/**
 * 这段代码包含了一个表达式块，而且在块中可以使用函数语句，最后一个步骤是表达式，此表达式的结果值是整个表达式块所代表的的值。这种表达式块叫做函数体表达式。
 * 注意：x+1之后没有分号，否则它将编程一条语句
 */

    fn five() -> i32 {
        5
    }
    println!("five()的值为：{}", five());
    /**
     * 在rust中，函数定义可以嵌套
     * Rust函数声明返回值类型的方式：在参数声明之后用 ->来声明函数返回值的类型（不是：）
     * 在函数体中，随时都可以以return关键字结束函数运行并返回一个类型合适的值。这也是最接近大多数开发者经验的做法。
     */


    println!("x的值为：{}", x);
    println!("y的值为：{}", y);
}

fn another_function() {
    println!("hello ,runoob!");
}

//rust中定义函数必须声明参数名称和类型；
fn another_function2(x: i32, y: i32) {
    println!("x的值为: {}",  x);
    println!("y的值为: {}",  y);
}

fn add(a: i32, b: i32) {
    return a + b;
}
/**
 * 但是Rust不支持自动返回值类型判断！如果没有明确声明函数返回值的类型，函数将被认为是“纯过程”，不允许产生返回值，return后面不能有返回值表达式，这样做的目的是为了让公开的函数能够形成可见的公报。
 * 
 * 函数体a表达式并不能等同于函数体，它不能使用return关键字。
 */