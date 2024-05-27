class HtmlGenerator {
    static generateReleases(releases) {
        let result = "";
        for (let key of Object.keys(releases)) {
            result += `<tr><td colspan="3"><h4>${key}</h4></td></td>`;
            let items = releases[key];
            for (let itm of items) {
                if (itm.released_version != itm.git_hub_version || !itm.released_version || !itm.git_hub_version) {
                    result += `<tr><td style="color:red">${itm.id}</td><td style="color:red">${itm.released_version}</td><td>${itm.git_hub_version}</td></tr>`;
                }
                else {
                    result += `<tr><td>${itm.id}</td><td>${itm.released_version}</td><td>${itm.git_hub_version}</td></tr>`;
                }
            }
        }
        return result;
    }
}
//# sourceMappingURL=html.js.map