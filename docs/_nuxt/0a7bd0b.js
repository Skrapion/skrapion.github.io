(window.webpackJsonp=window.webpackJsonp||[]).push([[2],{202:function(t,e,r){var content=r(206);"string"==typeof content&&(content=[[t.i,content,""]]),content.locals&&(t.exports=content.locals);(0,r(55).default)("143641b5",content,!0,{sourceMap:!1})},205:function(t,e,r){"use strict";r(202)},206:function(t,e,r){(e=r(54)(!1)).push([t.i,'#singlecontent[data-v-446b8eb3]{display:grid;height:100%;grid-template-columns:300px;grid-template-areas:"story pics" "story signposts";font-size:12pt;padding:20px}#story[data-v-446b8eb3]{grid-area:story;margin-right:40px}#prettypictures[data-v-446b8eb3]{grid-area:pics}#signposts[data-v-446b8eb3]{grid-area:signposts;display:grid;grid-template-areas:"nextsignpost . prevsignpost";grid-template-columns:150px auto 150px}#older[data-v-446b8eb3]{grid-area:prevsignpost;text-align:right}#newer[data-v-446b8eb3]{grid-area:nextsignpost}#newer a[data-v-446b8eb3],#older a[data-v-446b8eb3]{color:#fff;text-decoration:none}#newer a[data-v-446b8eb3]:hover,#older a[data-v-446b8eb3]:hover{text-decoration:underline}#prettypictures .size-post-thumbnail[data-v-446b8eb3]{width:100%;height:auto}#youtubewrapper[data-v-446b8eb3]{position:relative;padding-top:56.25%}#youtubewrapper iframe[data-v-446b8eb3]{position:absolute;top:0;left:0;width:100%;height:100%}#date[data-v-446b8eb3]{grid-area:date;text-align:right;font-style:italic;height:auto}#boringwords[data-v-446b8eb3]{grid-area:desc;margin-left:2em;text-align:justify}',""]),t.exports=e},220:function(t,e,r){"use strict";r.r(e);var o=r(9),n=(r(33),r(2)),d={asyncData:function(t){return Object(n.a)(regeneratorRuntime.mark((function e(){var r,n,d,c,l,h,v,m,f,y;return regeneratorRuntime.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return r=t.$content,n=t.params,d=t.error,c=r("posts",n.slug).fetch(),l=r("posts").only(["slug"]).sortBy("date").surround(n.slug).fetch(),e.prev=3,e.next=6,c;case 6:return h=e.sent,e.next=9,l;case 9:return v=e.sent,m=Object(o.a)(v,2),f=m[0],y=m[1],e.abrupt("return",{post:h,prev:f,next:y});case 16:e.prev=16,e.t0=e.catch(3),d({statusCode:404,message:"Not Found"});case 19:case"end":return e.stop()}}),e,null,[[3,16]])})))()},methods:{formatDate:function(t){return new Date(t).toLocaleString("en",{year:"numeric",month:"long",day:"numeric"})}},env:{BaseURL:"https://skrapion.gitlab.io/"},head:function(){return{title:this.post.title+" - Firefang",meta:[{hid:"description",name:"description",content:this.post.description},{hid:"og:title",property:"og:title",content:this.post.title+" - Firefang"},{hid:"og:url",property:"og:url",content:"https://nuxt.firefang.com/"+this.post.slug},{hid:"og:type",property:"og:type",content:this.post.youtube?"video":"article"},{hid:"og:description",property:"og:description",content:this.post.description},{hid:"og:image",property:"og:image",content:"https://nuxt.firefang.com/posts/"+this.post.slug+"/cover.jpg"},{hid:"twitter:title",property:"twitter:title",content:this.post.title+" - Firefang"},{hid:"twitter:description",property:"twitter:description",content:this.post.description},{hid:"twitter:image",property:"twitter:image",content:"https://nuxt.firefang.com/posts/"+this.post.slug+"/cover.jpg"}]}}},c=(r(205),r(22)),component=Object(c.a)(d,(function(){var t=this,e=t.$createElement,r=t._self._c||e;return r("article",{attrs:{id:"singlecontent"}},[r("div",{attrs:{id:"story"}},[r("div",{attrs:{id:"date"}},[t._v("\n            "+t._s(t.formatDate(t.post.date))+"\n        ")]),t._v(" "),r("div",{attrs:{id:"boringwords"}},[r("nuxt-content",{attrs:{document:t.post}})],1)]),t._v(" "),t.post.youtube?r("div",{attrs:{id:"prettypictures"}},[r("div",{attrs:{id:"youtubewrapper"}},[r("iframe",{attrs:{src:"https://www.youtube.com/embed/"+t.post.youtube,frameborder:"0",width:"100%",height:"100%",allow:"accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture",allowfullscreen:""}})])]):r("div",{attrs:{id:"prettypictures"}},[r("nuxt-image",{attrs:{src:"/posts/"+t.post.slug+"/cover.jpg",placeholder:!0}})],1),t._v(" "),r("div",{attrs:{id:"signposts"}},[t.next?r("div",{attrs:{id:"newer"}},[r("nuxt-link",{attrs:{to:t.next.slug}},[r("nuxt-image",{staticClass:"signpostimg",attrs:{src:"/posts/"+t.next.slug+"/cover.jpg",placeholder:!0,width:"400",height:"400"}}),t._v(" "),r("div",{staticClass:"signposttext"},[t._v("« Newer")])],1)],1):t._e(),t._v(" "),t.prev?r("div",{attrs:{id:"older"}},[r("nuxt-link",{attrs:{to:t.prev.slug}},[r("nuxt-image",{staticClass:"signpostimg",attrs:{src:"/posts/"+t.prev.slug+"/cover.jpg",placeholder:!0,width:"400",height:"400"}}),t._v(" "),r("div",{staticClass:"signposttext"},[t._v("Older »")])],1)],1):t._e()])])}),[],!1,null,"446b8eb3",null);e.default=component.exports}}]);