<template>
    <div class='attribution'>
        Photo credit:
        <a v-if='contributors[name].site' :href='contributors[name].site'>{{contributors[name].fullname}}</a>
        <span v-else>{{contributors[name].fullname}}</span>
    </div>
</template>

<script>
export default {
    props: { name: String },
    data() {
        return {
            contributors: {
                'caitlyn': { fullname: "Caitlyn MacMaster" },
                'sean': { fullname: "Sean Montgomery", site: "https://artofsean.threadless.com/" }
            }
        }
    }
}
</script>

<style scoped>
.attribution {
    font-style: italic;
    text-align: right;
}

.attribution, .attribution a {
    color: #aaa;
}
</style>