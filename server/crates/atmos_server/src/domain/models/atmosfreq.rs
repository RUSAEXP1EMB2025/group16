use derive_more::From;

// 雰囲気指数 (0 ~ 100)
#[derive(PartialEq, Eq, PartialOrd, Ord, From)]
pub struct AtmosFreq(i32);

impl AtmosFreq {
    pub fn new(texts: &Vec<String>) -> Self {
        AtmosFreq::from(texts)
    }
}

impl From<&Vec<String>> for AtmosFreq {
    fn from(texts: &Vec<String>) -> Self {
        // TODO: テキスト群から雰囲気指数を計算する。amountは0 ~ 100以内にする
        let amount: i32 = todo!();
        let amount = amount.clamp(0, 100);

        AtmosFreq(amount)
    }
}

#[cfg(test)]
mod test {
    use super::AtmosFreq;

    #[test]
    fn test_valid_atmosfreq() {
        let atmosfreq = AtmosFreq::from(0);
        assert!((0..100).contains(&atmosfreq.0))
    }
}
