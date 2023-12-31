计算机程序必须i咋运行时管理它们所使用的的内存资源。
大多数编程语言都有管理内存的能力：
C/C++这样的1语言主要通过手动方式管理内存，开发者需要手动的申请和释放内存资源，单位了提高开发效率，只要i不影响程序功能的实现，许多开发者没有及时释放内存的习惯，所以手动管理内存的方式常常造成in资源浪费。
java语言编写的虚拟机（JVM）中运行，JVM具备自动回收内存n资源的功能。但这种方式常常会降低运行时效率，所以JVM会尽可能少的回收资源，这样也会使程序占用较大的内存资源。
所有权对大多数开发者而言是一个新颖的概念，它是Rust语言为高效使用内存而设计的语法机制,。n所有权概念是为了让Rust在编译阶段更有效的分析内存资源的有用性以及实现内存管理而诞生的概念。

所有权有以下三条规则：
1.Rust中的每个值都有一个变量，称为其所有者。
2.一次只能有一个所有者。
3.当所有者不在程序运行范围时，该值将被删除。
这三条规则是所有权概念的基础。

变量范围
概念：
{
    // 在声明以前，变量 s 无效
    let s = "runoob";
    // 这里是变量 s 的可用范围
}
// 变量范围已经结束，变量 s 无效

变量范围是变量是变量的一个属性，其代表变量的可行域，默认从声明变量开始有效直到变量所在域结束。

内存和分配
如果我们定义一个变量并给它赋予了一个值，这个变量的值存在于内存中。这种情况很普遍，但如果我们需要存储的数据长度不确定（比如用户输入的一串字符串），我们就无法在定义时明确数据长度，也就无法在编译阶段令程序分配固定长度的内存控件供数据存储使用。
这就需要提供一种在程序运行时程序自己社情使用内存的机制——堆。“内存资源”几乎都是值堆所占用的内存空间。
有分配就有释放，程序不能一直占用某个内存资源。因此决定资源是否浪费的关键因素就是资源有没有及时的释放。

Rust中没有调用free函数来释放字符串s的资源，Rust之所以没有释放的步骤是因为在变量范围结束的时候，Rust编译器自动添加了调用释放资源函数的步骤。
这种机制看似很简单了：它不过是帮助了程序员在适当的i地方那个添加了一个释放资源的函数调用而已。但这种简单的极致可以有效的解决一个史上最令程序员头疼的编程问题。

变量和数据交互的方式
变量与数据交互方式主要有移动（Move）和克隆（Clone）两种：
移动
多个变量可以在Rust中以不同的方式与相同的数据交互：
let x = 5;
let y = x;
上面的程序将值5绑定到变量x，然后将x的值复制并复制给变量y。现在栈中将有两个值5。此情况h总的数据是“基本数据”类型的数据，不需要存储到堆中，仅在占中的数据“移动”方式是直接复制，这不会花费更长的时间或更多的存储空间。“基本数据”类型有这些：
所有整数类型，例如 i32 、 u32 、 i64 等。
布尔类型 bool，值为 true 或 false 。
所有浮点类型，f32 和 f64。
字符类型 char。
仅包含以上类型数据的元组（Tuples）。

但如果发生交互的数据在堆中就是另外一种情况：
let s1 = String::from("hello");
let s2 = s1;
第一步产生了一个String对象，值为“hello”，其中“hello”可以认为是类似于长度不确定的数据，需要在堆中存储。
两个String对象在栈中，每个String对象都有一个指针指向堆中的“hello”字符串。在给s2赋值时，只有栈中的数据被复制了，堆中的字符串依然还是原来的字符串。

前面我们说过，当变量超出范围时，Rust自动调用释放资源函数并清理该变量的堆内存。但是s1和s2都被释放的话堆区中的“hello”被释放两次，这是不被系统允许的。为了确保安全，在给s2赋值时s1已经无效了
没错，在把s1的值给s2以后s1将不可以再被使用。
let s1 = String::from("hello");
let s2 = s1; 
println!("{}, world!", s1); // 错误！s1 已经失效

克隆
Rust会尽可能地降低程序的运行成本，所以默认情况下，长度较大的数据存放在堆中，且采用移动的方式进行数据交互。但如果需要将数据单纯的复制一份以供他用，可以使用数据的第二种交互方式——克隆。
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
这里是真的将堆中“hello”复制了一份，所以s1和s2都分别绑定了一个值，释放的时候也会被当做两个资源。
当然，克隆仅在需要复制的情况下使用，毕竟复制数据需要花费更多的时间。
