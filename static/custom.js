window.loaded = function(obj) {
    obj.classList.add('loaded');
}

window.lightboxClose = function() {
    var lightbox = document.getElementById("lightbox");
    lightbox.style.opacity = '0%';

    window.setTimeout(function(){lightbox.parentNode.removeChild(lightbox);}, 250);
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

    window.setTimeout(triggerAnim, 10);
}