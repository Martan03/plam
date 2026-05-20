<script lang="ts">
    import type { Snippet } from "svelte";
    import type { SVGAttributes } from "svelte/elements";

    interface Props extends SVGAttributes<SVGElement> {
        children: Snippet;
    }
    let { children, fill = "none", ...rest }: Props = $props();
</script>

<svg xmlns="http://www.w3.org/2000/svg" {fill} {...rest}>
    {@render children()}
</svg>
