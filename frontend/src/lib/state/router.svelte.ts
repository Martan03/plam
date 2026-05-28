const base = import.meta.env.BASE_URL;

function getAppUrl(url: string) {
    if (url.startsWith(base)) {
        const stripped = url.slice(base.length);
        return stripped.startsWith("/") ? stripped : `/${stripped}`;
    }
    return url;
}

export const router = $state({
    path: getAppUrl(window.location.pathname),
});

export function navigate(path: string) {
    const target = path.startsWith("/") ? path : `/${path}`;
    const url = base.replace(/\/$/, "") + target;

    window.history.pushState({}, "", url);
    router.path = target;
}

window.addEventListener("popstate", () => {
    router.path = getAppUrl(window.location.pathname);
});
