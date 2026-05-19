/**
 * Creates persisted item
 * @param key - unique key identifier
 * @param fallback - fallback value when no value is stored
 * @returns persisted item
 */
export function persisted<T>(key: string, fallback: T) {
    const stored = localStorage.getItem(key);
    let state = $state<T>(stored !== null ? JSON.parse(stored) : fallback);

    return {
        get value(): T {
            return state;
        },
        set value(value: T) {
            state = value;
            localStorage.setItem(key, JSON.stringify(value));
        },
    };
}
