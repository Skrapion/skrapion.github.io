<template>
    <div id='indexcontentpadding'>
        <div id='indexcontent'>
            <div v-for="post of posts" :key="post.slug" :class="{post: true, featured: post.featured}"><nuxt-link :to="post.slug"><nuxt-image :src="`/posts/${post.slug}/cover.jpg`" :alt="post.title" width="400" height="400" :sizes="post.featured ? featuredSizes : sizes" :placeholder="true"/></nuxt-link></div>
        </div>
    </div>
</template>

<script>
export default {
    async asyncData({$content, params}){
        const posts = await $content('posts')
            .only(['title', 'featured', 'slug'])
            .sortBy('date', 'desc')
            .fetch();
        return {posts};
    },
    data() {
        return {
            sizes: [{width: 400}],
            featuredSizes: [
                {
                    width: 400
                },
                {
                    breakpoint: 400,
                    width: 800
                }
            ]
        }
    },
    head() {
        return {
            meta: [
                { hid: 'og:url', property: "og:url", content: process.env.baseURL },
                { hid: 'og:type', property: "og:type", content: "website" }
            ]
        }
    }
}
</script>

<style scoped>
#indexcontentpadding {
    width: auto;
    margin: 20px;
}

#indexcontent {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, auto));
    grid-gap: 10px;
    grid-auto-rows: minmax(200px, auto);
    grid-auto-flow: dense;
}

.post a img {
    width: 100%;
    height: 100%;
}

.featured {
    grid-column: span 2;
    grid-row: span 2;
}

.post a {
    display: block;
    text-decoration: none;
    color: white;
    height: 100%;
    width: 100%;
}

.post:hover {
    transform: scale(1.05);
    z-index: 1;
}

.post {
    transition: all 1s;
    z-index: unset;
}
</style>