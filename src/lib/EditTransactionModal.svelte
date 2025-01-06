<script lang="ts">
    import Database from "@tauri-apps/plugin-sql";
    import AddItem from "./AddItem.svelte";
    import Empty from "./Empty.svelte";
    import { DateInput } from "date-picker-svelte";
    import { onMount } from "svelte";
    import { scale, fly } from "svelte/transition";

    let { data, showEditModal = $bindable(), showDetailTransaction = $bindable(), refresh = $bindable() } = $props();

    let categories = $state([]);
    let filteredCategories: any = $state([]);

    let payload = $state({
        id: 0,
        type: 'in',
        amount: 0,
        category: 0,
        method: '',
        description: '',
        date: {} as Date
    });

    const handleTypeChange = (selected: string) => {

        payload.type = selected;

        if (payload.type === 'out') {
            payload.category = 2;
        } else {
            payload.category = 1;
        }

        filteredCategories = categories
            ? categories.filter((cat: any) => cat.type === payload.type)
            : [];
    }

    const handleEditTransaction = async () => {

        const db = await Database.load("sqlite:database.db");

        await db.execute(
            "UPDATE transactions SET\
                category_id = $2, description = $3, type = $4,\
                amount = $5, method = $6, date = $7\
            WHERE id = $1",
            [
                payload.id,
                payload.category,
                payload.description,
                payload.type,
                payload.amount,
                payload.method,
                Date.parse(payload.date.toString())
            ]
        );

        let notify = parseInt(localStorage.getItem('notify')!);
        let username = localStorage.getItem('username');

        if (payload.amount >= notify) {
            await db.execute(
                "UPDATE users SET new_notification = 1 WHERE username = $1",
                [ username ]
            )
        }

        showDetailTransaction = false;
        showEditModal = false;
        refresh = !refresh;
    }

    const load = async () => {

        const db = await Database.load("sqlite:database.db");

        categories = await db.select("SELECT id, type, name FROM categories");
    }

    $effect(() => {
        handleTypeChange(payload.type);
        console.log(payload.method)
    })

    onMount(async () => {

        payload.id = data.id;
        payload.amount = data.amount;
        payload.type = data.type;
        payload.date = new Date(data.date)
        payload.method = data.method;
        payload.category = data.categoryId;
        payload.description = data.description;

        console.log(data)

        await load();
    })
</script>

{#await load() then}
<div class="w-full h-full bg-white/100 top-0 left-0 fixed z-[999]" in:scale={{ duration: 50 }}>
    <div class="fixed z-[999] inset-0 top-0 mx-auto
        p-5 w-[90%] h-6/7 rounded-xl bg-white flex
        flex-col space-y-2 justify-between"
        in:fly|global={{ y: 50, duration: 100 }} out:fly|global={{ y: -50, duration: 100 }}
    >
        <div class="flex flex-row justify-between items-center scrollbar-hide pt-2">
            <span class="text-xl font-bold">Edit Transactions #{data.id}</span>
            <button class="text-md text-blue-900 font-bold" onclick={() => {showDetailTransaction = false; showEditModal = false; refresh = true}}>Cancel</button>
        </div>
        <div class="flex flex-col justify-center items-start">
            <span class="font-bold p-2">Date</span>
            <DateInput
                bind:value={payload.date}
                dynamicPositioning={true}
                placeholder="Select date"
                closeOnSelection={true}
                format="dd-MM-yyyy"
            />
        </div>
        <div class="flex flex-col justify-center items-start space-x-2">
            <span class="text-lg font-bold p-2">Transaction Type</span>
            <div class="flex flex-row items-center space-x-2">
                <AddItem value="Income" selected={payload.type === 'in'}
                    onclick={() => handleTypeChange('in')}
                />
                <AddItem value="Expense" selected={payload.type === 'out'}
                    onclick={() => handleTypeChange('out')}
                />
            </div>
        </div>
        <span class="text-lg font-bold p-2">Category</span>
        <div class="flex flex-col justify-center items-start space-x-2 overflow-scroll">
            <div class="flex flex-row items-center space-x-2">
                {#each filteredCategories as category}
                    <AddItem value={category.name} selected={payload.category === category.id}
                        onclick={() => payload.category = category.id}
                    />
                {/each}
            </div>
        </div>
        <div class="flex flex-row justify-start items-start">
            <div class="flex flex-col justify-center items-start space-x-2">
                <span class="text-lg font-bold p-2">Amount</span>
                <div class="flex flex-row items-center space-x-2">
                    <input
                        type="tel"
                        class="w-5/6 border-2 border-blue-100 shadow-xl shadow-blue-100 ring-0 rounded-xl"
                        bind:value={payload.amount}
                    />
                </div>
            </div>
            <div class="flex flex-col justify-center items-start">
                <span class="text-lg font-bold p-2">Method</span>
                <div class="flex flex-row items-center space-x-2">
                    <AddItem value="Cash" selected={payload.method === 'cash'}
                        onclick={() => payload.method = 'cash'}
                    />
                    <AddItem value="Bank" selected={payload.method === 'bank'}
                        onclick={() => payload.method = 'bank'}
                    />
                </div>
            </div>
        </div>
        <div class="flex flex-col justify-center items-start">
            <span class="text-lg font-bold p-2">Description</span>
            <div class="flex flex-row items-center w-full p-2">
                <textarea
                    class="resize-none w-full h-full border-2 border-blue-100 rounded-xl shadow-xl shadow-blue-100 ring-0 bg-white"
                    bind:value={payload.description}
                >
                </textarea>
            </div>
        </div>
        <button class="bg-blue-900 text-white font-bold px-8 py-2 text-lg rounded-lg" onclick={handleEditTransaction}>Save</button>
    </div>
</div>
{/await}
