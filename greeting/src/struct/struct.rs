/**
 * Rust中的结构体（Struct）与元组（Tuple）都可以将若干个类型不一定相同的数据捆绑在一起形成整体，但结构体的每个成员和其本身都有一个名字，这样访问它成员的时候就不用记住下标了。
 * 元组常用于非定义的多值传递，而结构体用于规范常用的数据结构。结构体的每个成员叫做“字段”
 */

//这是一个结构体的定义：
struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32
}
//注意：如果你常用C/C++，请记住在Rust里Struct语句仅用来定义，不能声明实例，结尾不需要;符号，而且每个字段定义之后用，分隔。

//Rust很多地方受javaScript影响，在实例化结构体的时候用JSON对象的key:value语法来实现定义：
fn main() {
    let runoob = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB" ),
        nation: String::from("China"),
        found: 2013
    };
}
//如果你不了解JSON对象，你可以不用管它，记住格式就可以了：
// 结构体类名 {
//     字段名 : 字段值,
//     ...
// }

//这样的好处是不仅使程序更加直观，还不需要按照定义的顺序来输入成员变量的值。
//如果正在实例化的结构体有字段名称和现存变量名称一样的，可以简化书写：
fn main() {
    let domain = String::from("www.runoob.com");
    let name = String::from("RUNOOB");
    let runoob = Site {
        domain,  // 等同于 domain : domain,
        name,    // 等同于 name : name,
        nation: String::from("China"),
        traffic: 2013
    };

    let site = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        ..runoob
    };
}
//有一种情况：你想要新建一个结构体实例，其中大部分属性需要被设置与现存的一个结构体一样，仅需要更改其中一两个字段，可以试用结构体更新语法
//注意：..runoob后面不可以有逗号，这种语法不允许一成不变的复制另一个结构体实例，意思就是说至少重新设定一个字段的值才能引用其他实例的值

/**
 * 元组结构体
 * 有一种更简单的定义和使用结构体的方式：元组结构体
 * 元组结构体是一种形式是元组的结构体。
 * 与元组的区别是它有名字和固定的类型格式。它存在的意义1是为了处理哪些需要定义类型（经常使用）又不想太复杂的简单数据：
 */
struct Color(u8, u8, u8);
struct Point(f64, f64);

fn main() {
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);
}
/**
 * 颜色和点坐标是常用的两种数据类型，但如果实例化时写个大括号再写上两个名字就为了可读性牺牲了便捷性，Rust不会遗留这个问题。元组结构体对象的使用方式和元组一样，通过.和下标来进行访问
 */

/**
 * 结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段。
 * 这就是为什么本章的案例使用了String类型而不适用&str的原因。
 * 但这不意味着结构体中不定义引用型字段，这需要通过“生命周期”机制来实现
 */


//调试中，完整的显示出一个结构体实例是非常有用的，但如果我们手动的书写一个格式会非常的不方便，所以Rust提供了一个方便的输出一整个结构体的方法：
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
}
//如第一行所示：一定要导入调试库 #[derive(Debug)] ，之后在 println 和 print 宏中就可以用 {:?} 占位符输出一整个结构体：
//如果属性较多的话可以h使用另一个占位符{:#?}

/**
 * 方法和函数类似，只不过它是用来操作结构体实例的。
 * 如果学习过一些面向对象的语言，那你一定很清楚函数一般放在类定义里并在函数中用this表示所操作的实例。
 * Rust语言不是面向x对象的，从它所有权机制的创新可以看出这一点。但是面向对象的思想可以在Rust中实现。
 * 结构体方法的第一个参数必须是&self，不需声明类型，因为self不是一种风格而是关键字。
 */
struct Rectangle {
    width: u32,
    height: u32,
}
   
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1's area is {}", rect1.area());
}
//请注意，在调用结构体方法的时候不需要填写self，这是出于对使用方便性的考虑。

//一个多参数的例子：
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 40, height: 20 };

    println!("{}", rect1.wider(&rect2));
}

/**
 * 结构体关联函数
 * 之所以“结构体方法”不叫“结构体函数”是因为“函数”这个名字留给了这种函数：它在impl块中却没有&self参数。
 * 这种函数不依赖实例，但是使用它需要声明是在哪个impl块中的。
 * 一直使用String:from("")函数就是一个关联函数_
 */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let rect = Rectangle::create(30, 50);
    println!("{:?}", rect);
}
//注：结构体impl块可以多写几次，效果相当于它们内容的拼接。

/**
 * 结构体可以只作为一种象征而无需任何成员：
 */
struct UnitStruct;
//我们称这种没有身体的结构体为单元结构体（Unit Struct）。
