//引用是C++开发者比较熟悉的概念。
//如果你熟悉指针，可以把它当作一种指针
//实际上“引用”是变量的间接访问方式
fn main() {
    let s1 = String::from("hello");
    let s2 = &s1;
    println("s1 is {}, s2 is {}", s1, s2);
}
/**
 * 当一个变量的值被引用时，变量本身不会被认定无效。因为“引用”并没有在栈中复制变量的值
 */


//函数传参的道理也一样
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println("the length of '{}' is {}.", s1, len);
}

fn calculate_length(s:String) -> usize {
    s.len()
}

/**
 * 引用不会获得值的所有权
 * 引用只能租借（borrow）值的所有权
 * 引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，但引用不具有所指值的所有权
 */
fn main() {
    let s1 = String::from("hello");
    let mut s2 = &s1;
    let s3 = s1;
    s2 = &s3; //重新从s3租借所有权
    println("{}", s2);
}
/**
 * 这段程序是正确的，
 * 既然引用不具有所有权，即使它租借了所有权，它也只享有使用权（这跟租房子一个道理）
 * 如果尝试利用租借的权利去修改数据会被阻止
 */
fn main() {
    let s1 = String::from("hello");
    let s2 = & s1;
    println("{}", s2);
    s2.push_str("oob"); //错误，禁止修改租借的值
    println("{}", s2);
}


/**
 * 这段程序中s2尝试修改s1的值被阻止，租借的所有权不能修改所有者的值。
 * 当然，也存在一种可变的租借方式，就像你租一个房子，如果物业规定房主可以修改房子结构，房主在租借时也在合同中声明赋予了你这种权利，你是可以重新装修房子的
 */
fn main() {
    let mut s1 = String::from("run");
    // s1 是可变的

    let s2 = &mut s1;
    // s2 是可变的引用

    s2.push_str("oob");
    println!("{}", s2);
}

/**
 * 这段程序就没有问题了。我们用 &mut 修饰可变的引用类型。
 * 可变引用与不可变引用相比除了权限不同以外，可变引用不允许多重引用，但不可变引用可以：
 */
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
}
/**
 * 这段程序不正确，因为多重可变引用了 s。
 * Rust 对可变引用的这种设计主要出于对并发状态下发生数据访问碰撞的考虑，在编译阶段就避免了这种事情的发生。
 * 由于发生数据访问碰撞的必要条件之一是数据被至少一个使用者写且同时被至少一个其他使用者读或写，所以在一个值被可变引用时不允许再次被任何引用。
 */

/**
 * 垂悬引用（Dangling References）
 * 这是一个换了个名字的概念，如果放在有指针概念的编程语言里它就指的是那种没有实际指向一个真正能访问的数据的指针（注意，不一定是空指针，还有可能是已经释放的资源）。它们就像失去悬挂物体的绳子，所以叫"垂悬引用"。
 * "垂悬引用"在 Rust 语言里不允许出现，如果有，编译器会发现它。
 * 下面是一个垂悬的典型案例：
 */
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
/**
 * 很显然，伴随着 dangle 函数的结束，其局部变量的值本身没有被当作返回值，被释放了。但它的引用却被返回，这个引用所指向的值已经不能确定的存在，故不允许其出现。
 */