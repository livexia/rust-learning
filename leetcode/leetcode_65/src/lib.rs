/**
有效数字（按顺序）可以分成以下几个部分：

一个 小数 或者 整数
（可选）一个 'e' 或 'E' ，后面跟着一个 整数
小数（按顺序）可以分成以下几个部分：

（可选）一个符号字符（'+' 或 '-'）
下述格式之一：
至少一位数字，后面跟着一个点 '.'
至少一位数字，后面跟着一个点 '.' ，后面再跟着至少一位数字
一个点 '.' ，后面跟着至少一位数字
整数（按顺序）可以分成以下几个部分：

（可选）一个符号字符（'+' 或 '-'）
至少一位数字
部分有效数字列举如下：

["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"]
部分无效数字列举如下：

["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"]
给你一个字符串 s ，如果 s 是一个 有效数字 ，请返回 true 。

示例 1：
输入：s = "0"
输出：true

示例 2：
输入：s = "e"
输出：false

示例 3：
输入：s = "."
输出：false

示例 4：
输入：s = ".1"
输出：true

提示：
1 <= s.length <= 20
s 仅含英文字母（大写和小写），数字（0-9），加号 '+' ，减号 '-' ，或者点 '.' 。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/valid-number
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn is_number(s: String) -> bool {
    s.chars()
        .try_fold(State::new(), State::handle)
        .as_ref()
        .map_or(false, State::is_valid)
}

enum State {
    Start,
    Sign,
    Integer,
    Dot,
    EmptyDot,
    Decimal,
    E,
    ExpSign,
    Exponent,
    End,
}

type Result = std::result::Result<State, ()>;

impl State {
    pub fn new() -> Self {
        State::Start
    }

    pub fn is_valid(&self) -> bool {
        use State::*;
        match self {
            Start | Sign | E | ExpSign | EmptyDot => false,
            _ => true,
        }
    }

    pub fn handle(self, c: char) -> Result {
        use State::*;
        match self {
            Start => match c {
                ' ' => Ok(Start),
                '+' | '-' => Ok(Sign),
                '0'..='9' => Ok(Integer),
                '.' => Ok(EmptyDot),
                _ => Err(())
            }
            Sign => match c {
                '0'..='9' => Ok(Integer),
                '.' => Ok(EmptyDot),
                _ => Err(())
            }
            Integer => match c {
                '0'..='9' => Ok(Integer),
                '.' => Ok(Dot),
                'e' | 'E' => Ok(E),
                ' ' => Ok(End),
                _ => Err(()),
            }
            EmptyDot => match c {
                '0'..='9' => Ok(Decimal),
                _ => Err(()),
            }
            Dot => match c {
                '0'..='9' => Ok(Decimal),
                'e' | 'E' => Ok(E),
                ' ' => Ok(End),
                _ => Err(()),
            }
            Decimal => match c {
                '0'..='9' => Ok(Decimal),
                'e' | 'E' => Ok(E),
                ' ' => Ok(End),
                _ => Err(()),
            }
            E => match c {
                '+' | '-' => Ok(ExpSign),
                '0'..='9' => Ok(Exponent),
                _ => Err(()),
            }
            ExpSign => match c {
                '0'..='9' => Ok(Exponent),
                _ => Err(()),
            }
            Exponent => match c {
                '0'..='9' => Ok(Exponent),
                ' ' => Ok(End),
                _ => Err(()),
            }
            End => match c {
                ' ' => Ok(End),
                _ => Err(()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
