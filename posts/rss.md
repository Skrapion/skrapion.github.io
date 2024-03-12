---
template_content: rawbody
template_root: rawcontent
parent: "hidden"
extension: xml
posts_by: by_postdate
skip_content: true
default_thumbnail: true
---
<rss xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:content="http://purl.org/rss/1.0/modules/content/" version="2.0">
    <channel>
        <title>Firefang</title>
        <link>https://firefang.com</link>
        <description>Rick Yorgason's portfolio blog. Everything from traditional woodworking to video game development.</description>
        <lastBuildDate>{{#with (lookup children 0)}}{{format-date postdate "%a, %d %b %Y 00:00:00 -0700"}}{{/with}}</lastBuildDate>
        <docs>https://validator.w3.org/feed/docs/rss2.html</docs>
        <image>
            <title>Firefang</title>
            <url>https://firefang.com/favicon.ico</url>
            <link>https://firefang.com</link>
        </image>
        {{#each children}}
            <item>
                <title>{{title}}</title>
                <link>https://firefang.com/{{slug}}/</link>
                <guid>https://firefang.com/{{slug}}/</guid>
                <pubDate>{{format-date postdate "%a, %d %b %Y 00:00:00 -0700"}}</pubDate>
                <description>{{description}}</description>
                <content:encoded>{{body}}</content:encoded>
                <enclosure url="https://firefang.com/{{slug}}/ogImage.jpg" length="0" type="image/jpg"/>
            </item>
        {{/each}}
   </channel>
</rss>
