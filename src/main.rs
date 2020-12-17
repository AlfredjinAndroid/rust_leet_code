fn main() {
    // let result = two_sum(vec![2, 7, 11, 15], 9);
    // print!("result : {:?}", result);

    // let result = roman_to_int(String::from("MCMXCIV"));
    // println!("result : {}",result);

    
    let result = longest_common_prefix(vec!["dog".to_string(),"racecar".to_string(),"car".to_string()]);
    println!("result : {}",result);
}

/*


===============================================================================================================

===============================================================================================================

1. 两数之和

给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。



===============================================================================================================

*/

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    return vec![];
}

/*


===============================================================================================================

===============================================================================================================

7. 整数反转
给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。


===============================================================================================================


*/

pub fn reverse(x: i32) -> i32 {
    if x >= std::i32::MAX {
        return 0;
    }

    if x <= std::i32::MIN {
        return 0;
    }

    let mut tmp = x.abs();
    let mut result: i32 = 0;

    while tmp > 0 {
        let (mul, bool) = result.overflowing_mul(10);
        if bool {
            return 0;
        }
        let (add, bool) = mul.overflowing_add(tmp % 10);
        if bool {
            return 0;
        }
        result = add;
        tmp = tmp / 10;
    }

    if x < 0 {
        result = result * -1;
    }

    if result >= std::i32::MAX || result <= std::i32::MIN {
        return 0;
    }

    return result;
}

/*



===============================================================================================================

===============================================================================================================

9. 回文数
判断一个整数是否是回文数。回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。




===============================================================================================================

*/
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut tmp = x;
    let mut new_number = 0;

    while tmp > 0 {
        new_number = new_number * 10 + tmp % 10;
        tmp = tmp / 10;
    }

    return new_number == x;
}

/*

===============================================================================================================

===============================================================================================================

13. 罗马数字转整数
罗马数字包含以下七种字符: I， V， X， L，C，D 和 M。
字符          数值
I             1
V             5
X             10
L             50
C             100
D             500
M             1000



===============================================================================================================

*/
pub fn roman_to_int(s: String) -> i32 {
    if s.len() <= 0 {
        return 0;
    }

    let mut result = 0;
    let mut last = 0;
    for c in s.chars().rev() {
        let current = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        let add_or_mut = if last > 0 && last > current { -1 } else { 1 };
        last = current;
        result = result + (last * add_or_mut);
    }

    return result;
}

/*

===============================================================================================================

===============================================================================================================

*/


/*
===============================================================================================================


14. 最长公共前缀

编写一个函数来查找字符串数组中的最长公共前缀。

如果不存在公共前缀，返回空字符串 ""。

===============================================================================================================

*/

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() <= 0 {
         return String::from("");
    }
    let mut prefix = strs[0].clone();

    for item in strs {
        if prefix.len() <= 0 {
            return "".to_string()
        };
        while prefix.len() > 0 {
            if !item.starts_with(&prefix){
                prefix = String::from(&prefix[..(prefix.len()-1)]);
            } else {
                break;
            }
        }
    }

    return prefix;
}


/*

===============================================================================================================

===============================================================================================================

*/