<script lang="ts">
    import { fly } from "svelte/transition";
    let { title = "The Title" } = $props();
</script>

<div in:fly|global={{ x: -50, delay: 50 }} class="px-4 py-2">
    <h1 class="text-black-500 text-2xl font-semibold">
        {title}
    </h1>
</div>
