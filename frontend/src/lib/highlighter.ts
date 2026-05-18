/**
 * Quick and dirty solution for the syntax highlighting.
 *
 * This will be replaced by more robust solution later using the actual AST
 * that the interpreter builds (maybe, not guaranteed :) ).
 */

import { Decoration, MatchDecorator, ViewPlugin } from "@codemirror/view";
import { EditorView } from "codemirror";

const editorDecor = new MatchDecorator({
    regexp: /\b(let)\b|([\\=\.])|(\/\/.*)/g,
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

const editorTheme = EditorView.baseTheme({
    ".keyword": { color: "#4C8DFF", fontWeight: "italic" },
    ".lambda": { color: "#e06c75", fontWeight: "bold" },
    ".operator": { color: "#E5DBA0" },
    ".comment": { color: "#666666", fontWeight: "italic" },
});

export const editorSyntax = [editorPlugin, editorTheme];
