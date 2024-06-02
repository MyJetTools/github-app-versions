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
//# sourceMappingURL=html.js.map