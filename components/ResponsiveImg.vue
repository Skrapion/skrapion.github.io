<template>
    <div class="responsive">
        <img class="placeholder cover" :src="image.placeholder" aria-hidden="true" :width="image.width" :height="image.height"/>
        <img onload="loaded(this)" loading="lazy" class="actual" :class="fit" :src="image.src" :srcSet="image.srcSet" :alt="alt" :width="image.width" :height="image.height"/>
        <div :style="{width: '100%', 'padding-bottom': fit=='contain'?image.height/image.width*100+'%':'100%'}"/>
    </div>
</template>

<style scoped>
.responsive {
    position: relative;
    overflow: hidden;
}

.cover {
    object-fit: cover;
}

.contain {
    object-fit: contain;
}

.placeholder {
    filter: blur(15px);
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    width: 100%;
}

.actual {
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    width: 100%;
    transition: opacity 800ms ease 0ms;
    opacity: 0;
}

.loaded {
    opacity: 1;
}
</style>

<script>

export default {
    props: {
        'alt': String,
        'image': Object,
        'fit': {type: String, default: 'cover'}
    }
}
</script>