<template>
    <div id='headercontainer'>
        <div id='header'>
            <div id='logo'><nuxt-link to='/'><img src='~/assets/images/hellcat.svg' width='250' alt='Home'/></nuxt-link></div>
            <div id='headericons'>
                <a href='mailto:rick@firefang.com'><img src='~/assets/images/icons/mail.svg' :height='iconheight' alt='email'/></a>
                <a href='https://facebook.com/skrapion'><img src='~/assets/images/icons/facebook.svg' :height='iconheight' alt='Facebook'/></a>
                <a href='https://www.youtube.com/channel/UCLCEtL5d5botNpR4TCU_f9w'><img src='~/assets/images/icons/youtube.svg' :height='iconheight' alt='Youtube'/></a>
                <a href='#'><img src='~/assets/images/icons/tshirt.svg' :height='iconheight' alt='Shirts!'/></a>
                <a href='#'><img src='~/assets/images/icons/bell.svg' :height='iconheight' alt='notifications'/></a>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    data() {
        return {
            iconheight: 24
        }
    }
}
</script>