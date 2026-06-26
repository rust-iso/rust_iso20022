//! Tiny dependency-free XML element/attribute scanner shared by the
//! [`app_hdr`](crate::app_hdr) and [`metadata`](crate::metadata) readers. It
//! reads a handful of well-known elements by local name without a full parse,
//! so it works on any message version and without the `model` feature.

/// Local-name of an XML tag, ignoring any namespace prefix (`h:Fr` -> `Fr`).
pub(crate) fn local_name(tag: &str) -> &str {
    tag.rsplit(':').next().unwrap_or(tag)
}

/// Trimmed inner text of the first element whose local name matches `local`.
pub(crate) fn element_text(xml: &str, local: &str) -> Option<String> {
    element_inner(xml, local).map(|s| s.trim().to_string())
}

/// Read an attribute value off the first element whose local name matches
/// `local`, e.g. the `Ccy` of `<IntrBkSttlmAmt Ccy="EUR">`.
pub(crate) fn element_attr(xml: &str, local: &str, attr: &str) -> Option<String> {
    let (open_start, open_end) = find_open_tag(xml, local)?;
    let open = &xml[open_start..open_end];
    let key = format!("{attr}=\"");
    let s = open.find(&key)? + key.len();
    let e = open[s..].find('"')? + s;
    Some(open[s..e].to_string())
}

/// Inner XML (raw) of the first element whose local name matches `local`.
pub(crate) fn element_inner(xml: &str, local: &str) -> Option<String> {
    let bytes = xml.as_bytes();
    let (start, gt) = find_open_tag(xml, local)?;
    if bytes[gt - 1] == b'/' {
        return Some(String::new()); // self-closing
    }
    let _ = start;
    let content_start = gt + 1;
    let mut depth = 1usize;
    let mut j = content_start;
    while let Some(next) = xml[j..].find('<') {
        let p = j + next;
        let a = &xml[p + 1..];
        if a.starts_with('/') {
            let close_end = xml[p..].find('>')? + p;
            let close_tag = &xml[p + 2..close_end];
            if local_name(close_tag) == local {
                depth -= 1;
                if depth == 0 {
                    return Some(xml[content_start..p].to_string());
                }
            }
            j = close_end + 1;
        } else if a.starts_with('!') || a.starts_with('?') {
            j = p + 1;
        } else {
            let oe = xml[p..].find('>')? + p;
            let ot = &xml[p + 1..oe];
            let oname_end = ot
                .find(|c: char| c == '>' || c == '/' || c.is_whitespace())
                .unwrap_or(ot.len());
            if local_name(&ot[..oname_end]) == local && !ot.ends_with('/') {
                depth += 1;
            }
            j = oe + 1;
        }
    }
    None
}

/// Find the first opening tag with local name `local`; returns `(tag_start, gt)`
/// where `tag_start` is the `<` index and `gt` is the `>` index.
fn find_open_tag(xml: &str, local: &str) -> Option<(usize, usize)> {
    let mut i = 0;
    while let Some(lt) = xml[i..].find('<') {
        let start = i + lt;
        let after = &xml[start + 1..];
        if after.starts_with('/') || after.starts_with('!') || after.starts_with('?') {
            i = start + 1;
            continue;
        }
        let name_end = after
            .find(|c: char| c == '>' || c == '/' || c.is_whitespace())
            .map(|e| start + 1 + e)?;
        let tag = &xml[start + 1..name_end];
        if local_name(tag) == local {
            let gt = xml[start..].find('>')? + start;
            return Some((start, gt));
        }
        i = name_end;
    }
    None
}

/// The first element (by local name) found among `candidates`, with its text.
pub(crate) fn first_of<'a>(xml: &str, candidates: &[&'a str]) -> Option<(&'a str, String)> {
    candidates
        .iter()
        .find_map(|&name| element_text(xml, name).map(|v| (name, v)))
}
