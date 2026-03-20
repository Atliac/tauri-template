use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="min-h-screen bg-base-200 flex items-center justify-center p-4">
            <div class="card w-96 bg-base-100 shadow-xl border border-primary/10">
                <div class="card-body items-center text-center">
                    <h1 class="card-title text-4xl font-black bg-linear-to-r from-primary to-secondary bg-clip-text text-transparent">
                        "Atliac"
                    </h1>

                    <p class="text-base-content/70 italic mt-2">
                        "Hey bro, what's up?"
                    </p>

                    <div class="card-actions justify-center mt-8 w-full gap-3">
                        <a href="https://bsky.app/profile/atliac.com"
                           class="btn btn-primary btn-outline flex-1 gap-2 hover:scale-105 transition-transform">
                            <svg fill="currentColor" viewBox="0 0 64 57" class="w-5 h-5">
                                <path d="M13.873 3.805C21.21 9.332 29.103 20.537 32 26.55v15.882c0-.338-.13.044-.41.867-1.512 4.456-7.418 21.847-20.923 7.944-7.111-7.32-3.819-14.64 9.125-16.85-7.405 1.264-15.73-.825-18.014-9.015C1.12 23.022 0 8.51 0 6.55 0-3.268 8.579-.182 13.873 3.805ZM50.127 3.805C42.79 9.332 34.897 20.537 32 26.55v15.882c0-.338.13.044.41.867 1.512 4.456 7.418 21.847 20.923 7.944 7.111-7.32-3.819-14.64-9.125-16.85 7.405 1.264 15.73-.825 18.014-9.015C62.88 23.022 64 8.51 64 6.55c0-9.818-8.578-6.732-13.873-2.745Z"></path>
                            </svg>
                            "@atliac.com"
                        </a>

                        <a href="https://github.com/atliac"
                           class="btn btn-ghost btn-outline flex-1 gap-2 hover:scale-105 transition-transform">
                            <svg fill="currentColor" viewBox="0 0 24 24" class="w-5 h-5">
                                <path d="M12 2C6.477 2 2 6.477 2 12c0 4.418 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.156-1.11-1.463-1.11-1.463-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.098-2.647 0 0 .84-.269 2.75 1.025A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.294 2.747-1.025 2.747-1.025.546 1.377.203 2.394.1 2.647.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482C19.138 20.161 22 16.416 22 12c0-5.523-4.477-10-10-10z"></path>
                            </svg>
                            "GitHub"
                        </a>
                    </div>
                </div>
            </div>
        </main>
    }
}
