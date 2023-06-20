/**
 * 枚举的目的是对某一类事物的分类，分类的目的是为了对不同的情况进行描述。
 * 基于这个原理，往往枚举类最终都会被分支结构处理（许多语言中的 switch ）。 
 * switch 语法很经典，但在 Rust 中并不支持，很多语言摒弃 switch 的原因都是因为 switch 容易存在因忘记添加 break 而产生的串接运行问题，
 * Java 和 C# 这类语言通过安全检查杜绝这种情况出现。
 */

//Rust 通过 match 语句来实现分支结构。先认识一下如何用 match 处理枚举类：
fn main() {
    enum Book {
        Papery {index: u32},
        Electronic {url: String},
    }
   
    let book = Book::Papery{index: 1001};
    let ebook = Book::Electronic{url: String::from("url...")};
   
    match book {
        Book::Papery { index } => {
            println!("Papery book {}", index);
        },
        Book::Electronic { url } => {
            println!("E-book {}", url);
        }
    }
}

//match 块也可以当作函数表达式来对待，它也是可以有返回值的：
/**
 * match 枚举类实例 {
    分类1 => 返回值表达式,
    分类2 => 返回值表达式,
    ...
   }
 */

/**
 * 但是所有返回值表达式的类型必须一样！
 * 如果把枚举类附加属性定义成元组，在 match 块中需要临时指定一个名字：
 */

 enum Book {
    Papery(u32),
    Electronic {url: String},
}
fn main() {
    let book = Book::Papery(1001);

    match book {
        Book::Papery(i) => {
            println!("{}", i);
        },
        Book::Electronic { url } => {
            println!("{}", url);
        }
    }
}

//match 除了能够对枚举类进行分支选择以外，还可以对整数、浮点数、字符和字符串切片引用（&str）类型的数据进行分支选择。
//其中，浮点数类型被分支选择虽然合法，但不推荐这样使用，因为精度问题可能会导致分支错误。
//对非枚举类进行分支选择时必须注意处理例外情况，即使在例外情况下没有任何要做的事 . 例外情况用下划线 _ 表示：
fn main() {
    let t = "abc";
    match t {
        "abc" => println!("Yes"),
        _ => {},
    }
}


