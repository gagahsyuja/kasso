<script lang="ts">
    import Navigation from "../lib/Navigation/Navigation.svelte";
    import ListTransaction from "../lib/ListTransaction.svelte";
    import Database from "@tauri-apps/plugin-sql";
    import Title from "../lib/Title.svelte";
    import Main from "../lib/Main.svelte";
    import Filter from "../lib/Filter.svelte";
    import Empty from "../lib/Empty.svelte";
    import { fly } from "svelte/transition";
    import { flip } from "svelte/animate";
    import { onMount } from "svelte";

    let visible = $state(false);

    let filter = $state({
        type: 'All',
        method: 'All',
        category: 'All',
        date: {
            from: null,
            to: null
        }
    });

    const getCategories = async (): Promise<Array<any>> => {

        const db = await Database.load("sqlite:database.db");

        return await db.select("SELECT * FROM categories");
    }

    const loadWithFilter = async (type: String, method: String, category: String,
        fromDate: Date | null, toDate: Date | null): Promise<Array<any>> => {

        const db = await Database.load("sqlite:database.db");

        let query = 'SELECT * FROM transactions LEFT JOIN categories ON transactions.category_id = categories.id WHERE transactions.id IS NOT NULL';

        if (type !== "All") query = `${query} AND transactions.type = "${type}"`;
        if (method !== "All") query = `${query} AND transactions.method = "${method}"`;
        if (category !== "All") query = `${query} AND categories.name = "${category}"`;

        if (fromDate) {
            let from = Date.parse(fromDate.toString());

            let to;

            if (toDate) {
                to = Date.parse(toDate.toString());
            } else {
                filter.date.to = new Date();
                to = Date.parse(new Date().toString());
            }

            query = `${query} AND transactions.date BETWEEN ${from} AND ${to}`;
        }

        query = `${query} ORDER BY transactions.date`;

        return await db.select(query);
    };

</script>

<Navigation />

<Main>
    <div class="flex flex-row justify-between items-center">
        <Title title="History" />
        <button
            onclick={() => visible = true}
            class="text-md font-bold text-blue-900 px-4"
            in:fly={{ x: 50, y: -50 }}
        >
            Filter
        </button>
    </div>
    {#await getCategories() then categories}
        {#if visible}
            <Filter bind:visible bind:filter { categories } />
        {/if}
    {/await}
    {#await loadWithFilter(filter.type, filter.method, filter.category, filter.date.from, filter.date.to) then transactions}
        <div class="overflow-scroll">
            {#each transactions.reverse() as transaction}
                <div in:fly|global={{ y: 50, delay: 0 }} class="py-2">
                    <ListTransaction
                        amount={transaction.amount}
                        type={transaction.type}
                        date={transaction.date}
                        description={transaction.description}
                        categoryId={transaction.category_id}
                    />
                </div>
            {:else}
                <div in:fly|global={{ y: 50, delay: 0 }} class="fixed top-0 w-full h-full flex justify-center items-center pr-4">
                    <Empty value="No transaction has been made." />
                </div>
            {/each}
        </div>
    {/await}
</Main>
