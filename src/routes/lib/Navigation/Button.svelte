<script lang="ts">
    import Fa from "svelte-fa";
    let props = $props();
</script>

<div class="flex flex-col items-center justify-center min-w-24 max-w-28 text-xl">
    <Fa icon={props.icon} />
    <span>{props.title}</span>
</div>
