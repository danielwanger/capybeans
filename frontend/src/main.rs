use dioxus::prelude::*;

// Startet Dioxus-Anwendung
fn main() {
    dioxus::launch(App);
}

// Hauptkomponente der Benutzeroberfläche
fn App() -> Element {
    let paragraph = "Hallo";

    rsx! {
        // bindet assets ein
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/main.css")
        }

        {paragraph}
    }
} 