// src/testing.rs
#[cfg(test)]
mod tests {
    use crate::evaluator::evaluate;
    use crate::player::Player;

    fn dummy_player() -> Player {
        let mut stats = std::collections::HashMap::new();
        stats.insert("Spd".to_string(), 10);
        stats.insert("Str".to_string(), 20);
        stats.insert("QB".to_string(), 60);
        stats.insert("HB".to_string(), 55);
        stats.insert("BrB".to_string(), 11);
        stats.insert("Ddg".to_string(), 30);
        stats.insert("Blk".to_string(), 15);
        stats.insert("Tck".to_string(), 60);
        stats.insert("Dur".to_string(), 50);
        stats.insert("Stm".to_string(), 40);
        Player { name: "Dummy".into(), stats }
    }

    #[test]
    fn test_whitespace_stripping() {
        let p = dummy_player();
        assert_eq!(evaluate(&p, " 2 +  3 *  4 ").unwrap(), 14.0);
        assert_eq!(evaluate(&p, " ( 2 + 3 ) * 4 ").unwrap(), 20.0);
        assert_eq!(evaluate(&p, " QB  +   HB ").unwrap(), 115.0);
    }

    #[test]
    fn test_arithmetic_and_precedence() {
        let p = dummy_player();
        assert_eq!(evaluate(&p, "2 + 3 * 4").unwrap(), 14.0);
        assert_eq!(evaluate(&p, "(2 + 3) * 4").unwrap(), 20.0);
        assert_eq!(evaluate(&p, "2 + (3 * 4)").unwrap(), 14.0);
        assert_eq!(evaluate(&p, "10 / 2 - 1").unwrap(), 4.0);
        assert_eq!(evaluate(&p, "2^3").unwrap(), 8.0);
    }

    #[test]
    fn test_unary_minus() {
        let p = dummy_player();
        assert_eq!(evaluate(&p, "-3").unwrap(), -3.0);
        assert_eq!(evaluate(&p, "-(2 + 3)").unwrap(), -5.0);
        assert_eq!(evaluate(&p, "1 + -2").unwrap(), -1.0);
        assert_eq!(evaluate(&p, "-QB").unwrap(), -60.0);
        assert_eq!(evaluate(&p, "-min(QB, HB)").unwrap(), -55.0);
    }
    
    #[test]
    fn test_comparison_and_logic() {
        let p = dummy_player();
        assert_eq!(evaluate(&p, "5 > 3").unwrap(), 1.0);
        assert_eq!(evaluate(&p, "4 == 4").unwrap(), 1.0);
        assert_eq!(evaluate(&p, "4 != 4").unwrap(), 0.0);
        assert_eq!(evaluate(&p, "3 < 2 || 1 == 1").unwrap(), 1.0);
        assert_eq!(evaluate(&p, "!(1 > 2)").unwrap(), 1.0);
        assert_eq!(evaluate(&p, "1 && 0").unwrap(), 0.0);
        assert_eq!(evaluate(&p, "1 || 0").unwrap(), 1.0);
    }

    #[test]
     fn test_logical_ops() {
         let p = dummy_player();
         assert_eq!(evaluate(&p, "not(1)").unwrap(), 0.0);
         assert_eq!(evaluate(&p, "not(0)").unwrap(), 1.0);
         assert_eq!(evaluate(&p, "and(1, 0)").unwrap(), 0.0);
         assert_eq!(evaluate(&p, "or(1, 0)").unwrap(), 1.0);
         assert_eq!(evaluate(&p, "not(1)").unwrap(), 0.0);
     }
    
    #[test]
    fn test_conditionals_and_functions() {
        let p = dummy_player();
        assert_eq!(evaluate(&p, "if(1 > 0, 10, 20)").unwrap(), 10.0);
        assert_eq!(evaluate(&p, "if(0, 10, 20)").unwrap(), 20.0);
        assert_eq!(evaluate(&p, "if(3 == 3, 1 + 2, 9)").unwrap(), 3.0);
        assert_eq!(evaluate(&p, "min(3, 7)").unwrap(), 3.0);
        assert_eq!(evaluate(&p, "max(2, 5)").unwrap(), 5.0);
        assert_eq!(evaluate(&p, "average(4, 8, 12)").unwrap(), 8.0);
    }
    
    #[test]
    fn test_stat_lookup() {
        let p = dummy_player();
        assert_eq!(evaluate(&p, "Spd").unwrap(), 10.0);
        assert_eq!(evaluate(&p, "QB + HB").unwrap(), 115.0);
        assert_eq!(evaluate(&p, "min(QB, HB)").unwrap(), 55.0);
        assert_eq!(evaluate(&p, "if(Dur >= 50, 1, 0)").unwrap(), 1.0);
    }
    
    #[test]
    fn test_errors() {
        let p = dummy_player();
    
        assert!(evaluate(&p, "2 +").is_err());
        assert!(evaluate(&p, "min()").is_err());
        assert!(evaluate(&p, "if(1, 2)").is_err());
        assert!(evaluate(&p, "1 / 0").is_err());
        assert!(evaluate(&p, "unknown_func(5)").is_err());
        assert!(evaluate(&p, "NotAStat").is_err());
    }
}