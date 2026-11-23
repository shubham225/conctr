#[cfg(not(feature = "server"))]
use conctr_app_dioxus::App;

fn main() -> Result<(), String> {
    #[cfg(feature = "server")]
    return conctr_app_dioxus::serve_workspace();
    #[cfg(not(feature = "server"))]
    {
        #[cfg(all(feature = "mobile", not(feature = "server")))]
        conctr_app_dioxus::configure_native_api()?;

        dioxus::launch(App);
        Ok(())
    }
}
