<template>
    <ul>
        <li v-for="post of posts" :key="post.slug"><nuxt-link :to="post.slug">{{post.title}}</nuxt-link></li>
    </ul>
</template>

<script>
export default {
    async asyncData({$content, params}){
        const posts = await $content('posts')
            .only(['title', 'featured', 'slug'])
            .sortBy('date', 'desc')
            .fetch();
        return {posts};
    }
}
</script>

<style scoped>

</style>