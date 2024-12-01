<script lang="ts">
    import Database from "@tauri-apps/plugin-sql";
    import { onMount } from "svelte";

    let { object } = $props();
    let selected = $state('in');

    let inCategory: Array<any> = $state([]);
    let outCategory: Array<any> = $state([]);

    let payload = $state({
        amount: 0,
        category: { id: 0, type: '', name: '' },
        description: '' 
    });

    const handleCategoryChange = (category: string) => {
        if (category === 'in') {
            inCategory ? payload.category = inCategory[0]: {};
            selected = 'in';
        } else {
            outCategory ? payload.category = outCategory[0]: {};
            selected = 'out';
        }
    }

    const handleSaveTransaction = async () => {

        const db = await Database.load("sqlite:database.db");

        console.log(payload);

        await db.execute(
            "INSERT INTO transactions\
            (user_id, category_id, description, type, amount, date)\
            VALUES (?, ?, ?, ?, ?, ?);",
            [
                1,
                payload.category.id,
                payload.description,
                selected,
                payload.amount,
                Date.now()
            ]
        );

        object.showAddTransactionModal = false;
    }

    const load = async () => {

        const db = await Database.load("sqlite:database.db");

        let categories: Array<Object> = await db.select("SELECT id, type, name FROM categories");

        inCategory = categories.filter((category: any) => category.type === 'in');
        outCategory = categories.filter((category: any) => category.type === 'out');

        payload.category = inCategory[0] ? inCategory[0] : {};
        console.log(payload.category);
    }

    onMount(async () => {
        
    })
</script>

{#await load() then}
    <div class="
        absolute w-[400px] h-[600px]
        flex flex-col items-center
        bg-gray-500 z-10
        top-0 left-0
        inset-x-0 mx-auto
        inset-y-44 my-auto
        rounded-2xl"
    >
        <h1 class="text-3xl font-bold p-6">Add Transaction</h1>
        <div class="flex flex-row text-3xl">
            <button class="bg-green-500 rounded-lg min-w-[120px]" onclick={() => handleCategoryChange('in')}>In</button>
            <button class="bg-red-500 rounded-lg min-w-[120px]" onclick={() => handleCategoryChange('out')}>Out</button>
        </div>
        <div class="p-8">
            {#if selected === 'in'}
                <div>
                    <span>Amount</span>
                    <input bind:value={payload.amount} />
                </div>

                <div>
                    <span>Category</span>
                    <select bind:value={payload.category}>
                    {#each inCategory as category}
                        <option value={category}>{category.name}</option>
                    {/each}
                    </select>
                </div>

                <div>
                    <span>Description</span>
                    <input bind:value={payload.description} />
                </div>
            {:else}
                <div>
                    <span>Amount</span>
                    <input bind:value={payload.amount} />
                </div>

                <div>
                    <span>Category</span>
                    <select bind:value={payload.category}>
                    {#each outCategory as category}
                        <option value={category}>{category.name}</option>
                    {/each}
                    </select>
                </div>

                <div>
                    <span>Description</span>
                    <input bind:value={payload.description} />
                </div>
            {/if}
        </div>
        <button class="bg-green-500 px-8 py-2 text-xl rounded-lg" onclick={handleSaveTransaction}>Save</button>
    </div>
{/await}
