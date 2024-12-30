<script lang="ts">
    import Fa from "svelte-fa";
    import { fade } from "svelte/transition";
    let { target, currentPage = $bindable(), name, icon, title, onclick } = $props();
</script>

<a data-sveltekit-preload-data="tap" href={target}  onclick={onclick} class="flex flex-col items-center justify-center min-w-20 min-h-20 max-w-28 text-xl transition-transform active:scale-90 duration-100 space-y-1 { currentPage === name ? "text-blue-900" : "text-black" }">
    <Fa icon={icon} />
    <span class="text-sm">{title}</span>
</a>
