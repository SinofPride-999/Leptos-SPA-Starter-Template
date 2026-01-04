use leptos::prelude::*;

use crate::components::Button;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center min-h-screen p-4 md:p-8">
            <div class="max-w-4xl w-full animate-fade-in">
                <div class="bg-gradient-to-br from-slate-800/70 to-slate-900/70 backdrop-blur-xl
                           rounded-2xl md:rounded-3xl border border-white/10
                           shadow-2xl shadow-slate-900/50 p-6 md:p-10 lg:p-12
                           animate-pulse-glow">

                    {/* Header */}
                    <div class="text-center mb-10 md:mb-12">
                        <h1 class="text-3xl md:text-4xl lg:text-5xl font-black
                                  bg-gradient-to-r from-slate-100 via-cyan-300 to-emerald-400
                                  bg-clip-text text-transparent mb-4">
                            "Welcome to Leptos SPA!"
                        </h1>
                        <p class="text-lg md:text-xl text-slate-300 max-w-3xl mx-auto leading-relaxed">
                            "A modern, reactive web application built with Rust and Leptos framework.
                            Experience lightning-fast performance with zero JavaScript overhead."
                        </p>
                    </div>

                    {/* Features Grid */}
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 md:gap-6 mb-10 md:mb-12">
                        <div class="bg-gradient-to-br from-slate-800/80 to-slate-900/80
                                  p-5 md:p-6 rounded-xl md:rounded-2xl border border-slate-700/50
                                  hover:border-cyan-500/50 hover:bg-slate-800/90
                                  transition-all duration-300 group cursor-pointer">
                            <div class="text-3xl md:text-4xl mb-4 group-hover:scale-110 transition-transform duration-300">"⚡"</div>
                            <h3 class="font-bold text-lg md:text-xl text-white mb-2">"Blazing Fast"</h3>
                            <p class="text-sm md:text-base text-slate-300">"Compiles to WebAssembly for native speed"</p>
                        </div>

                        <div class="bg-gradient-to-br from-slate-800/80 to-slate-900/80
                                  p-5 md:p-6 rounded-xl md:rounded-2xl border border-slate-700/50
                                  hover:border-emerald-500/50 hover:bg-slate-800/90
                                  transition-all duration-300 group cursor-pointer">
                            <div class="text-3xl md:text-4xl mb-4 group-hover:scale-110 transition-transform duration-300">"📦"</div>
                            <h3 class="font-bold text-lg md:text-xl text-white mb-2">"Modular Architecture"</h3>
                            <p class="text-sm md:text-base text-slate-300">"Pre-structured folders "</p>
                        </div>

                        <div class="bg-gradient-to-br from-slate-800/80 to-slate-900/80
                                  p-5 md:p-6 rounded-xl md:rounded-2xl border border-slate-700/50
                                  hover:border-purple-500/50 hover:bg-slate-800/90
                                  transition-all duration-300 group cursor-pointer">
                            <div class="text-3xl md:text-4xl mb-4 group-hover:scale-110 transition-transform duration-300">"🛣️"</div>
                            <h3 class="font-bold text-lg md:text-xl text-white mb-2">"Routing Ready"</h3>
                            <p class="text-sm md:text-base text-slate-300">"Client-side routing pre-configured"</p>
                        </div>

                        <div class="bg-gradient-to-br from-slate-800/80 to-slate-900/80
                                  p-5 md:p-6 rounded-xl md:rounded-2xl border border-slate-700/50
                                  hover:border-orange-500/50 hover:bg-slate-800/90
                                  transition-all duration-300 group cursor-pointer">
                            <div class="text-3xl md:text-4xl mb-4 group-hover:scale-110 transition-transform duration-300">"🎨"</div>
                            <h3 class="font-bold text-lg md:text-xl text-white mb-2">"Tailwind Ready"</h3>
                            <p class="text-sm md:text-base text-slate-300">"Tailwind CSS Pre-configured through CDN"</p>
                        </div>
                    </div>

                    {/* Interactive Demo */}
                    <div class="bg-gradient-to-br from-slate-800/50 to-slate-900/50
                              rounded-xl md:rounded-2xl border border-white/5
                              p-6 md:p-8">
                        <div class="text-center mb-6">
                            <h2 class="text-xl md:text-2xl font-bold text-white mb-2">
                                "Interactive Demo"
                            </h2>
                            <p class="text-slate-400 max-w-2xl mx-auto">
                                "Try the interactive counter below to experience Leptos' reactive system in action!"
                            </p>
                        </div>

                        <div class="flex flex-col items-center">
                            <div class="mb-6 animate-fade-in">
                                <Button/>
                            </div>

                            <div class="text-sm text-slate-500 max-w-md text-center">
                                "✨ This counter updates instantly with zero JavaScript overhead, powered by Rust and WebAssembly!"
                            </div>
                        </div>
                    </div>

                    {/* Footer */}
                    <div class="mt-8 md:mt-10 pt-6 md:pt-8 border-t border-white/10 text-center">
                        <div class="flex flex-col sm:flex-row justify-center items-center gap-4 sm:gap-8">
                            <a
                                href="/"
                                class="text-cyan-400 hover:text-cyan-300 font-medium
                                       transition-colors duration-200 hover:underline"
                            >
                                "Home"
                            </a>
                            <a
                                href="https://leptos.dev"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-slate-400 hover:text-slate-300 font-medium
                                       transition-colors duration-200 hover:underline flex items-center gap-1"
                            >
                                "Leptos Docs"
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd" />
                                </svg>
                            </a>
                            <a
                                href="https://github.com/SinofPride-999/Leptos-SPA-Starter-Template"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-slate-400 hover:text-slate-300 font-medium
                                       transition-colors duration-200 hover:underline flex items-center gap-1"
                            >
                                "GitHub"
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M12.293 5.293a1 1 0 011.414 0l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-2.293-2.293a1 1 0 010-1.414z" clip-rule="evenodd" />
                                </svg>
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
