/**
 * Option 是 Rust 标准库中的枚举类，这个类用于填补 Rust 不支持 null 引用的空白。

许多语言支持 null 的存在（C/C++、Java），这样很方便，但也制造了极大的问题，null 的发明者也承认这一点，"一个方便的想法造成累计 10 亿美元的损失"。

null 经常在开发者把一切都当作不是 null 的时候给予程序致命一击：毕竟只要出现一个这样的错误，程序的运行就要彻底终止。

为了解决这个问题，很多语言默认不允许 null，但在语言层面支持 null 的出现（常在类型前面用 ? 符号修饰）。

Java 默认支持 null，但可以通过 @NotNull 注解限制出现 null，这是一种应付的办法。

Rust 在语言层面彻底不允许空值 null 的存在，但无奈null 可以高效地解决少量的问题，所以 Rust 引入了 Option 枚举类：
 */
enum Option<T> {
    Some(T),
    None,
}

//如果你想定义一个可以为空值的类，你可以这样：
fn main() {
    let opt = Option::Some("Hello");
}
//如果你想针对 opt 执行某些操作，你必须先判断它是否是 Option::None：
fn main() {
    let opt = Option::Some("Hello");
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }
}
//如果你的变量刚开始是空值，你体谅一下编译器，它怎么知道值不为空的时候变量是什么类型的呢？
//所以初始值为空的 Option 必须明确类型：
fn main() {
    let opt: Option<&str> = Option::None;
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }
}

/**
 * 这种设计会让空值编程变得不容易，但这正是构建一个稳定高效的系统所需要的。由于 Option 是 Rust 编译器默认引入的，在使用时可以省略 Option:: 直接写 None 或者 Some()。
 */
//Option 是一种特殊的枚举类，它可以含值分支选择：
fn main() {
    let t = Some(64);
    match t {
            Some(64) => println!("Yes"),
            _ => println!("No"),
    }
}

fn main() {
    let i = 0;
    match i {
        0 => println!("zero"),
        _ => {},
    }
}

//这段程序的目的是判断 i 是否是数字 0，如果是就打印 zero。
//现在用 if let 语法缩短这段代码：
fn main() {
    let i = 0;
    if let 0 = i {
        println!("zero");
    }
}

//if let 语法格式如下：
/**
 * if let 匹配值 = 源变量 {
    语句块
   }
 */

//可以在之后添加一个 else 块来处理例外情况。
//if let 语法可以认为是只区分两种情况的 match 语句的"语法糖"（语法糖指的是某种语法的原理相同的便捷替代品）。
//对于枚举类依然适用：
fn main() {
    enum Book {
        Papery(u32),
        Electronic(String)
    }
    let book = Book::Electronic(String::from("url"));
    if let Book::Papery(index) = book {
        println!("Papery {}", index);
    } else {
        println!("Not papery book");
    }
}
