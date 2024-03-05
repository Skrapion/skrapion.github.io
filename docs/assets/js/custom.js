document.addEventListener("DOMContentLoaded", (event) => {
    document.addEventListener('htmx:beforeSwap', beforeSwap);
    document.addEventListener('htmx:afterSwap', afterSwap);
    document.addEventListener('htmx:afterSettle', afterSettle);
    afterSettle();
    initOneSignal();
});

htmx.on("htmx:responseError", function(evt) {
    var errdiv = document.getElementById("content");
    var parser = new DOMParser();
    var htmlDoc = parser.parseFromString(evt.detail.xhr.responseText, "text/html");

    errdiv.innerHTML = htmlDoc.getElementById("error").outerHTML; 
});

function beforeSwap() {
    var story = document.getElementById("story");
    if(story) {
        story.style="";
    }
}

function afterSwap() {
    window.scrollTo({top: 0, behavior: 'instant'});
}

function afterSettle() {
    postLoaded();
    registerDrag();
}

function postLoaded() {
    // Show/hide "read more" button depending on how big the text is
    var story = document.getElementById("story");

    if(story) {
        var ro = new ResizeObserver(entries => {
            var height = story.offsetHeight;
            var readmore = document.getElementById("readmore");
            if(readmore) {
                readmore.style.display =
                    (height < 150) ? "none" : "block";
            }
        });
        story.className = "storyjs";
        ro.observe(story);
    }
}

function readmore() {
    // max-height transitions don't work with 'auto' height, so we need to measure the end height.
    var story = document.getElementById("story");
    var height = Array.prototype.reduce.call(story.childNodes, function(p, c) {return p + (c.offsetHeight || 0);}, 0) + 'px';
    story.style.maxHeight = height;
    story.classList.add("openfull");

    window.setTimeout(function(){story.style.maxHeight = '100%';}, 1000); // After transition is done, reset max-height to 100%
}

var transitionName = "";

function setTransitionName(newTransitionName) {
    transitionName = newTransitionName;
    var contentDiv = document.getElementById("content");
    contentDiv.className = transitionName;

    return false;
}

function loaded(obj) {
    obj.classList.add('loaded');
}

window.closeLightboxOnHistory = function(event) {
    lightboxClose();
}

window.lightboxClose = function() {
    window.removeEventListener('popstate', closeLightboxOnHistory);
    
    var lightbox = document.getElementById("lightbox");
    if(lightbox) {
        lightbox.style.opacity = '0%';
    }

    window.setTimeout(function(){
        var lightbox = document.getElementById("lightbox");
        if(lightbox) {
            lightbox.parentNode.removeChild(lightbox);}
        }, 250);
}

window.lightboxOpen = function(pic) {
    var viewportOffset = pic.getBoundingClientRect();

    var lightbox = document.createElement("div");
    lightbox.id = "lightbox";
    lightbox.onclick = lightboxClose;
    
    lightbox.style.position = 'fixed';
    lightbox.style.left = lightbox.style.right = lightbox.style.top = lightbox.style.bottom = 0;
    lightbox.style.zIndex = 3;
    lightbox.style.background = '#000d';
    lightbox.style.opacity = '0%';
    lightbox.style.transition = 'opacity 250ms ease-out';

    var picContainer = document.createElement("div");
    picContainer.style.position = 'fixed';
    picContainer.style.left = viewportOffset.left+'px';
    picContainer.style.right = (document.documentElement.clientWidth - viewportOffset.right)+'px';
    picContainer.style.top = viewportOffset.top+'px';
    picContainer.style.bottom = (document.documentElement.clientHeight - viewportOffset.bottom)+'px';
    picContainer.style.display = 'flex';
    picContainer.style.alignItems = 'center';
    picContainer.style.transition = 'all 500ms';

    var picCopy = pic.cloneNode(true);
    picCopy.onclick = null;

    picCopy.style.width = '100%';
    picCopy.style.height = '100%';

    triggerAnim = function() {
        lightbox.style.opacity = '100%';

        picContainer.style.left = '10px';
        picContainer.style.top = '10px';
        picContainer.style.right = '10px';
        picContainer.style.bottom = '10px';
    }

    document.getElementById("bodycontainer").appendChild(lightbox).appendChild(picContainer).appendChild(picCopy);

    window.addEventListener('popstate', closeLightboxOnHistory);

    window.setTimeout(triggerAnim, 10);
}

function registerDrag() {
    // Touch events for next/prev post
    const dragParams = {
        yCancel: 50,
        xActivate: 100,
        friction: 4,
        dragMax: 25
    }

    const content = document.getElementById("singlecontent");
    if(content) {
        content.style.transform = null;
        var touchStart = null;

        content.addEventListener('touchstart', event => {
            if(touchStart == null) {
                var changedTouch = event.changedTouches[0];
                touchStart = {
                    identifier: changedTouch.identifier,
                    clientX: changedTouch.clientX,
                    clientY: changedTouch.clientY
                };
            }
        });
        content.addEventListener('touchmove', event => {
            if(touchStart != null) {
                for(const changedTouch of event.changedTouches) {
                    if(touchStart.identifier == changedTouch.identifier) {
                        var diffY = changedTouch.clientY - touchStart.clientY;
                        if(Math.abs(diffY) > dragParams.yCancel) {
                            // Cancel touch; scrolled too far.
                                content.style.transform = null;
                            touchStart = null;
                        } else {
                            var diffX = changedTouch.clientX - touchStart.clientX;
                            content.style.transform = "translate3d(" 
                                + Math.max(Math.min(diffX/dragParams.friction, dragParams.dragMax), dragParams.dragMax*-1) 
                                + "px, 0, 0)";
                        }
                        return;
                    }
                }
            }
        });
        content.addEventListener('touchcancel', event => {
            if(touchStart != null) {
                for(const changedTouch of event.changedTouches) {
                    if(touchStart.identifier == changedTouch.identifier) {
                        content.style.transform = null;
                        touchStart = null;
                        return;
                    }
                }
            }
        });
        content.addEventListener('touchend', event => {
            if(touchStart != null) {
                for(const changedTouch of event.changedTouches) {
                    if(touchStart.identifier == changedTouch.identifier) {
                        var diff = changedTouch.clientX - touchStart.clientX;
                        var clicked = false;
                        if(Math.abs(diff) > dragParams.xActivate) {
                            if(diff < 0) {
                                var olderlink = document.getElementById('olderlink');
                                if(olderlink) {
                                    olderlink.click();
                                    clicked = true;
                                }
                            } else if(diff > 0) {
                                var newerlink = document.getElementById('newerlink');
                                if(newerlink) {
                                    newerlink.click();
                                    clicked = true;
                                }
                            }
                        }

                        if(clicked == false) {
                            content.style.transform = null;
                        }
                        touchStart = null;
                        return;
                    }
                }
            }
        });
    }
}

function initOneSignal() {
    window.addEventListener("click", function(e) {
        var notificationBlock = document.getElementById("notification-block");
        var subscribe = document.getElementById("subscribe");
        if(notificationBlock.contains(e.target)) {
            subscribe.className = "show";
            setOneSignalSwitch();
        } else {
            subscribe.className = "";
        }
    });

    window.OneSignalDeferred = window.OneSignalDeferred || [];
    OneSignalDeferred.push(async function(OneSignal) {
        await OneSignal.init({
            appId: location.hostname == "localhost" ?
                "96ba05c2-8cf9-44d3-ab5e-6c5396ea8d85" :
                "3b78268c-b1c0-4037-b3f5-07cb3afb64fe",
            autoResubscribe: true,
        });
    });
}

function setOneSignalSwitch() {
    var switchtrack = document.getElementById("switchtrack");
    switchtrack.classList.remove("set");

    OneSignalDeferred.push(async function(OneSignal) {
        if(!OneSignal.Notifications.isPushSupported()) {
            var subscribeContent = document.getElementById("subscribe-content");
            subscribeContent.classList.add("unsupported");
        } else {
            Promise.all([
                OneSignal.Notifications.permission,
                OneSignal.User.PushSubscription.optedIn,
            ]).then((result) => {
                var isPushEnabled = result[0];
                var isOptedIn = result[1];

                if(isPushEnabled && isOptedIn) {
                    document.getElementById("switchtrack").classList.add("set");
                }
            });
        }
    });
}

function clickSubscribe() {
    OneSignalDeferred.push(async function(OneSignal) {
        Promise.all([
            OneSignal.Notifications.permission,
            OneSignal.User.PushSubscription.optedIn,
        ]).then(async (result) => {
            var isPushEnabled = result[0];
            var isOptedIn = result[1];

            if(!isPushEnabled) {
                if(await OneSignal.Notifications.requestPermission()) {
                    await OneSignal.User.PushSubscription.optIn();
                }
            } else {
                if(!isOptedIn) {
                    await OneSignal.User.PushSubscription.optIn();
                } else {
                    await OneSignal.User.PushSubscription.optOut();
                }
            }

            Promise.all([
                OneSignal.Notifications.permission,
                OneSignal.User.PushSubscription.optedIn,
            ]).then(async (result) => {
                var isPushEnabled = result[0];
                var isOptedIn = result[1];

                if(isPushEnabled && isOptedIn) {
                    switchtrack.classList.add("set");
                } else {
                    switchtrack.classList.remove("set");
                }
            });
        });
    });
}
