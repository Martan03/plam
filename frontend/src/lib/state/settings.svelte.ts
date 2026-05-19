const SETTINGS_KEY = "plam-setting";

const defaultSettings = {
    vimMode: false,
    primaryColor: "#3acbaf",
};

type Settings = typeof defaultSettings;

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
        get vimMode() {
            return state.vimMode;
        },
        set vimMode(value: boolean) {
            state.vimMode = value;
            save();
        },

        get primaryColor() {
            return state.primaryColor;
        },
        set primaryColor(value: string) {
            state.primaryColor = value;
            save();
        },
    };
}

export const settings = createSettings();
