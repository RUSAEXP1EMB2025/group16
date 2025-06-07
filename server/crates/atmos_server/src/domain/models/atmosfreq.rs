use derive_more::From;

// 雰囲気指数 (-100.0 ~ 100.0)
#[derive(PartialEq, Debug, From)]
pub struct AtmosFreq(f64);

impl AtmosFreq {
    pub fn new(texts: &Vec<String>) -> Self {
        AtmosFreq::from(texts)
    }
}

impl From<&Vec<String>> for AtmosFreq {
    fn from(texts: &Vec<String>) -> Self {
        // TODO: テキスト群から雰囲気指数を計算する。amountは0 ~ 100以内にする
        let amount: f64 = todo!();
        let amount = amount.clamp(-100.0, 100.0);

        AtmosFreq(amount)
    }
}

#[cfg(test)]
mod test {
    use super::AtmosFreq;

    fn mock_text(texts: impl Into<Vec<&'static str>>) -> Vec<String> {
        let texts: Vec<&str> = texts.into();
        let texts = texts.iter().map(|&s| String::from(s)).collect();
        texts
    }

    #[test]
    fn test_calc_atmosfreq_1() {
        // TODO: テストケースの実装
        let texts = mock_text(["a", "b", "c"]);
        let expect_result = 0.0;

        let atmosfreq = AtmosFreq::new(&texts);
        assert_eq!(atmosfreq, AtmosFreq::from(expect_result))
    }

    #[test]
    fn test_calc_atmosfreq_2() {
        // TODO: テストケースの実装
        let texts = mock_text(["a", "b", "c"]);
        let expect_result = 0.0;

        let atmosfreq = AtmosFreq::new(&texts);
        assert_eq!(atmosfreq, AtmosFreq::from(expect_result))
    }

    #[test]
    fn test_calc_atmosfreq_3() {
        // TODO: テストケースの実装
        let texts = mock_text(["a", "b", "c"]);
        let expect_result = 0.0;

        let atmosfreq = AtmosFreq::new(&texts);
        assert_eq!(atmosfreq, AtmosFreq::from(expect_result))
    }
}
