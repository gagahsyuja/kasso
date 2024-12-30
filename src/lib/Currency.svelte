<script lang="ts">
    import accounting from "accounting";

    let { amount = 69000, bold = false, subUnit = true, size = "text-md" } = $props();
    let number: HTMLElement;
</script>

<span>
    <span class="text-lg">Rp</span><span bind:this={number} class="{bold ? "font-bold text-lg" : ""} {size}">{accounting.formatMoney(amount, "", subUnit ? 2 : 0, ".", ",")}</span>
</span>
