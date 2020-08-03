// These are to reuse at routes/

#[macro_export]
macro_rules! json_body {
    () => {
        // let json_body = content_length_limit(1024 * 16).and(json())
        content_length_limit(1024 * 16).and(json())
    };
}
