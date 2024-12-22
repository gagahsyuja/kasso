<script lang="ts">
    import Fa from "svelte-fa";
    import { fade } from "svelte/transition";
    let props = $props();
</script>

<a data-sveltekit-preload-data="tap" href={props.target} class="flex flex-col items-center justify-center min-w-20 min-h-20 max-w-28 text-xl transition-transform active:scale-90 duration-100 space-y-1">
    <Fa icon={props.icon} />
    <span class="text-sm">{props.title}</span>
</a>
