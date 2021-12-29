import { hostname } from 'os';

const fs = require('fs').promises;
const path = require('path');

const baseUrl = "https://nuxt.firefang.com/";
const desc = "Rick Yorgason's portfolio blog. Everything from traditional woodworking to video game development."

const constructFeedItem = async (post) => {
    const filePath = path.join(__dirname, `docs/rssgen/${post.slug}/index.html`);
    const content = await fs.readFile(filePath, 'utf8');
    const url = `${baseUrl}${post.slug}/`;
    return {
        title: post.title,
        date: new Date(post.date),
        image: `${baseUrl}posts/${post.slug}/cover.jpg`,
        id: url,
        link: url,
        description: post.description,
        content: content
    }
}

const fetchPosts = async () => {
    const {$content} = require('@nuxt/content');
    return $content('posts')
        .sortBy('date', 'desc')
        .fetch();
}

const create = async (feed, args) => {
    const posts = await fetchPosts();

    feed.options = {
        title: "Firefang",
        description: desc,
        link: `${baseUrl}rss.xml`,
        image: `${baseUrl}favicon.ico`
    };

    for(const post of posts) {
        const feedItem = await constructFeedItem(post);
        feed.addItem(feedItem);
    }
    return feed;
}

const createSitemapRoutes = async () => {
    const posts = await fetchPosts();

    let routes = [];

    for(const post of posts) {
        routes.push(`/${post.slug}`);
    }

    return routes;
}

export default {
    target: 'static',
    css: [
        "~/assets/styles/main.css"
    ],
    modules: [
        '@nuxt/content',
        '@nuxtjs/feed',
        '@nuxtjs/sitemap',
        'nuxt-responsive-loader'
    ],
    plugins: [
        { src: "@/plugins/vClickOutside", ssr: false },
        { src: "@/plugins/OneSignal", ssr: false }
    ],
    components: true,
    generate: {
        dir: 'docs',
        fallback: '404.html',
        nestedProperties: ['posts.tags'],
        routes: ['/rssgen/']
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
        link: [
            {rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' },
            {rel: 'alternate', type: 'application/rss+xml', title: 'RSS Feed for Firefang', href: '/rss.xml' },
            {rel: 'preconnect', href: 'https://fonts.gstatic.com'},
            {rel: 'stylesheet', href: 'https://fonts.googleapis.com/css2?family=Rouge+Script&display=swap'},
            {rel: 'stylesheet', href: 'https://fonts.googleapis.com/css2?family=Fraunces:ital,wght@0,300;1,100;1,300&display=swap'},
            {rel: 'stylesheet', href: 'https://fonts.googleapis.com/css2?family=Quicksand:wght@300&display=swap'}
        ],
        script: [
            {
                src: "https://cdn.onesignal.com/sdks/OneSignalSDK.js",
                async: true
            },
            {
                src: '/custom.js'
            }
        ]
    },
    feed: [
        {
            path: '/rss.xml',
            create,
            cacheTime: 1000 * 60 * 15,
            type: 'rss2'
        }
    ],
    sitemap: {
        hostname: baseUrl,
        gzip: true,
        exclude: ['/rssgen'],
        routes: createSitemapRoutes
    },
    responsiveLoader: {
        adapter: require('responsive-loader/sharp'),
        sizes: process.env.NODE_ENV !== 'production' ? [800] : [200,400,800,1200,1920],
        placeholder: true,
        placeholderSize: 20
    }
}