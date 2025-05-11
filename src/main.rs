//引用依赖
use libc::{c_void, free, malloc};
use std::io;
use std::ptr;

//定义常量MAX_SIZE大小为1000
const MAX_SIZE: usize = 1000;

//使用堆对串的相关信息进行存储
struct MyString {
    p: *mut u8, //存放串的数组
    len: usize, //存放串的长度
}

//简单错误参数匹配输出错误信息函数
fn error_input(type_: i32) {
    match type_ {
        1 => println!("输入位置或请求位置不合法！"),
        2 => println!("输入长度或请求长度不合法！"),
        3 => println!("请求失败，可能的原因是输入了不存在的字符或子串！"),
        _ => return,
    }
}

//自定义比较大小函数
fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

//求子串函数（暴力）
fn query_substring(s: &MyString) {
    println!("请输入起始位置和所需长度，用空格分隔：");
    let mut input = String::new(); //定义可变参数input，记录信息
    io::stdin().read_line(&mut input).unwrap(); //从输入流中读取行，并写入input，调用unwrap简单处理错误
    let mut parts = input.split_whitespace(); //依据空格分隔信息，并返回迭代器，赋给parts
    let pos: i32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(-1); //第一个位置信息给pos，简单处理错误，若有问题，则赋-1
    let len: i32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(-1); //第二个位置信息给len，简单处理错误，若有问题，则赋-1

    //出现串的位置和长度错误
    if pos < 0 || len <= 0 || pos as usize >= s.len || (pos as usize + len as usize) > s.len {
        error_input(1); //调用自定义错误函数
        return; //返回
    }

    //继续执行
    print!("所求子串为：");
    //利用unsafe以便指针操作
    unsafe {
        //定义i遍历指针位置输出数据信息
        for i in pos as usize..(pos as usize + len as usize) {
            print!("{}", *s.p.add(i) as char);
        }
    }
    println!(); //仅换行
}

//串初始化函数
fn string_assignment(s: &mut MyString) {
    //使用unsafe进行指针操作
    unsafe {
        //指针非空，是否删除已有串
        if !s.p.is_null() {
            //输出当前已有串
            print!("当前已有数据：");
            //使用可变参数i对已有串进行遍历输出
            let mut i = 0;
            while i < s.len {
                print!("{}", *s.p.add(i) as char);
                i += 1;
            }
            println!("\n是否删除(y / n)");
            let mut opt = String::new(); //定义可变String参数opt
            io::stdin().read_line(&mut opt).unwrap(); //从输入流中读取行，并写入opt，用unwrap简单处理错误
            //删除串，则释放串
            if opt.trim() == "y" || opt.trim() == "Y" {
                free(s.p as *mut c_void); //释放串指针
                s.p = ptr::null_mut(); //初始化串指针
                s.len = 0; //初始化串长度
            }
            //不删除串，则返回
            else {
                return;
            }
        }

        //利用malloc进行内存分配，并使用panic进行简单处理
        s.p = malloc(MAX_SIZE) as *mut u8;
        if s.p.is_null() {
            panic!("内存分配失败");
        }
        //内存正常分配，输入新串并存储
        println!("请输入字符串（最大长度为1000字符）：");
        let mut input = String::new(); //定义可变String参数input
        io::stdin().read_line(&mut input).unwrap(); //从输入流中读取行，并写入input，用unwrap简单处理错误
        let input = input.split_whitespace().next().unwrap_or(""); //用不可变&str参数input覆盖原可变String参数input，并依据空格分割返回迭代器
        let bytes = input.as_bytes(); //将input转换为字节数组，并存储在bytes中
        let len = bytes.len().min(MAX_SIZE - 1); //返回bytes.len()和最大容量的较小值，并赋给len
        ptr::copy_nonoverlapping(bytes.as_ptr(), s.p, len); //将bytes指针转移给s.p
        s.len = len; //赋值串长
        *s.p.add(len) = 0; //指针偏移0
    }
}

//串比较函数
fn string_comparison(s1: &MyString, s2: &MyString) {
    let len = min(s1.len, s2.len); //定义不可变参数len
    //使用unsafe以便指针操作
    unsafe {
        //利用可变参数i遍历串指针进行串的输出
        for i in 0..len {
            let c1 = *s1.p.add(i);
            let c2 = *s2.p.add(i);
            if c1 > c2 {
                println!("第一个串大！");
                return;
            } else if c1 < c2 {
                println!("第二个串大！");
                return;
            }
        }
    }
    //判断是否存在串长不等情况，并处理
    if s1.len > s2.len {
        println!("第一个串大！");
    } else if s1.len < s2.len {
        println!("第二个串大！");
    } else {
        println!("两个串一样大！");
    }
}

//串相等函数
fn string_equality(s: &MyString) {
    //初始化比较串
    let mut s2 = MyString {
        p: ptr::null_mut(),
        len: 0,
    };
    string_assignment(&mut s2); //调用串初始化函数，对比较串进行存储
    //串长不等，直接输出，并返回
    if s.len != s2.len {
        println!("两个串不相等！");
        //利用unsafe释放指针
        unsafe {
            free(s2.p as *mut c_void);
        }
        return;
    }
    //定义可变参数equal记录是否相等
    let mut equal = true;
    //利用unsafe以便指针操作
    unsafe {
        //利用可变参数i遍历指针位置，并判断是否数据相等，不等则直接更改equal并break掉
        for i in 0..s.len {
            if *s.p.add(i) != *s2.p.add(i) {
                equal = false;
                break;
            }
        }
    }
    //利用equal判断是否相等
    if equal {
        println!("两个串相等！");
    } else {
        println!("两个串不相等！");
    }
    //利用unsafe释放指针
    unsafe {
        free(s2.p as *mut c_void);
    }
}

//求串长函数
fn string_length(s: &MyString) {
    println!("串的长度为：{}", s.len); //仅输出串的长度
}

//串匹配函数
fn string_match(s: &MyString) {
    //初始化匹配串
    let mut sub = MyString {
        p: ptr::null_mut(),
        len: 0,
    };
    string_assignment(&mut sub); //调用串初始化函数，存储匹配串
    //匹配串为0直接判断错误
    if sub.len == 0 {
        error_input(2); //调用自定义错误函数，输出错误信息
        //利用unsafe释放指针
        unsafe {
            free(sub.p as *mut c_void);
        }
        return;
    }

    //进行匹配操作
    let mut found = false; //定义可变参数found记录是否找到位置
    //利用unsafe以便指针调用
    unsafe {
        //i遍历，两串相减控制区间
        for i in 0..=(s.len - sub.len) {
            let mut j = 0;
            while j < sub.len {
                if *s.p.add(i + j) != *sub.p.add(j) {
                    break;
                }
                j += 1;
            }
            //匹配成功直接输出成功信息，并修改found，break掉for循环
            if j == sub.len {
                println!("模式匹配成功，起始位置为：{}", i);
                found = true;
                break;
            }
        }
    }
    //利用found判断匹配失败，输出失败信息
    if !found {
        println!("模式匹配失败！");
    }
    //利用unsafe释放指针
    unsafe {
        free(sub.p as *mut c_void);
    }
}

//串输出函数
fn string_print(s: &MyString) {
    //判断串是否为空，空则返回
    if s.p.is_null() {
        println!("字符串未初始化");
        return;
    }
    //使用unsafe以便指针操作
    print!("当前的串是：");
    unsafe {
        //利用可变参数i遍历串指针进行串的输出
        let mut i = 0;
        while i < s.len {
            print!("{}", *s.p.add(i) as char);
            i += 1;
        }
    }
    println!(); //仅换行
}

//串替换函数
fn string_replace(s: &mut MyString) {
    //初始化被替换串
    let mut old_sub = MyString {
        p: ptr::null_mut(),
        len: 0,
    };
    //初始化替换串
    let mut new_sub = MyString {
        p: ptr::null_mut(),
        len: 0,
    };
    
    //两次调用串初始化函数
    println!("请输入要被替换的子串：");
    string_assignment(&mut old_sub);
    println!("请输入替换后的子串：");
    string_assignment(&mut new_sub);

    //判断是否存在某串长为0的情况，若有则直接输出错误信息，并释放指针，返回
    if old_sub.len == 0 || new_sub.len == 0 {
        error_input(2);
        unsafe {
            free(old_sub.p as *mut c_void);
            free(new_sub.p as *mut c_void);
        }
        return;
    }
    
    //若没有问题，则继续进行串字串位置寻找操作
    //初始化Vec，利用其存位置
    let mut positions = Vec::new();
    //利用unsafe以便指针操作
    unsafe {
        //利用可变参数i遍历指针位置，利用相减长度来控制超限
        for i in 0..=(s.len - old_sub.len) {
            let mut j = 0;
            while j < old_sub.len {
                //不相等，break退出while
                if *s.p.add(i + j) != *old_sub.p.add(j) {
                    break;
                }
                //相等继续操作
                j += 1;
            }
            //长度相同，将该位置压入position
            if j == old_sub.len {
                positions.push(i);
            }
        }
    }

    //position为空，则进行利用自定义错误函数输出错误信息，并利用unsafe释放指针，返回
    if positions.is_empty() {
        error_input(3);
        unsafe {
            free(old_sub.p as *mut c_void);
            free(new_sub.p as *mut c_void);
        }
        return;
    }

    //若position不为空，则进行替换操作
    let new_len =
        s.len as isize + (new_sub.len as isize - old_sub.len as isize) * positions.len() as isize; //更新串长度
    //若串长超限，则输出错误
    if new_len < 0 || new_len as usize >= MAX_SIZE {
        error_input(2); //利用自定义错误函数输出错误信息
        //利用unsafe释放指针，并返回
        unsafe {
            free(old_sub.p as *mut c_void);
            free(new_sub.p as *mut c_void);
        }
        return;
    }

    //利用unsafe新串内存分配
    let new_p = unsafe { malloc(MAX_SIZE) as *mut u8 };
    if new_p.is_null() {
        panic!("内存分配失败");
    }

    //进行替换操作
    let mut new_idx = 0;
    let mut last_pos = 0;
    //利用unsafe以便于操作指针
    unsafe {
        for &pos in &positions {
            let copy_len = pos - last_pos;
            ptr::copy_nonoverlapping(s.p.add(last_pos), new_p.add(new_idx), copy_len);
            new_idx += copy_len;
            ptr::copy_nonoverlapping(new_sub.p, new_p.add(new_idx), new_sub.len);
            new_idx += new_sub.len;
            last_pos = pos + old_sub.len;
        }
        ptr::copy_nonoverlapping(s.p.add(last_pos), new_p.add(new_idx), s.len - last_pos);
        new_idx += s.len - last_pos;
        *new_p.add(new_idx) = 0;
    }

    //利用unsafe释放原串指针
    unsafe {
        free(s.p as *mut c_void);
    }
    //将原串更新为新串
    s.p = new_p;
    s.len = new_idx;

   //利用unsafe释放新串指针
    unsafe {
        free(old_sub.p as *mut c_void);
        free(new_sub.p as *mut c_void);
    }
}

fn main() {
    //初始化串
    let mut str = MyString {
        p: ptr::null_mut(), //初始化串指针
        len: 0,             //初始化串长度
    };

    //进入loop循环，直到panic或者break
    loop {
        //输出用户界面初始面板
        println!("\n欢迎使用“串基本操作”演示系统！");
        println!("=====请选择你要进行的下一步操作=====");
        println!("[1] 串赋值————给当前串赋值");
        println!("[2] 串输出————输出当前串");
        println!("[3] 串替换————给当前串的替换");
        println!("[4] 串比较————与当前串的比较");
        println!("[5] 串相等————与当前串是否相等");
        println!("[6] 串匹配————与当前串的模式匹配");
        println!("[7] 求子串————求当前串的子串");
        println!("[8] 求串长————获取当前串的长度");
        println!("=========退出系统输入0即可=========");

        //使用可变参数input对输入参数进行读取
        let mut input = String::new(); //定义可变String参数input，并对其进行初始化操作
        io::stdin().read_line(&mut input).unwrap(); //从输入流读取行并写入参数input中，使用unwrap()进行简单错误处理
        let opt = input.trim().parse().unwrap_or(-1); //解析参数input，使用unwrap_or()进行简单错误处理，无法解析，则默认为-1

        match opt {
            //当opt为0时退出演示系统
            0 => {
                println!("欢迎再次使用串基本操作演示系统，再会！");
                //使用unsafe对str的指针进行释放操作
                unsafe {
                    //当str为非空，释放掉
                    if !str.p.is_null() {
                        free(str.p as *mut c_void);
                    }
                }
                //释放结束，退出该系统
                return;
            }
            //当opt为1时进行串赋值操作
            1 => {
                string_assignment(&mut str); //调用串初始化函数
                string_print(&str); //调用串输出函数
            }
            //当opt为2时进行串输出操作
            2 => {
                string_print(&str); //调用串输出函数
            }
            //当opt为3时进行串替换操作
            3 => {
                string_replace(&mut str); //调用串替换函数
                string_print(&str); //调用串输出函数
            }
            //当opt为4时进行串比较操作
            4 => {
                //初始化比较串
                let mut str2 = MyString {
                    p: ptr::null_mut(), //初始化比较串指针
                    len: 0,             //初始化比较串长度
                };
                string_assignment(&mut str2); //调用串初始化函数
                string_comparison(&str, &str2); //调用串比较函数
                //使用unsafe对比较串的指针进行释放操作
                unsafe {
                    //因为比较串为非空，故不判定，直接释放
                    free(str2.p as *mut c_void);
                }
            }
            //当opt为5时进行串相等操作
            5 => string_equality(&str), //调用串相等函数
            //当opt为6时进行串匹配操作
            6 => string_match(&str), //调用串匹配函数
            //当opt为7时进行求子串操作
            7 => query_substring(&str), //调用求子串函数
            //当opt为8时进行求串长操作
            8 => string_length(&str), //调用求串长函数
            //当opt为非0~8时进行重新选择功能操作
            _ => println!("当前操作不合法，请重新选择功能"),
        }
    }
}
