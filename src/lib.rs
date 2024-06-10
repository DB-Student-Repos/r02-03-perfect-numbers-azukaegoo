#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
   /*  unimplemented!("classify {num}"); */
    if num == 0 {
        return None;
    }
    let aliqot_sum: u64 = (1..num).filter(|&divisor| num % divisor == 0).sum();

    if aliqot_sum > num {
        Some(Classification::Abundant)
    } else if aliqot_sum == num {
        Some(Classification::Perfect)
    }else {
        Some(Classification::Deficient)
    }
}
