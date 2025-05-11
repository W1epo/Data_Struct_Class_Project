use libc::{c_void, free, malloc};
use std::io;
use std::ptr;

const MAX_SIZE: usize = 1000;

struct MyString {
    p: *mut u8,
    len: usize,
}

fn match_error(type_: i32) {
    match type_ {
        1 => println!("输入位置或请求位置不合法！"),
        2 => println!("输入长度或请求长度不合法！"),
        _ => println!("请求失败，可能的原因是输入了不存在的字符或子串！"),
    }
}

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn print_string(s: &MyString) {
    if s.p.is_null() {
        println!("字符串未初始化");
        return;
    }
    print!("您输入的串是：");
    unsafe {
        let mut i = 0;
        while i < s.len {
            print!("{}", *s.p.add(i) as char);
            i += 1;
        }
    }
    println!();
}

fn string_length(s: &MyString) -> usize {
    s.len
}

fn string_assignment(s: &mut MyString) {
    unsafe {
        if !s.p.is_null() {
            print!("当前已有数据：");
            let mut i = 0;
            while i < s.len {
                print!("{}", *s.p.add(i) as char);
                i += 1;
            }
            println!("\n是否清除(y / n)");
            let mut opt = String::new();
            io::stdin().read_line(&mut opt).unwrap();
            if opt.trim() == "y" {
                free(s.p as *mut c_void);
                s.p = ptr::null_mut();
                s.len = 0;
            } else {
                return;
            }
        }

        s.p = malloc(MAX_SIZE) as *mut u8;
        if s.p.is_null() {
            panic!("内存分配失败");
        }
        println!("请输入字符串（1000字符以内）：");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.split_whitespace().next().unwrap_or("");
        let bytes = input.as_bytes();
        let len = bytes.len().min(MAX_SIZE - 1);
        ptr::copy_nonoverlapping(bytes.as_ptr(), s.p, len);
        s.len = len;
        *s.p.add(len) = 0;
    }
}

fn char_replace(s: &mut MyString) {
    if s.p.is_null() {
        println!("字符串未初始化");
        return;
    }
    println!("请输入要被替换的字符和替换后的字符，用空格分隔：");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut chars = input.trim().split_whitespace();
    let old = chars.next().and_then(|s| s.chars().next()).unwrap_or('\0');
    let new = chars.next().and_then(|s| s.chars().next()).unwrap_or('\0');

    unsafe {
        for i in 0..s.len {
            let c = *s.p.add(i) as char;
            if c == old {
                *s.p.add(i) = new as u8;
            }
        }
    }
}

fn string_comparison(s1: &MyString, s2: &MyString) {
    let len = min(s1.len, s2.len);
    unsafe {
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
    if s1.len > s2.len {
        println!("第一个串大！");
    } else if s1.len < s2.len {
        println!("第二个串大！");
    } else {
        println!("两个串一样大！");
    }
}

fn judge_equality(s: &MyString) {
    let mut s2 = MyString {
        p: ptr::null_mut(),
        len: 0,
    };
    string_assignment(&mut s2);
    if s.len != s2.len {
        println!("两个串不相等！");
        unsafe {
            free(s2.p as *mut c_void);
        }
        return;
    }
    let mut equal = true;
    unsafe {
        for i in 0..s.len {
            if *s.p.add(i) != *s2.p.add(i) {
                equal = false;
                break;
            }
        }
    }
    if equal {
        println!("两个串相等！");
    } else {
        println!("两个串不相等！");
    }
    unsafe {
        free(s2.p as *mut c_void);
    }
}

fn query_substring(s: &MyString) {
    println!("请输入起始位置和所需长度，用空格分隔：");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let pos: i32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(-1);
    let len: i32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(-1);

    if pos < 0 || len <= 0 || pos as usize >= s.len || (pos as usize + len as usize) > s.len {
        match_error(1);
        return;
    }

    print!("所求子串为：");
    unsafe {
        for i in pos as usize..(pos as usize + len as usize) {
            print!("{}", *s.p.add(i) as char);
        }
    }
    println!();
}

fn pattern_marching(s: &MyString) {
    let mut sub = MyString {
        p: ptr::null_mut(),
        len: 0,
    };
    string_assignment(&mut sub);
    if sub.len == 0 {
        match_error(2);
        unsafe {
            free(sub.p as *mut c_void);
        }
        return;
    }

    let mut found = false;
    unsafe {
        for i in 0..=(s.len - sub.len) {
            let mut j = 0;
            while j < sub.len {
                if *s.p.add(i + j) != *sub.p.add(j) {
                    break;
                }
                j += 1;
            }
            if j == sub.len {
                println!("模式匹配成功，起始位置为：{}", i);
                found = true;
                break;
            }
        }
    }
    if !found {
        println!("模式匹配失败！");
    }
    unsafe {
        free(sub.p as *mut c_void);
    }
}

fn string_replace(s: &mut MyString) {
    let mut old_sub = MyString {
        p: ptr::null_mut(),
        len: 0,
    };
    let mut new_sub = MyString {
        p: ptr::null_mut(),
        len: 0,
    };

    println!("请输入要被替换的子串：");
    string_assignment(&mut old_sub);
    println!("请输入替换后的子串：");
    string_assignment(&mut new_sub);

    if old_sub.len == 0 || new_sub.len == 0 {
        match_error(2);
        unsafe {
            free(old_sub.p as *mut c_void);
            free(new_sub.p as *mut c_void);
        }
        return;
    }

    let mut positions = Vec::new();
    unsafe {
        for i in 0..=(s.len - old_sub.len) {
            let mut j = 0;
            while j < old_sub.len {
                if *s.p.add(i + j) != *old_sub.p.add(j) {
                    break;
                }
                j += 1;
            }
            if j == old_sub.len {
                positions.push(i);
            }
        }
    }

    if positions.is_empty() {
        match_error(3);
        unsafe {
            free(old_sub.p as *mut c_void);
            free(new_sub.p as *mut c_void);
        }
        return;
    }

    let new_len =
        s.len as isize + (new_sub.len as isize - old_sub.len as isize) * positions.len() as isize;
    if new_len < 0 || new_len as usize >= MAX_SIZE {
        match_error(2);
        unsafe {
            free(old_sub.p as *mut c_void);
            free(new_sub.p as *mut c_void);
        }
        return;
    }

    let new_p = unsafe { malloc(MAX_SIZE) as *mut u8 };
    if new_p.is_null() {
        panic!("内存分配失败");
    }

    let mut new_idx = 0;
    let mut last_pos = 0;
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

    unsafe {
        free(s.p as *mut c_void);
    }
    s.p = new_p;
    s.len = new_idx;

    unsafe {
        free(old_sub.p as *mut c_void);
        free(new_sub.p as *mut c_void);
    }
}

fn main() {
    let mut str = MyString {
        p: ptr::null_mut(),
        len: 0,
    };
    loop {
        println!("\n欢迎进入串基本操作演示系统");
        println!("=====请选择你要进行的下一步操作=====");
        println!("[1] 串赋值————给当前串赋值");
        println!("[2] 求串长————获取当前串的长度");
        println!("[3] 串替换————给当前串的替换");
        println!("[4] 串比较————与当前串的比较");
        println!("[5] 串相等————与当前串是否相等");
        println!("[6] 求子串————求当前串的子串");
        println!("[7] 串匹配————与当前串的模式匹配");
        println!("[8] 串输出————输出当前串");
        println!("========退出系统输入0即可========");
        // println!("\n欢迎进入 串基本操作演示系统 ");
        // println!("=====请选择要进行的操作=====\n");
        // println!("[0] 退出系统");
        // println!("[1] 串赋值");
        // println!("[2] 获取串长度");
        // println!("[3] 串替换");
        // println!("[4] 串比较");
        // println!("[5] 判断相等");
        // println!("[6] 求子串");
        // println!("[7] 串的模式匹配");
        // println!("\n============================");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let opt = input.trim().parse().unwrap_or(-1);

        match opt {
            0 => {
                println!("感谢使用串基本操作演示系统");
                unsafe {
                    if !str.p.is_null() {
                        free(str.p as *mut c_void);
                    }
                }
                return;
            }
            1 => {
                string_assignment(&mut str);
                print_string(&str);
            }
            2 => println!("串的长度为：{}", str.len),
            3 => {
                println!("[1] 将某一字符全部替换为其他字符");
                println!("[2] 替换子串");
                let mut sub_opt = String::new();
                io::stdin().read_line(&mut sub_opt).unwrap();
                match sub_opt.trim().parse() {
                    Ok(1) => char_replace(&mut str),
                    Ok(2) => string_replace(&mut str),
                    _ => println!("无效选项"),
                }
                print_string(&str);
            }
            4 => {
                let mut str2 = MyString {
                    p: ptr::null_mut(),
                    len: 0,
                };
                string_assignment(&mut str2);
                string_comparison(&str, &str2);
                unsafe {
                    free(str2.p as *mut c_void);
                }
            }
            5 => judge_equality(&str),
            6 => query_substring(&str),
            7 => pattern_marching(&str),
            _ => println!("当前操作不合法，请重新选择功能"),
        }
    }
}
