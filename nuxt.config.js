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
        baseURL: "https://skrapion.github.io/"
    },
    head: {
        title: 'Firefang',
        meta: [
            {charset: 'utf-8'},
            {name: 'viewport', content: 'width=device-width, initial-scale=1'},
            {hid: 'description', name: 'description', content: "Rick Yorgason's portfolio blog"},

            { hid: 'og:title', property: "og:title", content: "Firefang" },
            { hid: 'og:description', property: "og:description", content: "Rick Yorgason's portfolio blog" },
            { hid: 'og:image', property: 'og:image', content: "/RickHoldingTheWorld.jpg" },

            { hid: 'twitter:title', property: 'twitter:title', content: "Firefang" },
            { hid: 'twitter:description', property: 'twitter:description', content: "Rick Yorgason's portfolio blog" },
            { hid: 'twitter:image', property: 'twitter:image', content: "/RickHoldingTheWorld.jpg" }
        ],
        link: [{rel: 'icon', type: 'image/x-icon', href: '/favicon.ico'}]
    }
}