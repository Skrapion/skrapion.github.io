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
                                <a href='#' class='button' @click="readmoreclicked = true">Read More</a>
                            </div>
                        </div>
                    </div>
                </div>
                <div id='indexcontent' v-if='children.length'>
                    <div v-for="child of children" :key="child.slug" :class="{post: true, featured: child.featured}"><nuxt-link :to="child.slug"><nuxt-img :src="`/posts/${child.slug}/cover.jpg`" :alt="child.title" width="400" height="400" :sizes="child.featured ? featuredSizes : sizes" :placeholder="true"/></nuxt-link></div>
                </div>
                <div id='prettypictures' v-else-if='post.pics'>
                    <div v-for='(pic, n) of post.pics' :key='n'>
                        <div v-if='pic.filename && pic.type == "youtube"' class='pic youtubewrapper'>
                            <iframe :src="`https://www.youtube.com/embed/${pic.filename}`" frameborder="0" width="100%" height="100%" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
                        </div>
                        <div v-else-if='pic.filename' class='pic'>
                            <nuxt-image :src="`/posts/${post.slug}/${pic.filename}`" :placeholder="true" sizes="200,200:400,400:800,800:1200,1200:1920"/>
                        </div>
                        <div v-else class='pic'>
                            <nuxt-image :src="`/posts/${post.slug}/${pic}`" :placeholder="true" sizes="200,200:400,400:800,800:1200,1200:1920"/>
                        </div>

                        <attribution v-if='pic.attrib' :name='pic.attrib'></attribution>
                    </div>
                </div>
                <div id='prettypictures' v-else>
                    <nuxt-image :src="`/posts/${post.slug}/cover.jpg`" :placeholder="true" sizes="200,200:400,400:800,800:1200,1200:1920"/>
                </div>
                <div id='signposts'>
                        <div id='newer' class='post'>
                            <nuxt-link id='newerlink' v-if="next" :to="{name: 'slug', params: {slug: next.slug, type: 'next'}}">
                                <nuxt-image class="signpostimg" :src="`/posts/${next.slug}/cover.jpg`" :placeholder="true" width="400" height="400" sizes="200"/>
                            </nuxt-link>
                            <nuxt-link id='newerlink' v-else-if="post.parent" :to="{name: 'slug', params: {slug: post.parent, type: 'next'}}">
                                <nuxt-image class="signpostimg" :src="`/posts/${post.parent}/cover.jpg`" :placeholder="true" width="400" height="400" sizes="200"/>
                            </nuxt-link>
                        </div>
                        <div id='older' class='post'>
                            <nuxt-link id='olderlink' v-if="prev" :to="{name: 'slug', params: {slug: prev.slug, type: 'prev'}}">
                                <nuxt-image class="signpostimg" :src="`/posts/${prev.slug}/cover.jpg`" :placeholder="true" width="400" height="400" sizes="200"/>
                            </nuxt-link>
                            <nuxt-link id='newerlink' v-else-if="post.parent" :to="{name: 'slug', params: {slug: post.parent, type: 'next'}}">
                                <nuxt-image class="signpostimg" :src="`/posts/${post.parent}/cover.jpg`" :placeholder="true" width="400" height="400" sizes="200"/>
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
                            <nuxt-image class="similarimg" :src="`/posts/${similar.slug}/cover.jpg`" :placeholder="true" :alt="similar.title" width="400" height="400" sizes="200,200:400,400:800"/>
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
                .only(['slug'])
                .sortBy('date')
                .surround(params.slug)
                .fetch();

            var similars = await $content('posts')
                .only(['slug', 'tags', 'title'])
                .where({
                    $and: [{
                        'parent': { $type: {$eq: 'undefined'}}
                    },{
                        'slug': { $ne: params.slug }
                    },{
                        'tags': { $contains: [post.tags[0]] }
                    }]
                })
                .fetch();
        
            var similarsCategory = "More " + post.tags[0] + " projects...";
            const cat1len = similars.length;

            // If we don't have enough interesting hits on the main category, check others.
            if(cat1len < 5) {
                similars = await $content('posts')
                    .only(['slug', 'tags', 'title'])
                    .where({
                        $and: [{
                            'parent': { $type: {$eq: 'undefined'}}
                        },{
                            'slug': { $ne: params.slug }
                        },{
                            'tags': { $containsAny: post.tags }
                        }]
                    })
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
            readmoreclicked: false,
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
    },
    methods: {
        formatDate(date) {
            const options = {year: 'numeric', month: 'long', day: 'numeric', timeZone: 'UTC'};
            return new Date(date).toLocaleString('en', options);
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
                { hid: 'og:image', property: 'og:image', content: process.env.BaseURL + "posts/" + this.post.slug + "/cover.jpg" },

                { hid: 'twitter:title', property: 'twitter:title', content: this.post.title + " - Firefang" },
                { hid: 'twitter:description', property: 'twitter:description', content: this.post.description },
                { hid: 'twitter:image', property: 'twitter:image', content: process.env.BaseURL + "posts/" + this.post.slug + "/cover.jpg" }
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
    padding: 0px 125px 20px 100px;
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

#prettypictures .size-post-thumbnail {
    width: 100%;
    height: auto;
}

.pic {
    padding-top: 20px;
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