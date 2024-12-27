<script lang="ts">
    let { value = $bindable('Value'), selected = $bindable(false), onclick = $bindable() } = $props();
</script>

<button
    class="min-w-12 px-4
        {selected
            ? "bg-blue-100 text-blue-900 border-blue-900"
            : "bg-white text-black border-blue-100"}
        rounded-2xl border-2 shadow-xl
        shadow-blue-100"

    onclick={onclick}
>
    {value}
</button>
