<template>
    <div id="bodycontainer">
        <MainNav/>
        <div id="contentcontainer">
            <Nuxt/>
        </div>
    </div>
</template>

<script>
export default {
    head() {
        return {
            script: [
                {
                    src: "https://cdn.onesignal.com/sdks/OneSignalSDK.js",
                    async: true
                }
            ],
            __dangerouslyDisableSanitizers: ['script']
        }
    }
}
</script>