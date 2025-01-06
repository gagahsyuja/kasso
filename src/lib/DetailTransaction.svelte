<script lang="ts">
    import Database from "@tauri-apps/plugin-sql";
    import Currency from "./Currency.svelte";
    import HugeButton from '$lib/HugeButton.svelte';
    import EditTransactionModal from "./EditTransactionModal.svelte";
    import { scale, fade } from "svelte/transition";
    import { fly } from 'svelte/transition';
    import Fa from "svelte-fa";
    import { faTrash } from "@fortawesome/free-solid-svg-icons";
    import { getDate, getDay, getMonth, getYear, getHours, getMinutes } from 'date-fns';

    let showEditModal = $state(false);

    let days = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let { showPopup = $bindable(false), showDetailTransaction = $bindable(false), refresh = $bindable(false), data } = $props();

    let category = $state('');

    const removeTransaction = async (id: number) => {

        const db = await Database.load("sqlite:database.db");

        await db.execute("DELETE FROM transactions WHERE id=?", [id]);
    }

    $effect(() => {
        Database.load("sqlite:database.db").then((db: Database) => {
            db.select(
                "SELECT name FROM categories WHERE id=?",
                [data.categoryId]
            ).then((categories: any) => {
                category = categories[0].name
            });
        });
    })
</script>

{#if !showEditModal}
    {#key showEditModal}
        <div class="w-full h-full bg-blue-900/95 top-0 left-0 fixed z-[999]" in:scale={{ duration: 50 }}>
            <div class="fixed z-[999] inset-0 top-14 mx-auto
                p-5 border w-[90%] h-5/6 rounded-xl bg-white flex
                flex-col space-y-2 justify-between"
                in:fly|global={{ y: 50, duration: 100 }} out:fly|global={{ y: -50, duration: 100 }}
            >
                <div class="flex flex-row justify-between items-center scrollbar-hide pt-2">
                    <span class="text-xl font-bold">Detail Transaction #{data.id}</span>
                    <button class="text-md text-blue-900 font-bold" onclick={() => {showDetailTransaction = false; refresh = !refresh}}>Close</button>
                </div>
                <div class="flex flex-col space-y-4">
                    <div class="flex flex-col p-2">
                        <span class="text-lg font-bold">Date</span>
                        <span class="">
                            {days[getDay(data.date)]}, {getDate(data.date)} {months[getMonth(data.date)]} {getYear(data.date)} at {getHours(data.date)}:{getMinutes(data.date)}
                        </span>
                    </div>
                    <div class="flex flex-row justify-between items-center">
                        <div class="flex flex-col p-2">
                            <span class="text-lg font-bold">Type</span>
                            <span class="text-md">
                                {data.type === 'in' ? "Income" : "Expense"}
                            </span>
                        </div>
                        <div class="flex flex-col p-2">
                            <span class="text-lg font-bold">Amount</span>
                            <Currency amount={data.amount} size="text-md" />
                        </div>
                    </div>
                    <div class="flex flex-row justify-between items-center">
                        <div class="flex flex-col p-2">
                            <span class="text-lg font-bold">Method</span>
                            <span class="text-md capitalize">
                                {data.method}
                            </span>
                        </div>
                        <div class="flex flex-col p-2">
                            <span class="text-lg font-bold">Category</span>
                            <span class="">
                                {category}
                            </span>
                        </div>
                    </div>
                    <div class="flex flex-col p-2">
                        <span class="text-lg font-bold">Description</span>
                        <div class="min-h-12">
                            {data.description ? data.description : '-'}
                        </div>
                    </div>
                    <div class="flex flex-row justify-between items-center">
                        <HugeButton onclick={() => showEditModal = true} value="Edit" />
                        <button
                            onclick={() => {
                                showPopup = !showPopup;
                            }}
                            class="px-8 py-4 bg-red-500 mr-8 text-white rounded-full"
                        >
                            <Fa icon={faTrash} size="1.2x" />
                        </button>
                    </div>
                </div>
            </div>  
        </div>
    {/key}
    {#if showPopup}
        <!-- <Popup  /> -->
        <div class="w-full h-full bg-blue-900/95 top-0 left-0 fixed z-[999]" in:fly={{ y: 50, duration: 0 }}>
            <div class="fixed z-[999] inset-0 top-32 mx-auto
                p-5 border w-[70%] h-1/4 rounded-xl bg-white flex
                flex-col space-y-2 justify-between"
                in:fly|global={{ y: 50, duration: 100 }} out:fly|global={{ y: -50, duration: 100 }}
            >
                <div class="flex flex-col justify-between items-center scrollbar-hide pt-2 space-y-4">
                    <span class="text-xl font-bold">Transaction Removed</span>
                    <button class="px-8 py-4 bg-blue-900 rounded-3xl text-md text-white font-bold" onclick={ () => { removeTransaction(data.id); showDetailTransaction = false; showPopup = false; refresh = !refresh }}>Close</button>
                </div>
            </div>
        </div>
    {/if}
{:else}
    <EditTransactionModal bind:showDetailTransaction bind:showEditModal bind:refresh { data } />
{/if}
