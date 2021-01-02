<template>
    <article id='singlecontent'>
        <div id='prettypictures' class='youtube' v-if='post.youtube'>
            <iframe :src="`https://www.youtube.com/embed/${post.youtube}`" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
        </div>
        <div id='prettypictures' v-else>
            <nuxt-image :src="`/posts/${post.slug}/cover.jpg`"/>
        </div>
        <div id='date'>
            {{formatDate(post.date)}}
        </div>
        <div id='boringwords'>
            <nuxt-content :document='post'/>
        </div>
        <div id='older' v-if="prev">
            <nuxt-link :to="prev.slug">&laquo; Older</nuxt-link>
        </div>
        <div id='newer' v-if="next">
            <nuxt-link :to="next.slug">Newer &raquo;</nuxt-link>
        </div>
    </article>
</template>

<script>
export default {
    async asyncData({$content, params, error}) {
        const postPromise = $content('posts', params.slug).fetch();
        const surroundPromise = $content('posts')
            .only(['slug'])
            .sortBy('date')
            .surround(params.slug)
            .fetch();

        try {
            const post = await postPromise;
            const [prev, next] = await surroundPromise;

            return {post, prev, next};
        } catch(e) {
            error({statusCode: 400, message: "Not Found"});
        }
    },
    methods: {
        formatDate(date) {
            const options = {year: 'numeric', month: 'long', day: 'numeric'};
            return new Date(date).toLocaleString('en', options);
        }
    },
    head() {
        return {
            title: this.post.title + " - Firefang"
        }
    }
}
</script>

<style scoped>
#singlecontent {
    display: grid;
    height: 100%;
    grid-template-columns: 66%;
    grid-template-areas:
            "pic .     ."
            "pic date  date"
            "pic desc  desc"
            "pic newer older"
            "pic .     .";
    grid-template-rows: auto min-content min-content min-content auto;
    font-size: 12pt;
    padding: 20px;
}

#prettypictures {
    grid-area: pic;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

#prettypictures .size-post-thumbnail {
    width: 100%;
    height: auto;
}

.youtube {
    position: relative;
    width: 100%;
    height: 0;
    padding-bottom: 56.25%;
    margin: 0;
}

.youtube iframe {
    position: absolute;
    width: 100%;
    height: 100%;
    left: 0;
    top: 0;
}

#date {
    grid-area: date;
    text-align: right;
    font-style: italic;
    height: auto;
}

#boringwords {
    grid-area: desc;
    margin-left: 2em;
    text-align: justify;
}

#older {
    grid-area: newer;
    margin-left: 2em;
}

#newer {
    grid-area: older;
    text-align: right;
}

#newer a, #older a {
    color: white;
    text-decoration: none;
}

#newer a:hover, #older a:hover {
    text-decoration: underline;
}
</style>