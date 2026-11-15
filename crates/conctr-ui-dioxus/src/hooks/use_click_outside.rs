use dioxus::prelude::*;

/// Register an outside-click listener owned by the mounting component.
/// The listener is removed before its captured signals can be used after unmount.
pub fn use_click_outside<F>(element: ReadSignal<Option<web_sys::Element>>, callback: F)
where
    F: Fn() + Clone + 'static,
{
    #[cfg(target_arch = "wasm32")]
    {
        use std::{cell::RefCell, rc::Rc};
        use wasm_bindgen::{JsCast, closure::Closure};
        struct Listener {
            document: web_sys::Document,
            handler: Closure<dyn Fn(web_sys::MouseEvent)>,
        }
        impl Drop for Listener {
            fn drop(&mut self) {
                let _ = self.document.remove_event_listener_with_callback(
                    "mousedown",
                    self.handler.as_ref().unchecked_ref(),
                );
            }
        }
        let listener = use_hook(|| Rc::new(RefCell::new(None::<Listener>)));
        let cleanup = listener.clone();
        use_drop(move || {
            cleanup.borrow_mut().take();
        });
        use_effect(move || {
            let Some(document) = web_sys::window().and_then(|w| w.document()) else {
                return;
            };
            let callback = callback.clone();
            let handler =
                Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                    let Ok(current) = element.try_read() else {
                        return;
                    };
                    let Some(element) = current.as_ref() else {
                        return;
                    };
                    let Some(target) = event
                        .target()
                        .and_then(|target| target.dyn_into::<web_sys::Node>().ok())
                    else {
                        return;
                    };
                    if !element.contains(Some(&target)) {
                        callback();
                    }
                });
            if document
                .add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())
                .is_ok()
            {
                *listener.borrow_mut() = Some(Listener { document, handler });
            }
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (element, callback);
}
