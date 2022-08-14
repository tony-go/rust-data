#[derive(Debug, PartialEq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
    fn from(f: f64) -> Self {
        if f >= 1.0 {
            Q7(127)
        } else if f <= -1.0 {
            Q7(-128)
        } else {
            let fixed = f * 128.0;
            let fixed = fixed as i8;
            Q7(fixed)
        }
    }
}

impl From<Q7> for f64 {
    fn from(q: Q7) -> f64 {
        (q.0 as f64) * 2_f64.powf(-7_f64)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.15), Q7::from(1.0));
        assert_eq!(Q7::from(-10.15), Q7::from(-1.0));
    }

    #[test]
    fn f64_to_q7() {
        let f: f64 = 0.7;
        let q = Q7::from(f);
        assert_eq!(q, Q7(89));

        let f2: f64 = -0.4;
        let q2 = Q7::from(f2);
        assert_eq!(q2, Q7(-51));
    }

    #[test]
    fn q7_to_f64() {
        let q: Q7 = Q7(89);
        let f = f64::from(q);
        assert_eq!(f, 0.6953125);
    }
}
