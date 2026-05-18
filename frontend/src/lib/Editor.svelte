<script lang="ts">
    import { oneDark } from "@codemirror/theme-one-dark";
    import { basicSetup, EditorView } from "codemirror";
    import { editorSyntax } from "./highlighter";

    let { code = $bindable() } = $props();

    function createEditor(node: HTMLElement) {
        const view = new EditorView({
            doc: code,
            extensions: [
                basicSetup,
                oneDark,
                editorSyntax,
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
</script>

<div class="editor" use:createEditor></div>

<style>
    .editor {
        overflow: hidden;
    }

    :global(.cm-editor) {
        height: 100%;
    }

    :global(.cm-scroller) {
        overflow: auto;
    }
</style>
