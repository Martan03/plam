const SETTINGS_KEY = "plam-setting";

export const THEMES = ["slate", "graphite"] as const;

const defaultSettings = {
    vimMode: false,
    primaryColor: "#3acbaf",
    theme: "slate",
};

type Theme = (typeof THEMES)[number];
type Settings = {
    vimMode: boolean;
    primaryColor: string;
    theme: Theme;
};

function createSettings() {
    const stored = localStorage.getItem(SETTINGS_KEY);

    const state = $state<Settings>(
        stored
            ? { ...defaultSettings, ...JSON.parse(stored) }
            : defaultSettings,
    );

    function save() {
        localStorage.setItem(SETTINGS_KEY, JSON.stringify(state));
    }

    return {
        get vimMode(): boolean {
            return state.vimMode;
        },
        set vimMode(value: boolean) {
            state.vimMode = value;
            save();
        },

        get primaryColor(): string {
            return state.primaryColor;
        },
        set primaryColor(value: string) {
            state.primaryColor = value;
            save();
        },

        get theme(): Theme {
            return state.theme;
        },
        set theme(value: Theme) {
            state.theme = value;
            save();
        },
    };
}

export const settings = createSettings();
