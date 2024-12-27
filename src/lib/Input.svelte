<script lang="ts">
    import Fa from "svelte-fa";
    import { faLock } from "@fortawesome/free-solid-svg-icons";

    let { name = "Password", type = "text", icon = faLock, value = $bindable() } = $props();
</script>

<div class="bg-white flex flex-col bg-red-500/0 px-6 w-full rounded-3xl shadow-xl shadow-blue-100">
    <label class="pt-2 text-gray-800" for="input">{name}</label>
    <div class="flex flex-row justify-center items-center pb-2">
        <Fa icon={icon} size="1.2x" />
        <input class="mx-2 border-0 w-full focus:ring-0 font-bold text-black text-xl" type={type} id="input" bind:value={value} />
    </div>
</div>
