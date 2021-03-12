static BOILERPLATE: &str = r#"
onmessage = function (e) {
    const request = {
        ...e.data,
        send: (response) => {
            postMessage({ type: "send", response });
        },
        redirect: (response) => {
            postMessage({ type: "redirect", response })
        },
    }
    
    handle(request);
}
"#;

pub fn prepare_module(import: &str) -> String {
    format!(
        r#"
import {{ handle }} from "{}";
{}
"#,
        import, BOILERPLATE
    )
}
