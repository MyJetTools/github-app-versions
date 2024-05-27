declare const $;



interface IdGroupHttpModel {
    category: string;
    ids: string[];
}


interface ReleaseHttpModel {
    id: string,
    released_version: string,
    git_hub_version: string,
}