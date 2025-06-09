/// A unit type representing a null value.
/// 表示空值的单元类型
///
/// This type is useful for places where a concrete type is required
/// but no actual data needs to be stored.
/// 当需要具体类型但实际上不需要存储数据时，此类型非常有用
///
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct Null;

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Test the default value of Null
    /// 测试 Null 的默认值
    #[test]
    fn test_null_default() {
        assert_eq!(Null::default(), Null);
    }
    
    /// Test ordering comparison of Null
    /// 测试 Null 的排序比较
    #[test]
    fn test_null_ordering() {
        assert!(Null <= Null);
        assert!(Null >= Null);
        assert!(!(Null < Null));
        assert!(!(Null > Null));
    }
}