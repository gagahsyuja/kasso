<script lang="ts">
    import accounting from "accounting";

    let { amount = 69000, bold = false, subUnit = true } = $props();
    let number: HTMLElement;
</script>

<span>
    Rp<span bind:this={number} class="{bold ? "font-bold" : ""}">{accounting.formatMoney(amount, "", subUnit ? 2 : 0, ".", ",")}</span>
</span>
