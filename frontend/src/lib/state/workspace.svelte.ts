export function createWorkspace(initCode: string) {
    const storedFiles = localStorage.getItem("plam-files");
    const storedActive = localStorage.getItem("plam-active");

    let files = $state<Record<string, string>>(
        storedFiles ? JSON.parse(storedFiles) : { "stdlib.pl": initCode },
    );

    let active = $state<string>(storedActive || "stdlib.pl");
    if (!files[active]) {
        active = Object.keys(files)[0];
    }

    function save() {
        localStorage.setItem("plam-files", JSON.stringify(files));
        localStorage.setItem("plam-active", active);
    }

    return {
        /** Gets all the filenames in the workspace. */
        get files() {
            return Object.keys(files);
        },
        /** Gets the currently active workspace file. */
        get active() {
            return active;
        },

        /** Gets the currently displayed code. */
        get currentCode() {
            return files[active];
        },
        /** Sets the currently displayed code to the given value. */
        set currentCode(code: string) {
            files[active] = code;
            save();
        },

        /** Selects the given file. */
        select(filename: string) {
            active = filename;
            save();
        },

        /**
         * Adds a new file to workspace. Doesn't add when already exists.
         * @param filename - filename of the new file.
         * @param content - optional content of the new file.
         * @returns {boolean} - true when succesfully added, otherwise false.
         */
        add(filename: string, content: string = ""): boolean {
            if (files[filename]) return false;

            files[filename] = content;
            active = filename;
            save();
            return true;
        },

        /**
         * Removes the given file. Removes only when not the last file.
         * @param filename - filename of the file to be removed.
         * @returns {boolean} - true when removed succesfully, otherwise false.
         */
        remove(filename: string): boolean {
            if (Object.keys(files).length <= 1) {
                alert("Cannot delete the last file.");
                return false;
            }

            delete files[filename];
            if (active === filename) {
                active = Object.keys(files)[0];
            }
            save();
            return true;
        },
    };
}
