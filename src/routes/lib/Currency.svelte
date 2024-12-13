<script lang="ts">
    let { amount = 69000, bold = false } = $props();
    import accounting from "accounting";
</script>

<span>
    Rp<span class="{bold ? "font-bold" : ""}">{accounting.formatMoney(amount, "", 2, ".", ",")}</span>
</span>
