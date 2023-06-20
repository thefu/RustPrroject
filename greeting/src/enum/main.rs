/**
 * 枚举类在Rust中并不像其他编程语言那样简单，但依然可以十分简单的使用
 */


 #[derive(Debug)]

 enum Book {
     Papery, Electronic
 }
 
 fn main() {
     let book = Book::Papery;
     println!("{:?}", book);
 }

 /**
  * 举例来说：书分为纸质书和电子书
  如果现在正在开发一个图书管理系统，你需要描述两种书的不同属性（纸质书只有索书号，电子书只有url），你可以为枚举类成员添加元组属性描述
  */

enum Book {
    Papery(u32),
    Electronic(String),
}

fn main() {
    let book = Book::Papery(1001);
    let ebook = Book::Electronic(String::from("url://..."));
}
//如果想为属性命名，可以使用结构体语法：
enum Book {
    Papery { index: u32 },
    Electronic { url: String },
}
fn main() {
    let book = Book::Papery{index: 1001};
}
//虽然可以如此命名，但请注意，并不能像访问结构体字段一样访问枚举类绑定的属性。访问的方法在 match 语法中。

