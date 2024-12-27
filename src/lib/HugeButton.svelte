<script lang="ts">
    let { value = $bindable('Button'), onclick } = $props();
</script>

<div class="w-full px-8 py-5">
    <button onclick={onclick} class="bg-blue-900 text-white font-semibold p-4 w-full rounded-[30px]">{value}</button>
</div>
