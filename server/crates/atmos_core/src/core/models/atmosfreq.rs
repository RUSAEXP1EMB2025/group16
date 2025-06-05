use super::adjust_lighting::SiteInfo;

/// Amount of atmosfreq is between 0 ~ 100
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct AtmosFreq(i32);

impl AtmosFreq {
    pub fn new(site_info: &SiteInfo) -> Self {
        let atmosfreq = Self::calc_atmosfreq(site_info);
        AtmosFreq(atmosfreq)
    }

    fn calc_atmosfreq(site_info: &SiteInfo) -> i32 {
        // TODO:  Caluclate atmosfreq using text
        match site_info {
            SiteInfo::Youtube { tags } => todo!(),
            SiteInfo::Netflix { title } => todo!(),
            SiteInfo::Others { text } => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::AtmosFreq;
    use crate::core::models::adjust_lighting::{SiteInfo, Url};

    #[test]
    fn test_calc_atmosfreq() {
        let site_info = SiteInfo::from(Url::from(""));
        let atmosfreq = AtmosFreq::new(&site_info);
        assert!((0..100).contains(&atmosfreq.0))
    }
}
