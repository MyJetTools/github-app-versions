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
//# sourceMappingURL=dialog.js.map