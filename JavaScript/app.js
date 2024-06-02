class AppContext {
    static showUpdateDialog(aThis) {
        let id = aThis.getAttribute("data-id");
        this.modalBackground.classList.remove('hidden');
        this.modalBackground.innerHTML = Dialog.generateContent(id);
    }
    static init() {
        if (this.initialized) {
            return;
        }
        this.initialized = true;
        this.modalBackground = document.getElementById("modal-background");
        this.modalBackground.addEventListener("click", function () {
            AppContext.hideModal();
        });
    }
    static hideModal() {
        this.modalBackground.innerHTML = "";
        this.modalBackground.classList.add('hidden');
    }
    static getData(id) {
        for (let key of Object.keys(AppContext.data)) {
            let items = AppContext.data[key];
            for (let itm of items) {
                if (itm.id == id) {
                    return itm;
                }
            }
        }
        return undefined;
    }
}
AppContext.initialized = false;
setTimeout(function () {
    AppContext.init();
    $.ajax({ url: "/api/Releases" }).then(function (data) {
        AppContext.data = data;
        let html = HtmlGenerator.generateReleases();
        document.getElementById("data-releases").innerHTML = html;
    });
}, 100);
//# sourceMappingURL=app.js.map