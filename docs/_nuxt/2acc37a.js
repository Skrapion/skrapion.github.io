(window.webpackJsonp=window.webpackJsonp||[]).push([[2],{202:function(t,e,r){var content=r(204);"string"==typeof content&&(content=[[t.i,content,""]]),content.locals&&(t.exports=content.locals);(0,r(56).default)("7b0ebfa2",content,!0,{sourceMap:!1})},203:function(t,e,r){"use strict";r(202)},204:function(t,e,r){var n=r(55),o=r(137),c=r(138);e=n(!1);var l=o(c);e.push([t.i,'#singlecontent article[data-v-3cb3aa58]{position:relative;display:grid;grid-template-columns:300px;grid-template-areas:"story pics" ".     signposts";font-size:12pt;padding:20px;z-index:1;border-bottom:17px solid transparent;-o-border-image:url('+l+") 17 repeat;border-image:url("+l+') 17 repeat}#story[data-v-3cb3aa58]{grid-area:story;padding-right:40px;max-height:100vh}#prettypictures[data-v-3cb3aa58],#story[data-v-3cb3aa58]{display:flex;flex-direction:column;justify-content:center}#prettypictures[data-v-3cb3aa58]{grid-area:pics;margin-bottom:20px}#signposts[data-v-3cb3aa58]{grid-area:signposts;display:grid;grid-template-areas:"nextsignpost . prevsignpost";grid-template-columns:150px auto 150px;padding-bottom:20px}#older[data-v-3cb3aa58]{grid-area:prevsignpost;text-align:right}#newer[data-v-3cb3aa58]{grid-area:nextsignpost}#newer a[data-v-3cb3aa58],#older a[data-v-3cb3aa58]{color:#fff;text-decoration:none}#newer a[data-v-3cb3aa58]:hover,#older a[data-v-3cb3aa58]:hover{text-decoration:underline}#similars[data-v-3cb3aa58]{min-height:20px;position:relative;margin-top:-17px;background:#262626;padding:1px 40px 40px;z-index:0}#prettypictures .size-post-thumbnail[data-v-3cb3aa58]{width:100%;height:auto}.pic[data-v-3cb3aa58]{margin-bottom:20px}.youtubewrapper[data-v-3cb3aa58]{position:relative;padding-top:56.25%}.youtubewrapper iframe[data-v-3cb3aa58]{position:absolute;top:0;left:0;width:100%;height:100%}#date[data-v-3cb3aa58]{grid-area:date;text-align:right;font-style:italic;height:auto}#boringwords[data-v-3cb3aa58]{grid-area:desc;margin-left:2em}#similars h2[data-v-3cb3aa58]{font-size:40px;font-style:italic;font-weight:100;font-family:"Rouge Script",cursive;margin-bottom:0}',""]),t.exports=e},220:function(t,e,r){"use strict";r.r(e);var n=r(9),o=(r(33),r(2)),c={asyncData:function(t){return Object(o.a)(regeneratorRuntime.mark((function e(){var r,o,c,l,d,h,m,v,f,x;return regeneratorRuntime.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return r=t.$content,o=t.params,c=t.error,l=r("posts",o.slug).fetch(),d=r("posts").only(["slug"]).sortBy("date").surround(o.slug).fetch(),e.prev=3,e.next=6,l;case 6:return h=e.sent,e.next=9,d;case 9:return m=e.sent,v=Object(n.a)(m,2),f=v[0],x=v[1],e.abrupt("return",{post:h,prev:f,next:x});case 16:e.prev=16,e.t0=e.catch(3),c({statusCode:404,message:"Not Found"});case 19:case"end":return e.stop()}}),e,null,[[3,16]])})))()},data:function(){return{similars:[],similarsCategory:""}},fetch:function(){var t=this;return Object(o.a)(regeneratorRuntime.mark((function e(){return regeneratorRuntime.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return e.next=2,t.$nuxt.context.$content("posts").only(["slug","tags","title"]).where({$and:[{slug:{$ne:t.$nuxt.context.params.slug}},{tags:{$contains:[t.post.tags[0]]}}]}).fetch();case 2:if(t.similars=e.sent,!(t.similars.length>3)){e.next=7;break}t.similarsCategory=t.post.tags[0],e.next=11;break;case 7:return e.next=9,t.$nuxt.context.$content("posts").only(["slug","tags","title"]).where({$and:[{slug:{$ne:t.$nuxt.context.params.slug}},{tags:{$containsAny:t.post.tags}}]}).fetch();case 9:t.similars=e.sent,t.similarsCategory="similar";case 11:case"end":return e.stop()}}),e)})))()},methods:{formatDate:function(t){return new Date(t).toLocaleString("en",{year:"numeric",month:"long",day:"numeric"})}},env:{BaseURL:"https://skrapion.gitlab.io/"},head:function(){return{title:this.post.title+" - Firefang",meta:[{hid:"description",name:"description",content:this.post.description},{hid:"og:title",property:"og:title",content:this.post.title+" - Firefang"},{hid:"og:url",property:"og:url",content:"https://nuxt.firefang.com/"+this.post.slug},{hid:"og:type",property:"og:type",content:this.post.youtube?"video":"article"},{hid:"og:description",property:"og:description",content:this.post.description},{hid:"og:image",property:"og:image",content:"https://nuxt.firefang.com/posts/"+this.post.slug+"/cover.jpg"},{hid:"twitter:title",property:"twitter:title",content:this.post.title+" - Firefang"},{hid:"twitter:description",property:"twitter:description",content:this.post.description},{hid:"twitter:image",property:"twitter:image",content:"https://nuxt.firefang.com/posts/"+this.post.slug+"/cover.jpg"}],link:[{rel:"preconnect",href:"https://fonts.gstatic.com"},{rel:"stylesheet",href:"https://fonts.googleapis.com/css2?family=Rouge+Script&display=swap"}]}}},l=(r(203),r(16)),component=Object(l.a)(c,(function(){var t=this,e=t.$createElement,r=t._self._c||e;return r("div",{attrs:{id:"singlecontent"}},[r("article",[r("div",{attrs:{id:"story"}},[r("div",{attrs:{id:"date"}},[t._v("\n                "+t._s(t.formatDate(t.post.date))+"\n            ")]),t._v(" "),r("div",{attrs:{id:"boringwords"}},[r("nuxt-content",{attrs:{document:t.post}})],1)]),t._v(" "),t.post.pics?r("div",{attrs:{id:"prettypictures"}},t._l(t.post.pics,(function(e){return r("div",{key:e},[".yt"==e.substr(e.length-3,e.length)?r("div",{staticClass:"pic youtubewrapper"},[r("iframe",{attrs:{src:"https://www.youtube.com/embed/"+e.substr(0,e.length-3),frameborder:"0",width:"100%",height:"100%",allow:"accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture",allowfullscreen:""}})]):r("div",{staticClass:"pic"},[r("nuxt-image",{attrs:{src:"/posts/"+t.post.slug+"/"+e,placeholder:!0}})],1)])})),0):r("div",{attrs:{id:"prettypictures"}},[r("nuxt-image",{attrs:{src:"/posts/"+t.post.slug+"/cover.jpg",placeholder:!0}})],1),t._v(" "),r("div",{attrs:{id:"signposts"}},[t.next?r("div",{staticClass:"post",attrs:{id:"newer"}},[r("nuxt-link",{attrs:{to:"/"+t.next.slug}},[r("nuxt-image",{staticClass:"signpostimg",attrs:{src:"/posts/"+t.next.slug+"/cover.jpg",placeholder:!0,width:"400",height:"400"}}),t._v(" "),r("div",{staticClass:"signposttext"},[t._v("« Newer")])],1)],1):t._e(),t._v(" "),t.prev?r("div",{staticClass:"post",attrs:{id:"older"}},[r("nuxt-link",{attrs:{to:"/"+t.prev.slug}},[r("nuxt-image",{staticClass:"signpostimg",attrs:{src:"/posts/"+t.prev.slug+"/cover.jpg",placeholder:!0,width:"400",height:"400"}}),t._v(" "),r("div",{staticClass:"signposttext"},[t._v("Older »")])],1)],1):t._e()])]),t._v(" "),r("div",{attrs:{id:"similars"}},[!t.$fetchState.pending&&t.similars.length?r("div",{staticClass:"indexcontentpadding"},[r("h2",[t._v("Other "+t._s(t.similarsCategory)+" projects...")]),t._v(" "),r("div",{attrs:{id:"indexcontent"}},t._l(t.similars,(function(t){return r("div",{key:t.slug,staticClass:"post"},[r("nuxt-link",{attrs:{to:"/"+t.slug}},[r("nuxt-image",{staticClass:"similarimg",attrs:{src:"/posts/"+t.slug+"/cover.jpg",placeholder:!0,alt:t.title,width:"400",height:"400"}})],1)],1)})),0)]):t._e()])])}),[],!1,null,"3cb3aa58",null);e.default=component.exports}}]);