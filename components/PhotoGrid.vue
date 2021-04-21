<template>
    <div id='indexcontent'>
        <div v-for="post of posts" :key="post.slug" :class="{post: true, featured: post.featured}"><nuxt-link :to="post.slug"><nuxt-image :src="`/posts/${post.slug}/cover.jpg`" :alt="post.title" width="400" height="400" :sizes="post.featured ? featuredSizes : sizes" :placeholder="true"/></nuxt-link></div>
    </div>
</template>

<script>
export default {
    props: [ 'posts' ],
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
    }
}
</script>