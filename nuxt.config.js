const baseUrl = "https://skrapion.github.io/";
const desc = "Rick Yorgason's portfolio blog. Everything from traditional woodworking to video game development."

export default {
    target: 'static',
    css: [
        "~/assets/styles/main.css"
    ],
    modules: [
        '@nuxt/content',
        '@nuxt/image'
    ],
    components: true,
    generate: {
        fallback: '404.html'
    },
    env: {
        BaseURL: baseUrl
    },
    head: {
        title: 'Firefang',
        meta: [
            {charset: 'utf-8'},
            {name: 'viewport', content: 'width=device-width, initial-scale=1'},
            {hid: 'description', name: 'description', content: desc},

            { hid: 'og:title', property: "og:title", content: "Firefang" },
            { hid: 'og:description', property: "og:description", content: desc },
            { hid: 'og:image', property: 'og:image', content: baseUrl + "RickHoldingTheWorld.jpg" },

            { hid: 'twitter:title', property: 'twitter:title', content: "Firefang" },
            { hid: 'twitter:description', property: 'twitter:description', content: desc },
            { hid: 'twitter:image', property: 'twitter:image', content: baseUrl + "RickHoldingTheWorld.jpg" }
        ],
        link: [{rel: 'icon', type: 'image/x-icon', href: '/favicon.ico'}]
    }
}