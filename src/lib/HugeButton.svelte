<script lang="ts">
    let { value = $bindable('Button'), onclick, bgColor = 'bg-blue-900', color = 'text-white' } = $props();
</script>

<div class="w-full px-8 py-5">
    <button onclick={onclick} class="{bgColor} {color} font-semibold py-4 w-full rounded-[30px]">{value}</button>
</div>
