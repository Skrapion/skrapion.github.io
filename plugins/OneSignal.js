window.OneSignal = window.OneSignal || [];
OneSignal.push(function() {
    OneSignal.init({
        appId: process.env.NODE_ENV === 'production'
            ? "3b78268c-b1c0-4037-b3f5-07cb3afb64fe"
            : "767434af-9188-4281-9afe-2977206c5a9f",
        safari_web_id: process.env.NODE_ENV === 'production'
            ? "web.onesignal.auto.3182d724-6e8d-450b-a283-f7f35292ae01"
            : "web.onesignal.auto.1afe2633-50cf-455e-8f3e-a50d8cbe1d12",
        allowLocalhostAsSecureOrigin: true,
        autoResubscribe: true
    });
});