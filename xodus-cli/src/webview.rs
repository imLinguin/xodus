use std::time::Duration;

use async_trait::async_trait;
use tao::{
    dpi::{LogicalSize, Size},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};
use wry::{
    PageLoadEvent, WebViewBuilder,
    http::{HeaderMap, HeaderValue},
};
use xodus::xal::AuthPromptCallback;

enum CustomEvent {
    Finish(String),
}

pub struct WebviewCallbackHandler;

impl WebviewCallbackHandler {
    pub async fn call(&self) -> Result<Option<xodus::xal::url::Url>, Box<dyn std::error::Error>> {
        let mut final_url = None;
        let mut event_loop: EventLoop<CustomEvent> = EventLoopBuilder::with_user_event().build();
        let window = WindowBuilder::new()
            .with_resizable(false)
            .with_title("Xodus login")
            .with_inner_size(Size::Logical(LogicalSize::new(500.0, 700.0)))
            .build(&event_loop)
            .unwrap();
        let proxy = event_loop.create_proxy();

        let clientid = "000000004424da1f".to_string();
        let market = "pl-PL".to_string();
        let uid = uuid::Uuid::new_v4();
        let url = format!(
            "https://login.live.com/ppsecure/InlineLogin.srf?id=80604&scid=3&mkt=en-US&Platform=Windows10&clientid={clientid}"
        );

        let mut headers = HeaderMap::new();
        headers.insert("cxh-capabilities", HeaderValue::from_static(r#"{"PrivatePropertyBag":1,"PasswordlessConnect":1,"PreferAssociate":1,"ChromelessUI":0}"#));
        headers.insert(
            "cxh-correlationId",
            HeaderValue::from_str(&format!("{uid}")).unwrap(),
        );
        headers.insert("cxh-msaBinaryVersion", HeaderValue::from_static(r#"55"#));
        headers.insert(
            "cxh-identityClientBinaryVersion",
            HeaderValue::from_static(r#"3"#),
        );
        headers.insert(
            "cxh-osVersionInfo",
            HeaderValue::from_static(
                r#"{"platformId":2,"majorVersion":10,"minorVersion":0,"buildNumber":26100}"#,
            ),
        );
        headers.insert(
            "cxh-platform",
            HeaderValue::from_static(r#"CloudExperienceHost.Platform.DESKTOP"#),
        );
        headers.insert("cxh-protocol", HeaderValue::from_static(r#"TokenBroker"#));
        headers.insert("cxh-source", HeaderValue::from_static(r#"TokenBroker"#));
        headers.insert(
            "hostApp",
            HeaderValue::from_static(r#"CloudExperienceHost"#),
        );
        headers.insert(
            "ms-identity-app-properties",
            HeaderValue::from_str(&format!("api-version=2.0&uaid={uid}&clientid={clientid}"))
                .unwrap(),
        );
        let headermap: HeaderMap = headers.try_into().unwrap();
        let builder = WebViewBuilder::new()
            .with_url(url)
            .with_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64; MSAppHost/3.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.102 Safari/537.36 Edge/18.26100")
            .with_headers(headermap)
            .with_on_page_load_handler(move |event, url| {
                if matches!(event, PageLoadEvent::Finished)
                    && url.starts_with("https://login.live.com/ppsecure/post.srf")
                {
                    proxy.send_event(CustomEvent::Finish(url)).ok();
                }
            });

        #[cfg(target_os = "linux")]
        let webview = {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;
            builder.build_gtk(window.default_vbox().unwrap()).unwrap()
        };
        #[cfg(not(target_os = "linux"))]
        let webview = builder.build(&window).unwrap();

        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::UserEvent(CustomEvent::Finish(url)) => {
                    if !url.contains("access_denied") {
                        final_url = Some(url.parse().unwrap());
                    }

                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }

                _ => (),
            }
        });
        
        let cookies = webview.cookies();

        println!("{final_url:#?}");
        println!("{cookies:#?}");

        Ok(final_url)
    }
}
