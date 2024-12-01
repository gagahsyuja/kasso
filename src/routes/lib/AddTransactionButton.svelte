<script lang="ts">
    import Fa from "svelte-fa";
    import { faPlus, faXmark } from "@fortawesome/free-solid-svg-icons";

    let { object } = $props();
</script>

{#if !object.showAddTransactionModal}
    <button
        class="fixed flex items-center justify-center
        bg-green-500 w-20 h-20 bottom-20
        rounded-full my-10 inset-x-0 mx-auto"

        onclick={() => object.showAddTransactionModal = !object.showAddTransactionModal}
    >
        <Fa icon={faPlus} class="text-6xl text-white" />
    </button>
{:else}
    <button
        class="fixed flex items-center justify-center
        bg-red-500 w-20 h-20 bottom-20
        rounded-full my-10 inset-x-0 mx-auto"

        onclick={() => object.showAddTransactionModal = !object.showAddTransactionModal}
    >
        <Fa icon={faXmark} class="text-6xl text-white" />
    </button>
{/if}
