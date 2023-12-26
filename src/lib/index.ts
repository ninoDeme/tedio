// place files you want to import through the `$lib` alias in this folder.

export type TRequest = {
    id: string,
    name: string,
    description?: string,
    url: string,
    method: string,
    headers: THeader[],
    body?: string,
}

export type THeader = {
    key: string,
    value: string,
}