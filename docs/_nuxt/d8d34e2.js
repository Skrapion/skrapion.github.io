(window.webpackJsonp=window.webpackJsonp||[]).push([[3],{224:function(t,e,n){"use strict";n.r(e);var r=n(2),o=(n(37),{asyncData:function(t){return Object(r.a)(regeneratorRuntime.mark((function e(){var n,r;return regeneratorRuntime.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return n=t.$content,t.params,e.next=3,n("posts").where({parent:{$type:{$eq:"undefined"}}}).only(["title","featured","slug"]).sortBy("date","desc").fetch();case 3:return r=e.sent,e.abrupt("return",{posts:r});case 5:case"end":return e.stop()}}),e)})))()},data:function(){return{sizes:[{width:400}],featuredSizes:[{width:400},{breakpoint:400,width:800}]}},head:function(){return{meta:[{hid:"og:url",property:"og:url",content:"https://nuxt.firefang.com/"},{hid:"og:type",property:"og:type",content:"website"}]}}}),c=n(18),component=Object(c.a)(o,(function(){var t=this,e=t.$createElement,n=t._self._c||e;return n("div",{attrs:{id:"indexcontentpadding"}},[n("div",{attrs:{id:"indexcontent"}},t._l(t.posts,(function(e){return n("div",{key:e.slug,class:{post:!0,featured:e.featured}},[n("nuxt-link",{attrs:{to:e.slug}},[n("nuxt-image",{attrs:{src:"/posts/"+e.slug+"/cover.jpg",alt:e.title,width:"400",height:"400",sizes:e.featured?t.featuredSizes:t.sizes,placeholder:!0}})],1)],1)})),0)])}),[],!1,null,"718c3006",null);e.default=component.exports}}]);