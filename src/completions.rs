use tower_lsp::lsp_types::*;

pub fn list_completions<'a>() -> Vec<Vec<&'a str>> {
    let completions: Vec<Vec<&str>> = vec![
        //header completions
        vec!["Host", "HTTP Header", "field"],
        vec!["User-Agent", "HTTP Header", "field"],
        vec!["Accept", "HTTP Header", "field"],
        vec!["Accept-Language", "HTTP Header", "field"],
        vec!["Accept-Encoding", "HTTP Header", "field"],
        vec!["Referer", "HTTP Header", "field"],
        vec!["Connection", "HTTP Header", "field"],
        vec!["Upgrade-Insecure-Requests", "HTTP Header", "field"],
        vec!["If-Modified-Since", "HTTP Header", "field"],
        vec!["If-None-Match", "HTTP Header", "field"],
        vec!["Cache-Control", "HTTP Header", "field"],
        vec!["Access-Control-Allow-Origin", "HTTP Header", "field"],
        vec!["Content-Encoding", "HTTP Header", "field"],
        vec!["Content-Type", "HTTP Header", "field"],
        vec!["Date", "HTTP Header", "field"],
        vec!["Etag", "HTTP Header", "field"],
        vec!["Keep-Alive", "HTTP Header", "field"],
        vec!["Last-Modified", "HTTP Header", "field"],
        vec!["Server", "HTTP Header", "field"],
        vec!["Set-Cookie", "HTTP Header", "field"],
        vec!["Transfer-Encoding", "HTTP Header", "field"],
        vec!["Vary", "HTTP Header", "field"],
        vec!["X-Backend-Server", "HTTP Header", "field"],
        vec!["X-Cache-Info", "HTTP Header", "field"],
        vec!["X-kuma-revision", "HTTP Header", "field"],
        vec!["x-frame-options", "HTTP Header", "field"],
        vec!["Content-Length", "HTTP Header", "field"],
        vec!["Content-Range", "HTTP Header", "field"],
        vec!["Trailer", "HTTP Header", "field"],
        vec!["Accept", "HTTP Header", "field"],
        vec!["Accept-Charset", "HTTP Header", "field"],
        vec!["Authorization", "HTTP Header", "field"],
        vec!["Cookie", "HTTP Header", "field"],
        vec!["Expect", "HTTP Header", "field"],
        vec!["Forwarded", "HTTP Header", "field"],
        vec!["From", "HTTP Header", "field"],
        vec!["If-Match", "HTTP Header", "field"],
        vec!["If-Range", "HTTP Header", "field"],
        vec!["If-Unmodified-Since", "HTTP Header", "field"],
        vec!["Max-Forwards", "HTTP Header", "field"],
        vec!["Origin", "HTTP Header", "field"],
        vec!["Proxy-Authorization", "HTTP Header", "field"],
        vec!["Range", "HTTP Header", "field"],
        vec!["TE", "HTTP Header", "field"],
        vec!["Accept-Ranges", "HTTP Header", "field"],
        vec!["Age", "HTTP Header", "field"],
        vec!["Allow", "HTTP Header", "field"],
        vec!["Content-Disposition", "HTTP Header", "field"],
        vec!["Content-Language", "HTTP Header", "field"],
        vec!["Content-Location", "HTTP Header", "field"],
        vec!["Content-MD5", "HTTP Header", "field"],
        vec!["Content-Security-Policy", "HTTP Header", "field"],
        vec!["Expires", "HTTP Header", "field"],
        vec!["Link", "HTTP Header", "field"],
        vec!["Location", "HTTP Header", "field"],
        vec!["Proxy-Authenticate", "HTTP Header", "field"],
        vec!["Retry-After", "HTTP Header", "field"],
        vec!["Strict-Transport-Security", "HTTP Header", "field"],
        vec!["WWW-Authenticate", "HTTP Header", "field"],
        vec!["X-Content-Type-Options", "HTTP Header", "field"],
        vec!["X-DNS-Prefetch-Control", "HTTP Header", "field"],
        vec!["X-Frame-Options", "HTTP Header", "field"],
        vec!["X-XSS-Protection", "HTTP Header", "field"],
    ];

    completions
}

pub fn get_completions() -> Vec<CompletionItem> {
    list_completions()
        .iter()
        .map(|item| CompletionItem {
            label: item.get(0).unwrap().to_string(),
            detail: Some(item.get(1).unwrap().to_string()),
            kind: Some(CompletionItemKind::FIELD),
            ..Default::default()
        })
        .collect()
}
