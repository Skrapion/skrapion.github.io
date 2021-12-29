<template>
    <div id='indexcontent'>
        <div v-for="post of posts" :key="post.slug" :class="{post: true, featured: post.featured}">
            <nuxt-link class='photosquare' :to="post.slug">
                <responsive-img v-if="post.featured"
                    :image="require(`~/assets/posts/${post.slug}/cover.jpg?{sizes:[200,400,800]}`)"
                    :alt="post.title"/>
                <responsive-img v-else
                    :image="require(`~/assets/posts/${post.slug}/cover.jpg?{sizes:[200,400]}`)"
                    :alt="post.title"/>
            </nuxt-link>
        </div>
    </div>
</template>

<script>
export default {
    props: [ 'posts' ]
}
</script>

<style scoped>
.photosquare {
    position: relative;
    overflow: hidden;
    border-radius: 20px;
}
.placeholder {
    object-fit: cover;
    filter: blur(15px);
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    width: 100%;
}

.actual {
    object-fit: cover;
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    width: 100%;
    transition: opacity 800ms ease 0ms;
    opacity: 0;
}

.loaded {
    opacity: 1;
}
</style>