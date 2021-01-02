export default {
    target: 'static',
    head: {
        title: 'Firefang',
        meta: [
            {charset: 'utf-8'},
            {name: 'viewport', content: 'width=device-width, initial-scale=1'},
            {hid: `description`, name: 'description', content: "Rick Yorgason's portfolio blog"}
        ],
        link: [{rel: 'icon', type: 'image/x-icon', href: '/favicon.ico'}]
    },
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
    }
}