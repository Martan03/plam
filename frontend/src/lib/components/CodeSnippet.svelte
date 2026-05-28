<script lang="ts">
    import { bracketMatching } from "@codemirror/language";
    import { onMount } from "svelte";
    import { EditorView, basicSetup } from "codemirror";
    import { Compartment, EditorState } from "@codemirror/state";
    import { editorSyntax, editorTheme, snippetTheme } from "../highlighter";
    import { eval_lambda } from "../../../../engine/pkg/plam";
    import { vim } from "@replit/codemirror-vim";
    import { settings } from "../state/settings.svelte";
    import { persisted } from "../state/storage.svelte";

    type TestCase = {
        code: string;
        output: string;
    };

    let {
        id = "",
        hiddenCode = "",
        code = "",
        runnable = false,
        editable = false,
        tests = [] as TestCase[],
    } = $props();

    let output = $state("");

    let container: HTMLDivElement;
    let view: EditorView;
    const vimCompartment = new Compartment();

    let storage: any = null;

    onMount(() => {
        let initDoc = code;
        if (editable && id) {
            storage = persisted(`plam-snippet-${id}`, code);
            initDoc = storage.value;
        }

        const extensions = [
            basicSetup,
            editorTheme,
            snippetTheme,
            editorSyntax,
            bracketMatching(),
        ];

        if (!editable) {
            extensions.push(EditorState.readOnly.of(true));
            extensions.push(EditorView.editable.of(false));
        } else {
            extensions.push(vimCompartment.of(settings.vimMode ? vim() : []));
            extensions.push(
                EditorView.updateListener.of((update) => {
                    if (update.docChanged && storage) {
                        storage.value = update.state.doc.toString();
                    }
                }),
            );
        }

        view = new EditorView({
            doc: initDoc.trim(),
            extensions,
            parent: container,
        });

        return () => view.destroy();
    });

    $effect(() => {
        if (view) {
            view.dispatch({
                effects: vimCompartment.reconfigure(
                    settings.vimMode ? vim() : [],
                ),
            });
        }
    });

    function runEval(e?: MouseEvent) {
        e?.preventDefault();

        const curCode = view.state.doc.toString();
        const baseCode = `${hiddenCode}\n${curCode}`;

        try {
            if (tests.length === 0) {
                output = eval_lambda(baseCode, "").trim();
                return;
            }

            let fails = [];
            for (const test of tests) {
                const fullCode = `${baseCode}\n;${test.code};`;
                const res = eval_lambda(fullCode, "").trim();

                if (res !== test.output.trim()) {
                    fails.push(
                        `❌ ${test.code} (Got: '${res}', Expected: '${test.output}')`,
                    );
                }
            }

            if (fails.length === 0) {
                output = `✅ All ${tests.length} tests passed!`;
            } else {
                const passed = tests.length - fails.length;
                output =
                    `⚠️ ${passed}/${tests.length} passed. Fix the following:\n` +
                    fails.join("\n");
            }
        } catch (e) {
            output = `Error: ${e}`;
        }
    }

    function resetCode() {
        if (storage) storage.value = code;

        view.dispatch({
            changes: {
                from: 0,
                to: view.state.doc.length,
                insert: code.trim(),
            },
        });
        output = "";
    }
</script>

<div class="wrapper" class:is-editable={editable}>
    <div bind:this={container} class="cm-container"></div>

    {#if runnable}
        <div class="output-bar">
            <div class="output">{output}</div>

            <div class="actions">
                {#if editable && id}
                    <button class="btn secondary" onclick={resetCode}>
                        Reset
                    </button>
                {/if}

                <button class="btn secondary" onclick={runEval}>
                    {tests.length === 0 ? "▶ Run" : "▶ Run Test"}
                </button>
            </div>
        </div>
    {/if}
</div>

<style>
    .wrapper {
        margin: 1.5rem 0;
        border: 1px solid var(--border);
        border-radius: 8px;
        overflow: hidden;
        background-color: var(--bg-light);
    }

    .wrapper.is-editable {
        border-color: var(--primary);
    }

    .cm-container {
        padding: 0.5rem;
    }

    .output-bar {
        padding: 0.5rem;
        border-top: 1px solid var(--border);
        display: flex;
        align-items: flex-start;
        justify-content: space-between;
        background-color: var(--bg);
    }

    .output {
        white-space: pre-wrap;
    }

    .actions {
        display: flex;
        gap: 0.5rem;
    }

    :global(.cm-activeLine) {
        background-color: transparent !important;
    }

    :global(.wrapper .cm-gutters) {
        display: none;
    }

    .btn {
        padding: 0.3rem 0.5rem;
        background: transparent;
        color: var(--fg);
        border: 1px solid var(--border-light);
        border-radius: 4px;
        font-weight: bold;
        cursor: pointer;
        transition: background 0.1s;
    }

    .btn:hover {
        background: var(--bg-light);
        color: var(--fg-max);
    }
</style>
