use async_trait::async_trait;
use tao::{
    dpi::{LogicalSize, Size},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    platform::{run_return::EventLoopExtRunReturn},
    window::WindowBuilder,
};
use wry::{PageLoadEvent, WebViewBuilder};
use xodus::xal::AuthPromptCallback;

enum CustomEvent {
    Finish(String),
}

pub struct WebviewCallbackHandler;

#[async_trait]
impl AuthPromptCallback for WebviewCallbackHandler {
    async fn call(
        &self,
        cb_data: xodus::xal::AuthPromptData,
    ) -> Result<Option<xodus::xal::url::Url>, Box<dyn std::error::Error>> {
        let url = cb_data.authentication_url();
        let mut final_url = None;
        let mut event_loop: EventLoop<CustomEvent> = EventLoopBuilder::with_user_event().build();
        let window = WindowBuilder::new()
            .with_resizable(false)
            .with_title("Xodus login")
            .with_inner_size(Size::Logical(LogicalSize::new(500.0, 700.0)))
            .build(&event_loop)
            .unwrap();
        let proxy = event_loop.create_proxy();
        let builder = WebViewBuilder::new()
            .with_url(url)
            .with_on_page_load_handler(move |event, url| {
                if matches!(event, PageLoadEvent::Started)
                    && url.starts_with("https://login.live.com/oauth20_desktop.srf")
                {
                    proxy.send_event(CustomEvent::Finish(url)).ok();
                }
            });
        
        #[cfg(target_os = "linux")]
        {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;
            let _webview = builder.build_gtk(window.default_vbox().unwrap()).unwrap();
        }
        #[cfg(not(target_os = "linux"))]
        let _webview = builder.build(&window).unwrap();

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

        Ok(final_url)
    }
}
