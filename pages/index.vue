<template>
    <div id='indexcontentpadding'>
        <photo-grid :posts='posts'></photo-grid>
    </div>
</template>

<script>
export default {
    async asyncData({$content, params}){
        const posts = await $content('posts')
            .where({parent: {$type: {$eq: 'undefined'}}})
            .only(['title', 'featured', 'slug'])
            .sortBy('date', 'desc')
            .fetch();
        return {posts};
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