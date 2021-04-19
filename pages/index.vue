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
            .where({parent: {$type: {$ne: 'string'}}})
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
                { hid: 'og:url', property: "og:url", content: process.env.BaseURL },
                { hid: 'og:type', property: "og:type", content: "website" }
            ]
        }
    }
}
</script>

<style scoped>

</style>