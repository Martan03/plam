<script lang="ts">
    import { bracketMatching } from "@codemirror/language";
    import { basicSetup, EditorView } from "codemirror";
    import { editorSyntax, editorTheme } from "../highlighter";
    import { Compartment } from "@codemirror/state";
    import { settings } from "../state/settings.svelte";
    import { vim } from "@replit/codemirror-vim";

    let { code = $bindable() } = $props();

    let view: EditorView;
    const vimCompartment = new Compartment();

    function createEditor(node: HTMLElement) {
        view = new EditorView({
            doc: code,
            extensions: [
                basicSetup,
                editorTheme,
                editorSyntax,
                bracketMatching(),
                vimCompartment.of(settings.vimMode ? vim() : []),
                EditorView.updateListener.of((update) => {
                    if (update.docChanged) {
                        code = update.state.doc.toString();
                    }
                }),
            ],
            parent: node,
        });

        return {
            destroy() {
                view.destroy();
            },
        };
    }

    $effect(() => {
        if (view && code !== view.state.doc.toString()) {
            view.dispatch({
                changes: { from: 0, to: view.state.doc.length, insert: code },
            });
        }
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
</script>

<div class="editor" use:createEditor></div>

<style>
    .editor {
        overflow: hidden;
        flex: 1;
        min-height: 0;
    }

    :global(.cm-editor) {
        height: 100%;
    }

    :global(.cm-scroller) {
        overflow: auto;
    }
</style>
