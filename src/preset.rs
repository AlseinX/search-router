use url::Url;

pub enum Provider {
    Baidu,
    Google,
    BingDomestic,
    BingInternational,
}

impl Provider {
    pub fn handle(query: &str) -> String {
        if let Err((provider, query)) = Self::parse_prefix(query) {
            provider.search_url(query)
        } else if query
            .chars()
            .any(|c| (0x4e00u32..=0x9fa5).contains(&(c as _)))
        {
            Self::Baidu.search_url(query)
        } else {
            Self::Google.search_url(query)
        }
    }

    fn parse_prefix(query: &str) -> Result<(), (Self, &str)> {
        Self::Baidu.parse_prefix_one("b", query)?;
        Self::Baidu.parse_prefix_one("baidu", query)?;
        Self::Google.parse_prefix_one("g", query)?;
        Self::Google.parse_prefix_one("google", query)?;
        Self::BingDomestic.parse_prefix_one("bd", query)?;
        Self::BingInternational.parse_prefix_one("bi", query)?;
        Self::BingInternational.parse_prefix_one("bing", query)?;
        Ok(())
    }

    fn parse_prefix_one<'a>(
        self,
        prefix: &'static str,
        query: &'a str,
    ) -> Result<(), (Self, &'a str)> {
        let mut q = query.chars();
        for p in prefix.chars() {
            if q.next() != Some(p) {
                return Ok(());
            }
        }
        if q.next() == Some(' ') {
            Err((self, q.as_str()))
        } else {
            Ok(())
        }
    }

    fn search_url(self, query: &str) -> String {
        match self {
            Provider::Baidu => Self::make_url("https://www.baidu.com/s", "wd", query),
            Provider::Google => Self::make_url("https://www.google.com/search", "q", query),
            Provider::BingDomestic => {
                Self::make_url("https://cn.bing.com/search?ensearch=0", "q", query)
            }
            Provider::BingInternational => {
                Self::make_url("https://cn.bing.com/search?ensearch=1", "q", query)
            }
        }
    }

    fn make_url(base: &str, p_name: &str, value: &str) -> String {
        Url::parse_with_params(base, [(p_name, value)])
            .unwrap()
            .to_string()
    }
}
