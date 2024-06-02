// html.js
class HtmlGenerator {
    static generateReleases() {
        let result = "";
        for (let key of Object.keys(AppContext.data)) {
            result += `<tr><td colspan="4"><h4>${key}</h4></td></td>`;
            let items = AppContext.data[key];
            for (let itm of items) {
                if (itm.released_version != itm.git_hub_version || !itm.released_version || !itm.git_hub_version) {
                    let button = "";
                    if (itm.git_hub_version) {
                        button = `<button data-id="${itm.id}" class="btn btn-primary btn-sm" onclick="AppContext.showUpdateDialog(this)">Update</button>`;
                    }
                    result += `<tr><td style="color:red">${itm.id}</td><td style="color:red">${itm.released_version}</td><td>${itm.git_hub_version}</td><td>${button}</td></tr>`;
                }
                else {
                    result += `<tr><td>${itm.id}</td><td>${itm.released_version}</td><td>${itm.git_hub_version}</td><td></td></tr>`;
                }
            }
        }
        return result;
    }
}

// dialog.js
class Dialog {
    static generateContent(appId) {
        let data = AppContext.getData(appId);
        return `<div class="dialog-window" onclick="event.stopImmediatePropagation()">
        <div class="modal-content">
      <div class="modal-header">
      <table style="width:100%">
      <td style="width:100%"><h5 class="modal-title">You are about to update version for application</h5></td>
      <td><button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close" onclick="AppContext.hideModal()"></button></td>
      </table>
        
      </div>
      <div class="modal-body">
      <hr/>
        <div id="selected-app-id" data-id="${appId}" ><b>${appId}</b></div>

        <div>GitHub Version: <b>${data.git_hub_version}</b></div>
        <div>Released Version: <b>${data.released_version}</b></div>
        <hr/>
        <div class="form-group">
            <label for="version">New Version</label>
            <input id="new-app-version" type="text" class="form-control" value="${data.git_hub_version}">
        </div>
        <hr/>

      </div>
      <div class="modal-footer">
      <div class="btn-group">
        <button type="button" class="btn btn-primary" data-bs-dismiss="modal" onclick="Dialog.updateVersion()">Update</button>
        <button type="button" class="btn btn-secondary" onclick="AppContext.hideModal()">Cancel</button>
        </div>
      </div>
    </div></div>`;
    }
    static updateVersion() {
        let appId = document.getElementById("selected-app-id").getAttribute("data-id");
        let versionElement = document.getElementById("new-app-version");
        let version = versionElement.value;
        let data = {
            id: appId,
            version
        };
        $.ajax({ url: "/api/Releases/SetToReleaseVersion", type: "POST", data }).then(function () {
            location.reload();
        });
        console.log(data);
    }
}

// app.js
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

