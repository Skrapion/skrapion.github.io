(window.webpackJsonp=window.webpackJsonp||[]).push([[3],{208:function(t,e,n){"use strict";n.r(e);var r={props:["posts"],data:function(){return{sizes:[{width:400}],featuredSizes:[{width:400},{breakpoint:400,width:800}]}}},l=n(16),component=Object(l.a)(r,(function(){var t=this,e=t.$createElement,n=t._self._c||e;return n("div",{attrs:{id:"indexcontent"}},t._l(t.posts,(function(e){return n("div",{key:e.slug,class:{post:!0,featured:e.featured}},[n("nuxt-link",{attrs:{to:e.slug}},[n("nuxt-image",{attrs:{src:"/posts/"+e.slug+"/cover.jpg",alt:e.title,width:"400",height:"400",sizes:e.featured?t.featuredSizes:t.sizes,placeholder:!0}})],1)],1)})),0)}),[],!1,null,null,null);e.default=component.exports}}]);