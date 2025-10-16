//! Manus AI Liquidity Frontend
//! 
//! Rust WASM frontend for the Manus AI Liquidity Autonomy Platform

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod pages;
mod utils;

use pages::{Home, Technology, Strategies, Funding, Agents};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/manus-liquidity-frontend.css"/>
        <Title text="Manus AI Liquidity | Quantum-Resistant LP Platform"/>
        <Meta name="description" content="The world's first AI-native, formally verified, quantum-resistant liquidity provisioning platform on Sui"/>
        
        <Router>
            <nav class="navbar">
                <div class="nav-container">
                    <a href="/" class="nav-logo">"Manus AI"</a>
                    <div class="nav-links">
                        <A href="/" class="nav-link">"Overview"</A>
                        <A href="/agents" class="nav-link">"AI Agents"</A>
                        <A href="/technology" class="nav-link">"Technology"</A>
                        <A href="/strategies" class="nav-link">"Strategies"</A>
                        <A href="/funding" class="nav-link">"Funding"</A>
                    </div>
                </div>
            </nav>
            
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/agents" view=Agents/>
                    <Route path="/technology" view=Technology/>
                    <Route path="/strategies" view=Strategies/>
                    <Route path="/funding" view=Funding/>
                </Routes>
            </main>
            
            <footer class="footer">
                <div class="footer-container">
                    <p>"Built with ❤️ for the Sui ecosystem"</p>
                    <p>"© 2025 Manus AI | Apache 2.0 License"</p>
                </div>
            </footer>
        </Router>
    }
}

/// Initialize the application
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");
    
    leptos::mount_to_body(App);
}

