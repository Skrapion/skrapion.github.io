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
        image: baseUrl + "favicon.ico",
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
        link: `${baseUrl}rss.xml`
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
        '@nuxt/image',
        '@nuxtjs/feed',
        '@nuxtjs/sitemap'
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
            {rel: 'alternate', type: 'application/rss+xml', title: 'RSS Feed for Firefang', href: '/rss.xml' }
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
    }
}