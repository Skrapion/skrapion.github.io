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
        if(process.env.NODE_ENV === 'production') return {
            script: [
                {
                    src: "https://cdn.onesignal.com/sdks/OneSignalSDK.js",
                    async: true
                },
                {
                    innerHTML: `
                        window.OneSignal = window.OneSignal || [];
                            OneSignal.push(function() {
                                OneSignal.init({
                                    appId: "3b78268c-b1c0-4037-b3f5-07cb3afb64fe",
                                });
                            });`
                }
            ],
            __dangerouslyDisableSanitizers: ['script']
        }
    }
}
</script>