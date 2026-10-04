use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum SubtitleError {
    #[error("字幕 JSON 无效：{0}")]
    Json(#[from] serde_json::Error),
    #[error("字幕第 {cue} 条时间范围无效")]
    InvalidTime { cue: usize },
    #[error("字幕没有可用内容")]
    Empty,
}

#[derive(Deserialize)]
struct BilibiliSubtitle {
    body: Vec<SubtitleCue>,
}

#[derive(Deserialize)]
struct SubtitleCue {
    from: f64,
    to: f64,
    content: String,
}

/// Convert Bilibili's downloaded subtitle payload, retaining Unicode and cue order.
pub fn bilibili_json_to_srt(json: &str) -> Result<String, SubtitleError> {
    let subtitle: BilibiliSubtitle = serde_json::from_str(json.trim_start_matches('\u{feff}'))?;
    let mut srt = String::new();
    let mut number = 0;
    for (index, cue) in subtitle.body.into_iter().enumerate() {
        let from = milliseconds(cue.from);
        let to = milliseconds(cue.to);
        let (Some(from), Some(to)) = (from, to) else {
            return Err(SubtitleError::InvalidTime { cue: index + 1 });
        };
        if to <= from {
            return Err(SubtitleError::InvalidTime { cue: index + 1 });
        }
        // Empty lines delimit SRT cues, so normalize line endings and drop blank
        // lines inside a cue rather than accidentally creating another cue.
        let normalized = cue.content.replace("\r\n", "\n").replace('\r', "\n");
        let content = normalized
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        if content.trim().is_empty() {
            continue;
        }
        number += 1;
        srt.push_str(&format!(
            "{number}\n{} --> {}\n{content}\n\n",
            timestamp(from),
            timestamp(to)
        ));
    }
    if number == 0 {
        return Err(SubtitleError::Empty);
    }
    Ok(srt)
}

fn milliseconds(seconds: f64) -> Option<u64> {
    let value = (seconds * 1000.0).round();
    (seconds.is_finite() && seconds >= 0.0 && value < u64::MAX as f64).then_some(value as u64)
}

fn timestamp(ms: u64) -> String {
    format!(
        "{:02}:{:02}:{:02},{:03}",
        ms / 3_600_000,
        ms / 60_000 % 60,
        ms / 1000 % 60,
        ms % 1000
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_unicode_multiline_and_rounds_across_minute_boundary() {
        let json = r#"{"font_size":0.4,"body":[
            {"from":0.38,"to":6.22,"content":"第一行\r\n\r\n第二行"},
            {"from":59.9996,"to":3601.002,"content":"♪ 色彩 ♪","sid":2}
        ]}"#;
        assert_eq!(
            bilibili_json_to_srt(json).unwrap(),
            "1\n00:00:00,380 --> 00:00:06,220\n第一行\n第二行\n\n2\n00:01:00,000 --> 01:00:01,002\n♪ 色彩 ♪\n\n"
        );
    }

    #[test]
    fn rejects_invalid_or_empty_subtitles() {
        for json in [
            "not json",
            "{}",
            r#"{"body":[]}"#,
            r#"{"body":[{"from":-1,"to":2,"content":"a"}]}"#,
            r#"{"body":[{"from":2,"to":1,"content":"a"}]}"#,
            r#"{"body":[{"from":0,"to":0.0001,"content":"a"}]}"#,
            r#"{"body":[{"from":0,"to":1,"content":" \n "}]}"#,
            r#"{"body":[{"from":0,"to":1e30,"content":"a"}]}"#,
        ] {
            assert!(bilibili_json_to_srt(json).is_err(), "{json}");
        }
    }

    #[test]
    fn accepts_bom_and_numbers_nonempty_cues_contiguously() {
        let json = "\u{feff}".to_owned()
            + r#"{"body":[{"from":0,"to":1,"content":" "},{"from":1,"to":2,"content":"内容"}]}"#;
        assert_eq!(
            bilibili_json_to_srt(&json).unwrap(),
            "1\n00:00:01,000 --> 00:00:02,000\n内容\n\n"
        );
    }
}
