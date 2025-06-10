use leptos::prelude::*;

use crate::components::NavUserMenu;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="absolute w-full px-4 py-1 flex justify-between items-center bg-white z-40">
        <a href="/" class="py-1.5 hover:bg-white">
            <img src="/img/favicon/favicon-32x32.png" class="w-[24px] h-[24px]"/>
        </a>
        <ul
            class="hidden absolute top-1/2 sm:left-1/3 sm:pl-16 md:left-1/2 lg:left-1/2
                    transform -translate-y-1/2 -translate-x-1/2"
        >
            <li>
                <a href="/" class="text-sm text-gray-600 py-1 px-4 hover:bg-gray-100 rounded-lg transition duration-200">Home</a>
            </li>
        </ul>
        <NavUserMenu/>
    </nav>
    }
}
