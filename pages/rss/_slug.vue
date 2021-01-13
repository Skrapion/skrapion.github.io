<template>
    <nuxt-content :document='post'/>
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
