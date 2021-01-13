<template>
    <div id='rsscontent'>
        <nuxt-content :document='post'/>
        <nuxt-image :src="`/posts/${post.slug}/cover.jpg`" :lazy='false' :sizes='200' />
    </div>
</template>

<script>
export default {
    async asyncData({$content, params, error}) {
        const post = await $content('posts', params.slug).fetch();
        return {post};
    },
    layout: 'empty'
}
</script>
