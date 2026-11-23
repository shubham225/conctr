#[cfg(not(feature = "server"))]
use conctr_app_dioxus::App;

fn main() -> Result<(), String> {
    #[cfg(feature = "server")]
    return conctr_app_dioxus::serve_workspace();
    #[cfg(not(feature = "server"))]
    {
        dioxus::launch(App);
        Ok(())
    }
}
