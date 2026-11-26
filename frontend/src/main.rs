use dioxus::prelude::*;

// Startet Dioxus-Anwendung
fn main() {
    dioxus::launch(HabitTracker);
}

// Hauptkomponente der Benutzeroberfläche
pub fn HabitTracker() -> Element {
    let paragraph = "Hallo";

    rsx! {
        // bindet assets ein
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/main.css")
        }

        h1 {
            "Habit Tracker"
        }
        
        div {
            class: "content",
            {paragraph}
        }
    }
} 