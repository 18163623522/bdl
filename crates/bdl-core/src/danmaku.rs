use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DanmakuFormat {
    #[default]
    Xml,
    Html,
}

impl DanmakuFormat {
    pub fn extension(self) -> &'static str {
        match self {
            Self::Xml => "xml",
            Self::Html => "html",
        }
    }
}

/// Standalone, offline viewer. Untrusted XML stays data, never executable HTML.
pub fn xml_to_html(xml: &str) -> Result<String, serde_json::Error> {
    let data = serde_json::to_string(xml)?
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026");
    Ok(include_str!("danmaku-viewer.html").replace("__BDL_XML__", &data))
}

#[cfg(test)]
mod tests {
    #[test]
    fn embeds_untrusted_xml_as_data_and_keeps_viewer_offline() {
        let html =
            super::xml_to_html("<i><d p=\"1,1,25,16777215\">&lt;/script&gt;中文</d></i>").unwrap();
        assert!(html.contains("\\u003ci\\u003e"));
        assert!(!html.contains("<i>"));
        assert!(!html.contains("https://"));
        assert!(html.contains("textContent"));
    }
}
