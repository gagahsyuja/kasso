<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import AddTransactionModal from "./lib/AddTransactionModal.svelte";
    import AddTransactionButton from "./lib/AddTransactionButton.svelte";
    import Database from "@tauri-apps/plugin-sql";
    import Navigation from "./lib/Navigation/Navigation.svelte";

    let name = $state("");
    let greetMsg = $state("");
    let object = $state({
        showAddTransactionModal: false,
        last: { amount: 0, description: 0 },
        totalIn: 0,
        totalOut: 0
    });

    async function greet(event: Event) {
        event.preventDefault();
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        greetMsg = await invoke("greet", { name });
    }

    const load = async () => {

        const db = await Database.load("sqlite:database.db");

        let last: Array<any> = await db.select("SELECT * FROM transactions ORDER BY id DESC LIMIT 1");
        let totalIn: Array<any> = await db.select("SELECT amount FROM transactions WHERE type = ?", ['in']);
        let totalOut: Array<any> = await db.select("SELECT amount FROM transactions WHERE type = ?", ['out']);

        object.totalIn = totalIn.reduce((total, num) => {
            return total + num.amount;
        }, 0) as number;

        object.totalOut = totalOut.reduce((total, num) => {
            return total + num.amount;
        }, 0) as number;

        object.last = last ? last[0] : {};
    }

    onMount(async () => {
        const db = await Database.load("sqlite:database.db");
        // await db.execute("INSERT INTO users (role, fullname, username, password) VALUES (?, ?, ?, ?)", ["bendahara", "Gagah Syuja", "gagahsyuja", "whatever"]);
        db.select("SELECT * FROM transactions").then(response => console.log(response));
        // db.execute("DELETE FROM users WHERE id = 2").then(response => console.log(response));
    });
</script>

<main class="m-0 p-4">
    {#await load() then}
        <div class="p-4">
            <h1 class="text-black-500 text-4xl font-semibold">
                Today
            </h1>
        </div>
        <div class="flex flex-col items-center text-3xl font-semibold bg-gray-300 rounded-2xl">
            <div class="flex flex-col items-center py-12">
                <h1 class="text-green-500 text-6xl py-4">
                    In
                </h1>
                <h1>
                    {object.totalIn}
                </h1>
            </div>
            <div class="flex flex-col items-center py-12">
                <h1 class="text-red-500 text-6xl py-4">
                    Out
                </h1>
                <h1>
                    {object.totalOut}
                </h1>
            </div>
        </div>
        <div>
            <h1>Last Transaction</h1>
            {object.last.amount}
            {object.last.description}
        </div>
        <AddTransactionButton { object } />
        {#if object.showAddTransactionModal}
            <AddTransactionModal { object } />
        {/if}
        <Navigation />
    {/await}
</main>

<style>
</style>
