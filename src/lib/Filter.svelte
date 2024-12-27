<script lang="ts">
    import Title from "./Title.svelte";
    import HugeButton from "./HugeButton.svelte";
    import FilterItem from "./FilterItem.svelte";
    import Empty from "./Empty.svelte";
    import { DateInput } from "date-picker-svelte";
    import { fly, scale } from "svelte/transition";

    let { visible = $bindable(), filter = $bindable(), categories } = $props();

    let filteredCategories: any = $state(
        categories
        ? categories.map((item: any) => item.name)
        : null
    );

    const getCategories = () => {
        
        // if (filteredCategories) {
        filteredCategories = filter.type === "All"
        ? categories.map((item: any) => item.name)
        : categories.filter((item: any) => item.type === filter.type).map((item: any) => item.name)
        // }

    }

    const reset = () => {
        filter = {
            type: 'All',
            method: 'All',
            category: 'All',
            date: {
                from: null,
                to: null
            }
        };
    }

    $effect(() => {

        filter.type, getCategories();

        if (filter.date.from) filter.date.from.setHours(0, 0, 0);
        if (filter.date.to) filter.date.to.setHours(23, 59, 59);
    })

</script>

<div class="w-full h-full bg-blue-900/95 top-0 left-0 fixed z-[999]" in:fly={{ y: 50, duration: 0 }}>
    <div class="fixed z-[999] inset-0 top-20 mx-auto
        p-5 border w-[90%] h-2/3 rounded-xl bg-white flex
        flex-col space-y-2 justify-between"
        in:fly|global={{ y: 50, duration: 100 }} out:fly|global={{ y: -50, duration: 100 }}
    >
        <div class="flex flex-row justify-between items-center scrollbar-hide pt-2">
            <span class="text-xl font-bold">Filter Transactions</span>
            <button class="text-md text-blue-900 font-bold" onclick={reset}>Reset</button>
        </div>
        <div class="flex flex-col justify-start items-start space-x-2 overflow-scroll">
            <h1 class="font-bold p-2">Transaction Type</h1>
            <div class="flex flex-row items-center space-x-2">
                <FilterItem value={"All"}
                    selected={filter.type === "All"}
                    onclick={() => filter.type = "All"}
                />
                <FilterItem value={"Income"}
                    selected={filter.type === "in"}
                    onclick={() => filter.type = "in"}
                />
                <FilterItem value={"Expense"}
                    selected={filter.type === "out"}
                    onclick={() => filter.type = "out"}
                />
            </div>
        </div>
        <div class="flex flex-col justify-start items-start space-x-2 overflow-scroll pb-2">
            <h1 class="font-bold p-2">Transaction Method</h1>
            <div class="flex flex-row items-center space-x-2">
                <FilterItem value={"All"}
                    selected={filter.method === "All"}
                    onclick={() => filter.method = "All"}
                />
                <FilterItem value={"Cash"}
                    selected={filter.method === "cash"}
                    onclick={() => filter.method = "cash"}
                />
                <FilterItem value={"Bank"}
                    selected={filter.method === "bank"}
                    onclick={() => filter.method = "bank"}
                />
            </div>
        </div>
        <h1 class="font-bold px-2">Transaction Category</h1>
        <div class="flex flex-col justify-start items-start space-x-2 overflow-scroll px-2">
            <div class="flex flex-row items-center space-x-2">
                {#if filteredCategories}
                    <FilterItem value={"All"}
                        selected={filter.category === "All"}
                        onclick={() => filter.category = "All"}
                    />
                    {#each filteredCategories as category}
                        <FilterItem value={category}
                            selected={filter.category === category}
                            onclick={() => filter.category = category}
                        />
                    {/each}
                {:else}
                    <Empty value="No category available." />
                {/if}
            </div>
        </div>
        <div class="flex flex-row justify-center items-center space-x-2">
            <div class="flex flex-col justify-center items-start">
                <span class="font-bold p-2">Date (From)</span>
                <DateInput
                    bind:value={filter.date.from}
                    dynamicPositioning={true}
                    placeholder="Select date"
                    closeOnSelection={true}
                    format="dd-MM-yyyy"
                />
            </div>
            <div class="flex flex-col justify-center items-start">
                <span class="font-bold p-2">Date (To)</span>
                <DateInput
                    bind:value={filter.date.to}
                    dynamicPositioning={true}
                    placeholder="Select date"
                    closeOnSelection={true}
                    format="dd-MM-yyyy"
                />
            </div>
        </div>
        <HugeButton value="Apply" onclick={() => visible = false} />
    </div>
</div>
