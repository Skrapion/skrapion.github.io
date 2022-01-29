<template>
    <div id='singlecontent'>
        <div id='articlecontainer'>
            <article>
                <div id='story' :class="{openfull: readmoreclicked}">
                    <div id='storycontainer'>
                        <div id='boringwords'>
                            <nuxt-content :document='post'/>
                        </div>
                        <div id='date'>
                            {{formatDate(post.date)}}
                        </div>
                        <div id='readmorecontainer'>
                            <div id='readmore'>
                                <a href='#' class='button' @click="readmore()">Read More</a>
                            </div>
                        </div>
                    </div>
                </div>
                <div id='prettypictures'>
                    <div v-if='post.pics && post.pics != "none"'>
                        <div v-for='(pic, n) of post.pics' :key='n' class='picbox'>
                            <div v-if='pic.filename && pic.type == "youtube"' class='pic youtubewrapper'>
                                <iframe :src="`https://www.youtube.com/embed/${pic.filename}`" frameborder="0" width="100%" height="100%" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
                            </div>
                            <div v-else-if='pic.filename' class='pic' onclick="lightboxOpen(this)">
                                <responsive-img
                                    :image="require(`~/assets/posts/${post.slug}/${pic.filename}`)"
                                    fit="contain"
                                    class='prettyimg'/>
                            </div>
                            <div v-else class='pic' onclick="lightboxOpen(this)">
                                <responsive-img
                                    :image="require(`~/assets/posts/${post.slug}/${pic}`)"
                                    fit="contain"
                                    class='prettyimg'/>
                            </div>

                            <attribution v-if='pic.credit' :name='pic.credit'></attribution>
                        </div>
                    </div>
                    <div v-else-if='post.pics != "none"' class='picbox'>
                        <div class='pic' onclick="lightboxOpen(this)">
                            <responsive-img
                                :image="require(`~/assets/posts/${post.slug}/cover.jpg`)"
                                fit="contain"
                                class='prettyimg'/>
                        </div>

                    </div>

                    <photo-grid :posts='children'></photo-grid>
                </div>
                <div id='signposts'>
                        <div id='newer' class='post'>
                            <nuxt-link id='newerlink' v-if="next" :to="{name: 'slug', params: {slug: next.slug, type: 'next'}}">
                                <responsive-img
                                    :image="require(`~/assets/posts/${next.slug}/cover.jpg?size=200`)"
                                    :alt="`Newer: ${next.title}`"
                                    class='signpostimg'/>
                            </nuxt-link>
                            <nuxt-link id='newerlink' v-else-if="post.parent" :to="{name: 'slug', params: {slug: post.parent, type: 'next'}}">
                                <responsive-img
                                    :image="require(`~/assets/posts/${post.parent}/cover.jpg?size=200`)"
                                    alt="Back to project"
                                    class='signpostimg'/>
                            </nuxt-link>
                        </div>
                        <div id='older' class='post'>
                            <nuxt-link id='olderlink' v-if="prev" :to="{name: 'slug', params: {slug: prev.slug, type: 'prev'}}">
                                <responsive-img
                                    :image="require(`~/assets/posts/${prev.slug}/cover.jpg?size=200`)"
                                    :alt="`Older: ${prev.title}`"
                                    class='signpostimg'/>
                            </nuxt-link>
                            <nuxt-link id='newerlink' v-else-if="post.parent" :to="{name: 'slug', params: {slug: post.parent, type: 'next'}}">
                                <responsive-img
                                    :image="require(`~/assets/posts/${post.parent}/cover.jpg?size=200`)"
                                    alt="Back to project"
                                    class='signpostimg'/>
                            </nuxt-link>
                        </div>
                    </div>
            </article>
        </div>
        <div id='similars'>
            <div v-if='similars.length' class="indexcontentpadding">
                <h2>{{similarsCategory}}</h2>
                <div id="indexcontent">
                    <div v-for="similar of similars" :key='similar.slug' class='post'>
                        <nuxt-link :to="`/${similar.slug}`">
                            <responsive-img
                                class="similarimg"
                                :image="require(`~/assets/posts/${similar.slug}/cover.jpg?{sizes:[200,400]}`)"
                                :alt="similar.title"/>
                        </nuxt-link>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import ResponsiveImg from '../components/ResponsiveImg.vue';
export default {
  components: { ResponsiveImg },
    mounted() {
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

        // Touch events for next/prev post
        const dragParams = {
            yCancel: 50,
            xActivate: 100,
            friction: 4,
            dragMax: 25
        }
        const content = document.getElementById("singlecontent");
        content.style.transform = null;
        var touchStart = null;

        content.addEventListener('touchstart', event => {
            if(touchStart == null) {
                var changedTouch = event.changedTouches[0];
                touchStart = {
                    identifier: changedTouch.identifier,
                    clientX: changedTouch.clientX,
                    clientY: changedTouch.clientY
                };
            }
        });
        content.addEventListener('touchmove', event => {
            if(touchStart != null) {
                for(const changedTouch of event.changedTouches) {
                    if(touchStart.identifier == changedTouch.identifier) {
                        var diffY = changedTouch.clientY - touchStart.clientY;
                        if(Math.abs(diffY) > dragParams.yCancel) {
                            // Cancel touch; scrolled too far.
                            content.style.transform = null;
                            touchStart = null;
                        } else {
                            var diffX = changedTouch.clientX - touchStart.clientX;
                            content.style.transform = "translate3d(" 
                                + Math.max(Math.min(diffX/dragParams.friction, dragParams.dragMax), dragParams.dragMax*-1) 
                                + "px, 0, 0)";
                        }
                        return;
                    }
                }
            }
        });
        content.addEventListener('touchcancel', event => {
            if(touchStart != null) {
                for(const changedTouch of event.changedTouches) {
                    if(touchStart.identifier == changedTouch.identifier) {
                        content.style.transform = null;
                        touchStart = null;
                        return;
                    }
                }
            }
        });
        content.addEventListener('touchend', event => {
            if(touchStart != null) {
                for(const changedTouch of event.changedTouches) {
                    if(touchStart.identifier == changedTouch.identifier) {
                        var diff = changedTouch.clientX - touchStart.clientX;
                        var clicked = false;
                        if(Math.abs(diff) > dragParams.xActivate) {
                            if(diff < 0) {
                                var olderlink = document.getElementById('olderlink');
                                if(olderlink) {
                                    olderlink.click();
                                    clicked = true;
                                }
                            } else if(diff > 0) {
                                var newerlink = document.getElementById('newerlink');
                                if(newerlink) {
                                    newerlink.click();
                                    clicked = true;
                                }
                            }
                        }
                        
                        if(clicked == false) {
                            content.style.transform = null;
                        }
                        touchStart = null;
                        return;
                    }
                }
            }
        });
    },
    async asyncData({$content, params, error}) {
        const postPromise = $content('posts', params.slug).fetch();
        const childrenPromise = $content('posts')
            .where({parent: params.slug})
            .only(['title', 'featured', 'slug'])
            .sortBy('date', 'desc')
            .fetch();

        try {
            const post = await postPromise;
            
            const surroundPromise = $content('posts')
                .where({parent: post.parent ? post.parent : {$type: {$eq: 'undefined'}}})
                .only(['slug', 'title'])
                .sortBy('date')
                .surround(params.slug)
                .fetch();

            var wheres = [{
                    'tags': { $contains: [post.tags[0]] }
                },{
                    'parent': { $type: {$eq: 'undefined'}}
                },{
                    'slug': { $ne: params.slug }
                }];
            
            if(post.parent) {
                wheres.push({'slug': { $ne: post.parent}});
            }

            var similars = await $content('posts')
                .only(['slug', 'tags', 'title'])
                .where({$and: wheres})
                .fetch();
        
            var similarsCategory = "More " + post.tags[0] + " projects...";
            const cat1len = similars.length;

            // If we don't have enough interesting hits on the main category, check others.
            if(cat1len < 5) {
                wheres[0] = {'tags': { $containsAny: post.tags }};
                similars = await $content('posts')
                    .only(['slug', 'tags', 'title'])
                    .where({$and: wheres})
                    .fetch();

                // Assuming we actually found more posts, change the category name.
                if(cat1len != similars.length)
                    similarsCategory = "More projects like this...";
            }

            const [prev, next] = await surroundPromise;
            const children = await childrenPromise;

            return {post, prev, next, children, similars, similarsCategory};
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
    methods: {
        formatDate(date) {
            const options = {year: 'numeric', month: 'long', day: 'numeric', timeZone: 'UTC'};
            return new Date(date).toLocaleString('en', options);
        },
        readmore() {
            this.readmoreclicked = true;

            // max-height transitions don't work with 'auto' height, so we need to measure the end height.
            var story = document.getElementById("story");
            var height = Array.prototype.reduce.call(story.childNodes, function(p, c) {return p + (c.offsetHeight || 0);}, 0) + 'px';
            story.style.maxHeight = height;

            window.setTimeout(function(){story.style.maxHeight = '100%';}, 1000); // After transition is done, reset max-height to 100%
        }
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
                { hid: 'og:image', property: 'og:image', content: process.env.BaseURL + this.post.slug + "/cover.jpg" }
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
  transition: opacity 0.2s, transform 0.3s !important;
}
.slide-left-enter,
.slide-right-leave-to {
  opacity: 0;
  transform: translate3d(100px, 0, 0) !important;
}
.slide-right-enter,
.slide-left-leave-to {
  opacity: 0;
  transform: translate3d(-100px, 0, 0) !important;
}

#singlecontent {
    display: flex;
    flex-direction: column;
    min-height: 100vh;

    margin-left: -100px;
    margin-right: -100px;

    transition: opacity 0.2s, transform 0.3s;
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
    padding: 20px 125px 20px 100px;
}

#story {
    grid-area: story;
    padding-right: 40px;
    font-family: 'Fraunces', serif;
    font-weight: 300;
    min-height: min(100%, 100vh);
}

#storycontainer {
    display: flex;
    flex-direction: column;
    justify-content: center;
    min-height: min(100%, 100vh);
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

.prettyimg {
    border-radius: 20px;
    cursor: zoom-in;
}

#lightbox .prettyimg {
    height: 100%;
}

#signposts {
    display: flex;
    grid-area: signposts;
    justify-content: space-between;
    padding-top: 20px;
    padding-bottom: 20px;
}

#newer, #older {
    width: 100px;
}

#newer {
    padding-right: 10px;
}

#older {
    padding-left: 10px;
}

#newer .signpostimg {
    clip-path: polygon(100% 0%, 75% 50%, 100% 100%, 25% 100%, 0% 50%, 25% 0%);
    margin-left: -33%;
}

#older .signpostimg {
    clip-path: polygon(75% 0%, 100% 50%, 75% 100%, 0% 100%, 25% 50%, 0% 0%);
    margin-right: -33%;
}

#similars {
    flex-grow: 2;
    position: relative;
    margin-top: -17px;
    background: #262626;
    padding: 1px 140px 40px 140px;
    z-index: 0;
}

.similarimg {
    border-radius: 20px;
}

#prettypictures .size-post-thumbnail {
    width: 100%;
    height: auto;
}

.picbox {
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
    font-family: 'Fraunces', serif;
    font-weight: 100;
    font-size: 1.25em;
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

    #articlecontainer {
        padding-left: 125px;
    }

    #story {
        display: block;
        position: relative;
        padding-right: 0;
        overflow: hidden;
        max-height: 100vh;
        
        transition: max-height 1s ease-out;
    }

    #story.storyjs {
        max-height: 150px;
        
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
        padding-left: 120px;
        padding-right: 120px;
    }
    #indexcontentpadding {
        max-width: 500px;
    
        margin-left: auto;
        margin-right: auto;
    }
}
</style>