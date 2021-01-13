<template>
    <div id='error'>
        <h1>{{error.statusCode}}</h1>
        <h2>{{error.message.toLowerCase()}}</h2>
        <p>Well this is embarassing.</p>
    </div>
</template>

<script>
export default {
    props: ['error'],
    layout: 'error',
    head() {
        return {
            title: "Error " + this.error.statusCode + " - Firefang",
            link: [
                {rel: 'preconnect', href: 'https://fonts.gstatic.com'},
                {rel: 'stylesheet', href: 'https://fonts.googleapis.com/css2?family=Rouge+Script&display=swap'},
            ]
        }
    }
}
</script>

<style scoped>
#error {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 100vh;
}

h1 {
    font-size: 12rem;
    font-family: 'Rouge Script', cursive;
    font-weight: 100;
    font-style: italic;
    line-height: 0.5em;
    margin: 0;
}

h2 {
    font-size: 4rem;
    font-family: 'Rouge Script', cursive;
    font-weight: 100;
    font-style: italic;
    margin-top: 0;
}
</style>