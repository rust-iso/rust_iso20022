//! Runtime ISO 20022 schema fetcher (`catalogue` feature).
//!
//! Downloads message-definition XSD schemas so that `src/bin/codegen.rs` can
//! regenerate the model. The canonical source is the official ISO 20022
//! catalogue at <https://www.iso20022.org>, but that host sits behind Akamai
//! bot-protection that refuses non-browser TLS clients, so the [`Fetcher`] base
//! URL is configurable and can also target a schema mirror.
//!
//! Enable with the `catalogue` feature; this module pulls in `tokio`,
//! `reqwest` and `regex`.
//!
//! ```no_run
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! use rust_iso20022::fetch::Fetcher;
//!
//! let fetcher = Fetcher::new();
//! // Discover the message-definition links from the catalogue index page.
//! let messages = fetcher.index().await?;
//! // Download each schema into ./xsds.
//! fetcher.download_all(&messages, "xsds".as_ref()).await?;
//! # Ok(())
//! # }
//! ```

use std::path::Path;

use regex::Regex;
use reqwest::Client;

/// The official ISO 20022 message-definitions index page.
pub const ISO20022_INDEX: &str = "https://www.iso20022.org/iso-20022-message-definitions";

/// A discovered remote message definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteMessage {
    /// The message name if it could be determined, e.g. `"pacs.008.001.08"`.
    pub message_name: Option<String>,
    /// Absolute URL to download the XSD schema.
    pub xsd_url: String,
}

/// Downloads ISO 20022 schemas from a configurable base host.
pub struct Fetcher {
    client: Client,
    /// Base origin used to resolve relative links, e.g. `https://www.iso20022.org`.
    pub origin: String,
    /// The index page listing message definitions.
    pub index_url: String,
}

impl Default for Fetcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Fetcher {
    /// A fetcher targeting the official iso20022.org catalogue, with a
    /// browser-like user agent (the host rejects the default `reqwest` agent).
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent(
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
                 AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0 Safari/537.36",
            )
            .build()
            .expect("build reqwest client");
        Self {
            client,
            origin: "https://www.iso20022.org".to_string(),
            index_url: ISO20022_INDEX.to_string(),
        }
    }

    /// Point the fetcher at an alternative origin and index URL (e.g. a mirror).
    pub fn with_source(mut self, origin: impl Into<String>, index_url: impl Into<String>) -> Self {
        self.origin = origin.into();
        self.index_url = index_url.into();
        self
    }

    /// Fetch and scrape the index page for schema download links.
    ///
    /// Recognises both the catalogue's per-message `/message/{id}/download`
    /// endpoints and any direct `.xsd` hyperlinks.
    pub async fn index(&self) -> Result<Vec<RemoteMessage>, reqwest::Error> {
        let html = self.client.get(&self.index_url).send().await?.text().await?;
        Ok(self.parse_index(&html))
    }

    /// Parse message-definition links out of an index HTML page (pure, testable).
    pub fn parse_index(&self, html: &str) -> Vec<RemoteMessage> {
        // `aaaa.999.999.99` message-name token, used to label discovered links.
        let name_re = Regex::new(r"[a-z]{4}\.\d{3}\.\d{3}\.\d{2}").unwrap();
        // href to either a direct .xsd or a /message/{id}/download endpoint.
        let href_re =
            Regex::new(r#"href="([^"]*(?:\.xsd|/message/\d+/download)[^"]*)""#).unwrap();

        let mut out = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for cap in href_re.captures_iter(html) {
            let raw = &cap[1];
            let url = self.absolutize(raw);
            if !seen.insert(url.clone()) {
                continue;
            }
            let message_name = name_re.find(raw).map(|m| m.as_str().to_string());
            out.push(RemoteMessage { message_name, xsd_url: url });
        }
        out
    }

    /// Resolve a possibly-relative href against the fetcher origin.
    fn absolutize(&self, href: &str) -> String {
        if href.starts_with("http://") || href.starts_with("https://") {
            href.to_string()
        } else if let Some(rest) = href.strip_prefix('/') {
            format!("{}/{}", self.origin.trim_end_matches('/'), rest)
        } else {
            format!("{}/{}", self.origin.trim_end_matches('/'), href)
        }
    }

    /// Download one schema's bytes.
    pub async fn download(&self, message: &RemoteMessage) -> Result<String, reqwest::Error> {
        self.client.get(&message.xsd_url).send().await?.text().await
    }

    /// Download a schema by message name directly from the static schemas path,
    /// e.g. `sese.023.001.13` →
    /// `…/sites/default/files/documents/messages/sese/schemas/sese.023.001.13.xsd`.
    ///
    /// This static path is served without the Akamai bot-protection that guards
    /// the `/message/{id}/download` endpoint, so it is the reliable way to fetch
    /// schemas programmatically.
    pub async fn download_schema(&self, message_name: &str) -> Result<String, reqwest::Error> {
        let family = message_name.split('.').next().unwrap_or("");
        let url = format!(
            "{}/sites/default/files/documents/messages/{family}/schemas/{message_name}.xsd",
            self.origin.trim_end_matches('/')
        );
        self.client.get(url).send().await?.text().await
    }

    /// Download every discovered schema into `dir`, naming files by message name
    /// when known. Returns the number successfully written.
    pub async fn download_all(
        &self,
        messages: &[RemoteMessage],
        dir: &Path,
    ) -> std::io::Result<usize> {
        tokio::fs::create_dir_all(dir).await?;
        let mut written = 0usize;
        for (i, m) in messages.iter().enumerate() {
            let body = match self.download(m).await {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("download failed for {}: {e}", m.xsd_url);
                    continue;
                }
            };
            let name = m
                .message_name
                .clone()
                .unwrap_or_else(|| format!("schema_{i}"));
            let path = dir.join(format!("{name}.xsd"));
            tokio::fs::write(&path, body).await?;
            written += 1;
        }
        Ok(written)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_message_links_from_html() {
        let f = Fetcher::new();
        let html = r#"
            <a href="/message/14316/download">pacs.008.001.08</a>
            <a href="https://example.org/schemas/pain.001.001.09.xsd">pain</a>
            <a href="/message/14316/download">dup</a>
        "#;
        let msgs = f.parse_index(html);
        assert_eq!(msgs.len(), 2, "duplicate links should be de-duplicated");
        // The /message/{id}/download href carries no message name (it lives in
        // the link text), so the name is unknown for that form.
        assert_eq!(msgs[0].message_name, None);
        assert_eq!(
            msgs[0].xsd_url,
            "https://www.iso20022.org/message/14316/download"
        );
        // A direct .xsd link does carry the name in the URL.
        assert_eq!(msgs[1].message_name.as_deref(), Some("pain.001.001.09"));
    }
}
