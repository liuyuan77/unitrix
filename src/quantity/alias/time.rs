/// Time (second) / 时间 (秒)
use super::{Quantity, prefix::*};
use crate::dimension::Second;

/// Generic time with SI prefixes / 带SI前缀的通用时间
pub type Time<V, P = NoPrefix> = Quantity<V, SI<P, Dimension<Z0, Z0, P1, Z0, Z0, Z0, Z0>>>;

/// Seconds / 秒
pub type Seconds<V = i64> = Time<V, NoPrefix>;
impl<V> Seconds<V> {    
    /// Create from days, hours, minutes and seconds
    pub fn from_dhms(days: V, hours: V, minutes: V, seconds: V) -> Self{        
        Seconds::new(seconds+minutes*60+hours*3600+days*86400)
    }
}

/// kiloseconds / 千秒
pub type Kiloseconds<V> = Quantity<V, Kilo, Second>;

/// Milliseconds / 毫秒
pub type Milliseconds<V> = Time<V, Milli>;

/// Microseconds / 微秒
pub type Microseconds<V> = Time<V, Micro>;

/// Nanoseconds / 纳秒
pub type Nanoseconds<V> = Time<V, Nano>;
/// Minutes / 分钟
pub type Minutes<V = i64> = Time<V, Minute>;

/// Hours / 小时
pub type Hours<V = i64> = Time<V, Hour>;

/// Days / 天
pub type Days<V = i64> = Time<V, Day>;




/// 处理时间单位的符号
    fn time_symbol() -> Option<&'static str> {
        type One = Ratio<Z0, Z0, Z0>;
        type Minute = Ratio<P1, P1, P2>;  // 60 = 5^1 * 3^1 * 2^2
        type Hour = Ratio<P2, P2, P2>;    // 3600 = 5^2 * 3^2 * 2^2
        type Day = Ratio<P2, P3, P7>;     // 86400 = 5^2 * 3^3 * 2^7

        if TypeId::of::<Self>() == TypeId::of::<Minute>() {
            Some("min")
        } else if TypeId::of::<Self>() == TypeId::of::<Hour>() {
            Some("h")
        } else if TypeId::of::<Self>() == TypeId::of::<Day>() {
            Some("d")
        } else if TypeId::of::<Self>() == TypeId::of::<One>() {
            Some("")
        } else {
            None
        }
    }


    // ========== 时间单位定义 ==========

/// 分钟 (60秒) = 5^1 * 3^1 * 2^2
pub type Minute = Ratio<P1, P1, P2>;

/// 小时 (3600秒) = 5^2 * 3^2 * 2^2
pub type Hour = Ratio<P2, P2, P2>;

/// 天 (86400秒) = 5^2 * 3^3 * 2^7
pub type Day = Ratio<P2, P3, P7>;

