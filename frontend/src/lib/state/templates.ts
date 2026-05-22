const rawTemplates = import.meta.glob("../../../../pl-examples/*", {
    eager: true,
    query: "?raw",
    import: "default",
});

export interface Template {
    name: string;
    content: string;
}

export const templates: Template[] = Object.entries(rawTemplates).map(
    ([path, content]) => {
        const name = path.split("/").pop() || "Unknown";
        return {
            name,
            content: content as string,
        };
    },
);
