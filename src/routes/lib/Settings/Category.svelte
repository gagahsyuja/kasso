<script lang="ts">
    import HugeButton from "../HugeButton.svelte";
    import FilterItem from "../FilterItem.svelte";
    import Database from "@tauri-apps/plugin-sql";
    import { fly, scale } from "svelte/transition";

    let { showCategory = $bindable() } = $props();

    let type = $state('in');
    let name = $state('');

    const handleAddCategory = async () => {
        
        const db = await Database.load("sqlite:database.db");

        await db.execute(
            "INSERT INTO categories\
            (type, name)\
            VALUES ($1, $2);",
            [type, name]
        );

        showCategory = false;
    };

</script>

<div class="w-full h-full bg-blue-900/95 top-0 left-0 fixed z-[999]" in:fly={{ y: 50, duration: 0 }}>
    <div class="fixed z-[999] inset-0 top-20 mx-auto
        p-5 border w-[90%] h-1/2 rounded-xl bg-white flex
        flex-col space-y-2 justify-between"
        in:fly|global={{ y: 50, duration: 100 }} out:fly|global={{ y: -50, duration: 100 }}
    >
        <div class="flex flex-row justify-between items-center scrollbar-hide pt-2">
            <span class="text-xl font-bold">Add Category</span>
            <button class="text-md text-blue-900 font-bold" onclick={() => showCategory = false}>Cancel</button>
        </div>
        <div class="flex flex-col justify-start items-start space-x-2 overflow-scroll">
            <h1 class="font-bold p-2">Transaction Type</h1>
            <div class="flex flex-row items-center space-x-2">
                <FilterItem value={"Income"}
                    selected={type === "in"}
                    onclick={() => type = "in"}
                />
                <FilterItem value={"Expense"}
                    selected={type === "out"}
                    onclick={() => type = "out"}
                />
            </div>
        </div>
        <div class="flex flex-col justify-center items-start">
            <span class="font-bold p-2">Name</span>
            <div class="flex flex-row items-center w-full px-2">
                <input
                    type="text"
                    class="w-full border-2 border-blue-100 shadow-xl shadow-blue-100 focus:ring-0 rounded-xl"
                    bind:value={name}
                />
            </div>
        </div>
        <HugeButton value="Add" onclick={handleAddCategory} />
    </div>
</div>
