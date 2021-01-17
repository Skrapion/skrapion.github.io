<template>
    <div id='singlecontent'>
        <div id='articlecontainer'>
            <article>
                <div id='story' :class="{openfull: readmoreclicked}">
                    <div id='boringwords'>
                        <nuxt-content :document='post'/>
                    </div>
                    <div id='date'>
                        {{formatDate(post.date)}}
                    </div>
                    <div id='readmorecontainer'>
                        <div id='readmore'>
                            <a href='#' class='button' @click="readmoreclicked = true">Read More</a>
                        </div>
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
                        <div id='newer' class='post'>
                            <nuxt-link v-if="next" :to="{name: 'slug', params: {slug: next.slug, type: 'next'}}">
                                <nuxt-image class="signpostimg" :src="`/posts/${next.slug}/cover.jpg`" :placeholder="true" width="400" height="400"/>
                                <div class="signposttext">&laquo; Newer</div>
                            </nuxt-link>
                        </div>
                        <div id='older' class='post'>
                            <nuxt-link v-if="prev" :to="{name: 'slug', params: {slug: prev.slug, type: 'prev'}}">
                                <nuxt-image class="signpostimg" :src="`/posts/${prev.slug}/cover.jpg`" :placeholder="true" width="400" height="400"/>
                                <div class="signposttext">Older &raquo;</div>
                            </nuxt-link>
                        </div>
                    </div>
            </article>
        </div>
        <div id='similars'>
            <div v-if='!$fetchState.pending && similars.length' class="indexcontentpadding">
                <h2>Other {{similarsCategory}} projects...</h2>
                <div id="indexcontent">
                    <div v-for="similar of similars" :key='similar.slug' class='post'>
                        <nuxt-link :to="`/${similar.slug}`">
                            <nuxt-image class="similarimg" :src="`/posts/${similar.slug}/cover.jpg`" :placeholder="true" :alt="similar.title" width="400" height="400"/>
                        </nuxt-link>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    mounted() {
        document.documentElement.style.overflowX = 'hidden';

        // Show/hide "read more" button depending on how big the text is
        var story = document.getElementById("story");

        if(story) {
            var ro = new ResizeObserver(entries => {
                var height = story.offsetHeight;
                var readmore = document.getElementById("readmore");
                if(readmore) {
                    readmore.style.display =
                        (height < 150) ? "none" : "block";
                }
            });
            story.classList.add("storyjs");
            ro.observe(story);
        }
    },
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
            similarsCategory: "",
            readmoreclicked: false
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
            ],
            link: [
                {rel: 'preconnect', href: 'https://fonts.gstatic.com'},
                {rel: 'stylesheet', href: 'https://fonts.googleapis.com/css2?family=Rouge+Script&display=swap'},
            ]
        }
    },
    transition(to, from) {
        if(!to.params.type) {
            return 'page';
        }
        if(to.params.type == 'next') {
            return 'slide-right';
        }
        return 'slide-left';
    }
}
</script>

<style scoped>
.slide-left-enter-active,
.slide-left-leave-active,
.slide-right-enter-active,
.slide-right-leave-active {
  transition: opacity 0.2s, transform 0.3s;
}
.slide-left-enter,
.slide-right-leave-to {
  opacity: 0;
  transform: translate3d(100px, 0, 0);
}
.slide-right-enter,
.slide-left-leave-to {
  opacity: 0;
  transform: translate3d(-100px, 0, 0);
}

#singlecontent {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
}

#singlecontent article {
    position: relative;
    display: grid;
    grid-template-columns: 300px;
    grid-template-areas:
            "story pics"
            ".     signposts";
    font-size: 12pt;
    padding: 20px;
}

#articlecontainer {
    z-index: 1;
    border-bottom: 17px solid transparent;
    border-image: url(~assets/images/halftone.png) 17 repeat;
}

#story {
    grid-area: story;
    padding-right: 40px;
    max-height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

#readmorecontainer {
    display: none;
}

#prettypictures {
    grid-area: pics;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

#signposts {
    display: flex;
    grid-area: signposts;
    justify-content: space-between;
    padding-top: 20px;
    padding-bottom: 20px;
}

#newer, #older {
    width: 200px;
}

#newer {
    padding-right: 5px;
}

#older {
    padding-left: 5px;
    text-align: right;
}

#newer a, #older a {
    color: white;
    text-decoration: none;
}

#newer a:hover, #older a:hover {
    text-decoration: underline;
}

#similars {
    flex-grow: 2;
    position: relative;
    margin-top: -17px;
    background: #262626;
    padding-top: 1px;
    padding-left: 40px;
    padding-right: 40px;
    padding-bottom: 40px;
    z-index: 0;
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
    margin-top: -0.75em;
    margin-bottom: 20px;
    text-align: right;
    font-style: italic;
    height: auto;
}

#boringwords {
    grid-area: desc;
    margin-left: 2em;
}

#similars h2 {
    font-size: 40px;
    font-size: min(40px, 8vw);
    font-style: italic;
    font-weight: 100;
    font-family: 'Rouge Script', cursive;
    margin-bottom: 0;
}

/* 768 wide */
@media only screen and (max-width: 768px) {
    #contentcontainer article {
        display: flex;
        flex-direction: column;

        max-width: 500px;
        
        margin-left: auto;
        margin-right: auto;
    }

    #story {
        display: block;
        position: relative;
        padding-right: 0;
        overflow: hidden;

        
        transition: max-height 1s ease-out;
    }

    #story.storyjs {
        max-height: 150px;
        
    }

    #story.openfull {
        max-height: auto;
    }

    #readmorecontainer {
        display: block;
        position: absolute;
        bottom: 0;
        left: 0;
        width: 100%;
        margin: 0;
        padding: 50px 0;
        text-align: center;

        opacity: 1;
        visibility: visible;
        transition: opacity 1s ease-out, visibility 1s;
    }

    #story.openfull #readmorecontainer {
        opacity: 0;
        visibility: hidden;
    }

    #readmore {
        position: absolute;
        bottom: 0;
        left: 0;
        width: 100%;
        margin: 0;
        padding: 50px 0;

        background-image: -webkit-gradient(
            linear,
            left top,
            left bottom,
            color-stop(0, #1d1d1d00),
            color-stop(0.4, #1d1d1dcc),
            color-stop(1, #1d1d1dff));
    }

    #readmore a {
        display: block;
        position: absolute;
        bottom: 20px;
        width: 100%;
        text-align: center;
    }

    #boringwords {
        margin-left: 0;
    }
}

/* 600 wide */
@media only screen and (max-width: 600px) {
    #similars {
        padding-left: 20px;
        padding-right: 20px;
    }
    #indexcontentpadding {
        max-width: 500px;
    
        margin-left: auto;
        margin-right: auto;
    }
}
</style>