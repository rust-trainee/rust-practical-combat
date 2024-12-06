/// # 定点数格式
/// 除了用浮点数格式 还可以用定点数格式来表示小数
/// 定点数可以用来表示分数,并且是在没有浮点单元的 CPU 上进行计算的一种选择.
/// 与浮点数不同的是,定点数不会位了适应不同的表示范围而移动小数点的位置.
///
/// Q 格式,是一种使用单个字节的定点数格式.叫做 Q7,是 Q 格式的一个特定的版本,它的名字叫做 Q7 用7
/// 位表示数字,然后加1 位用于符号位.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Q7(i8);

impl From<f64> for Q7 {
    fn from(n: f64) -> Self {
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-128)
        } else {
            Q7((n * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(value: Q7) -> Self {
        (value.0 as f64) * 2_f64.powf(-7.0)
    }
}

impl From<f32> for Q7 {
    fn from(value: f32) -> Self {
        Q7::from(value as f64)
    }
}

impl From<Q7> for f32 {
    fn from(value: Q7) -> Self {
        f64::from(value) as f32
    }
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.), Q7::from(1.));
        assert_eq!(Q7::from(-10.), Q7::from(-1.));
    }
    #[test]
    fn f32_to_q7() {
        let n1: f32 = 0.7;
        let q1 = Q7::from(n1);

        let n2 = -0.4;
        let q2 = Q7::from(n2);

        let n3 = 123.0;
        let q3 = Q7::from(n3);

        assert_eq!(q1, Q7(89));
        assert_eq!(q2, Q7(-51));
        assert_eq!(q3, Q7(127));
    }
    #[test]
    fn q7_to_f32() {
        let q1 = Q7::from(0.7);
        let n1 = f32::from(q1);
        assert_eq!(n1, 0.6953125);

        let q2 = Q7::from(n1);
        let n2 = f32::from(q2);
        assert_eq!(n1, n2);
    }
}
