window.OneSignal = window.OneSignal || [];
OneSignal.push(function() {
    OneSignal.init({
        appId: process.env.NODE_ENV === 'production'
            ? "3b78268c-b1c0-4037-b3f5-07cb3afb64fe"
            : "767434af-9188-4281-9afe-2977206c5a9f",
        allowLocalhostAsSecureOrigin: true,
        autoResubscribe: true
    });
});