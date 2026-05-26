/**
 * Quick and dirty solution for the syntax highlighting.
 *
 * This will be replaced by more robust solution later using the actual AST
 * that the interpreter builds (maybe, not guaranteed :) ).
 */

import { Decoration, MatchDecorator, ViewPlugin } from "@codemirror/view";
import { EditorView } from "codemirror";

const editorDecor = new MatchDecorator({
    regexp: /\b(let)\b|([\\=\.])|(\/\/ .*)/g,
    decoration: (match) => {
        if (match[1] === "let") return Decoration.mark({ class: "keyword" });

        if (match[2] === "\\") return Decoration.mark({ class: "lambda" });
        if (match[2] === "=" || match[2] === ".")
            return Decoration.mark({ class: "operator" });

        if (match[3]) return Decoration.mark({ class: "comment" });

        return null;
    },
});

const editorPlugin = ViewPlugin.fromClass(
    class {
        decorations;
        constructor(view: any) {
            this.decorations = editorDecor.createDeco(view);
        }
        update(update: any) {
            this.decorations = editorDecor.updateDeco(update, this.decorations);
        }
    },
    {
        decorations: (v) => v.decorations,
    },
);

const syntaxTheme = EditorView.baseTheme({
    ".keyword": { color: "#4C8DFF", fontWeight: "italic" },
    ".lambda": { color: "#ED478F", fontWeight: "bold" },
    ".operator": { color: "#E5DBA0" },
    ".comment": { color: "#666666", fontWeight: "italic" },
});

export const editorSyntax = [editorPlugin, syntaxTheme];

export const editorTheme = EditorView.theme(
    {
        "&": {
            backgroundColor: "var(--bg)",
            color: "var(--fg)",
        },
        ".cm-gutters": {
            backgroundColor: "var(--bg-panel)",
            color: "var(--fg-disabled)",
            borderRight: "1px solid var(--border)",
        },
        ".cm-activeLine": {
            backgroundColor: "var(--bg-light)",
        },
        ".cm-activeLineGutter": {
            backgroundColor: "var(--bg-light)",
            color: "var(--fg-max)",
        },
        "&.cm-focused .cm-cursor": {
            borderLeftColor: "var(--primary)",
        },
        "&.cm-focused .cm-selectionBackground, ::selection": {
            backgroundColor: "var(--border-light)",
        },
        "&.cm-focused .cm-fat-cursor": {
            backgroundColor: "var(--primary) !important",
            border: "none !important",
            color: "var(--bg) !important",
        },
        "&:not(.cm-focused) .cm-fat-cursor": {
            backgroundColor: "transparent !important",
            outline: "1px solid var(--primary) !important",
            color: "var(--fg) !important",
        },
    },
    { dark: true },
);

export const snippetTheme = EditorView.theme({
    "&": {
        backgroundColor: "var(--bg-light) !important",
    },
    ".cm-activeLine": {
        backgroundColor: "transparent !important",
    },
});
