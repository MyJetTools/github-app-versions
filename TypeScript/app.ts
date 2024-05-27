

setTimeout(function () {
    $.ajax({ url: "/api/Releases" }).then(function (data) {
        let html = HtmlGenerator.generateReleases(data);
        document.getElementById("data-releases").innerHTML = html;
    });
}, 100);
