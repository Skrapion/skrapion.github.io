<template>
    <article id='singlecontent'>
        <div id='story'>
            <div id='date'>
                {{formatDate(post.date)}}
            </div>
            <div id='boringwords'>
                <nuxt-content :document='post'/>
            </div>
        </div>
        <div id='prettypictures' v-if='post.pics'>
            <div v-for='pic of post.pics' :key='pic'>
                <div v-if='pic.substr(pic.length-3, pic.length) == ".yt"' class='pic youtubewrapper'>
                    <iframe :src="`https://www.youtube.com/embed/${pic.substr(0, pic.length-3)}`" frameborder="0" width="100%" height="100%" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
                </div>
                <div v-else class='pic'>
                    <nuxt-image :src="`/posts/${post.slug}/${pic}`" :placeholder="true"/>
                </div>
            </div>
        </div>
        <div id='prettypictures' v-else>
            <nuxt-image :src="`/posts/${post.slug}/cover.jpg`" :placeholder="true"/>
        </div>
        <div id='signposts'>
            <div id='newer' v-if="next">
                <nuxt-link :to="`/${next.slug}`">
                    <nuxt-image class="signpostimg" :src="`/posts/${next.slug}/cover.jpg`" :placeholder="true" width="400" height="400"/>
                    <div class="signposttext">&laquo; Newer</div>
                </nuxt-link>
            </div>
            <div id='older' v-if="prev">
                <nuxt-link :to="`/${prev.slug}`">
                    <nuxt-image class="signpostimg" :src="`/posts/${prev.slug}/cover.jpg`" :placeholder="true" width="400" height="400"/>
                    <div class="signposttext">Older &raquo;</div>
                </nuxt-link>
            </div>
        </div>
        <div v-if='!$fetchState.pending && similars.length' id='similars'>
            <h2>Other {{similarsCategory}} projects</h2>
            <div v-for="similar of similars" :key='similar.slug'>
                <nuxt-link :to="`/${similar.slug}`">
                    <nuxt-image class="similarimg" :src="`/posts/${similar.slug}/cover.jpg`" :placeholder="true" width="400" height="400"/>
                </nuxt-link>
            </div>
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
            error({statusCode: 404, message: "Not Found"});
        }
    },
    data() {
        return {
            similars: [],
            similarsCategory: ""
        }
    },
    async fetch() {
        this.similars = await this.$nuxt.context.$content('posts')
            .only(['slug', 'tags', 'title'])
            .where({
                $and: [{
                    'slug': { $ne: this.$nuxt.context.params.slug }
                },{
                    'tags': { $contains: [this.post.tags[0]] }
                }]
            })
            .fetch();
        if(this.similars.length > 3) {
            this.similarsCategory = this.post.tags[0];
        } else {
            this.similars = await this.$nuxt.context.$content('posts')
            .only(['slug', 'tags', 'title'])
            .where({
                $and: [{
                    'slug': { $ne: this.$nuxt.context.params.slug }
                },{
                    'tags': { $containsAny: this.post.tags }
                }]
            })
            .fetch();

            this.similarsCategory = "similar";
        }
    },
    methods: {
        formatDate(date) {
            const options = {year: 'numeric', month: 'long', day: 'numeric'};
            return new Date(date).toLocaleString('en', options);
        }
    },
    env: {
        "BaseURL": "https://skrapion.gitlab.io/"
    },
    head() {
        return {
            title: this.post.title + " - Firefang",
            meta: [
                { hid: 'description', name: 'description', content: this.post.description },

                { hid: 'og:title', property: "og:title", content: this.post.title + " - Firefang" },
                { hid: 'og:url', property: "og:url", content: process.env.BaseURL + this.post.slug },
                { hid: 'og:type', property: "og:type", content: this.post.youtube ? "video" : "article" },
                { hid: 'og:description', property: "og:description", content: this.post.description },
                { hid: 'og:image', property: 'og:image', content: process.env.BaseURL + "posts/" + this.post.slug + "/cover.jpg" },

                { hid: 'twitter:title', property: 'twitter:title', content: this.post.title + " - Firefang" },
                { hid: 'twitter:description', property: 'twitter:description', content: this.post.description },
                { hid: 'twitter:image', property: 'twitter:image', content: process.env.BaseURL + "posts/" + this.post.slug + "/cover.jpg" }
            ]
        }
    }
}
</script>

<style scoped>
#singlecontent {
    display: grid;
    grid-template-columns: 300px;
    grid-template-areas:
            "story pics"
            ".     signposts"
            "similars similars";
    font-size: 12pt;
    padding: 20px;
}

#story {
    grid-area: story;
    padding-right: 40px;
    max-height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

#prettypictures {
    grid-area: pics;
    margin-bottom: 20px;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

#signposts {
    grid-area: signposts;
    display: grid;
    grid-template-areas: "nextsignpost . prevsignpost";
    grid-template-columns: 150px auto 150px;
}

#older {
    grid-area: prevsignpost;
    text-align: right;
}

#newer {
    grid-area: nextsignpost;
}

#newer a, #older a {
    color: white;
    text-decoration: none;
}

#newer a:hover, #older a:hover {
    text-decoration: underline;
}

#similars {
    grid-area: similars;
}

#prettypictures .size-post-thumbnail {
    width: 100%;
    height: auto;
}

.pic {
    margin-bottom: 20px;
}

.youtubewrapper {
    position: relative;
    padding-top: 56.25%;
}

.youtubewrapper iframe {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
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
}

.similarimg {
    width: 200px;
}
</style>