<script lang="ts">
    import Fa from "svelte-fa";
    import { faChevronRight } from "@fortawesome/free-solid-svg-icons";
    import { scale, fade } from "svelte/transition";

    let { value = $bindable('Value'), icon = $bindable(), onclick = $bindable() } = $props();
</script>

<button
    onclick={onclick}
    class="flex flex-row justify-between items-center bg-white shadow-xl shadow-blue-100
        px-4 py-4 rounded-xl w-full text-left border-2 border-blue-100"
>
    <!-- <div class="flex flex-row justify-between items-center space-x-2"> -->
        <div class="flex flex-row justify-center items-center space-x-4 px-4">
            <Fa icon={icon} size="1x" />
            <span class="font-bold text-lg">{value}</span>
        </div>
        <div class="px-4">
            <Fa icon={faChevronRight} size="1x" />
        </div>
    <!-- </div> -->
</button>
