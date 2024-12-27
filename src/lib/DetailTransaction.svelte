<script lang="ts">
    import Currency from "./Currency.svelte";
    import { scale, fade } from "svelte/transition";
    import { fly } from 'svelte/transition';
    import { getDate, getDay, getMonth, getYear, getHours, getMinutes } from 'date-fns';

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

    let { data } = $props();
</script>

<div class="w-full h-full bg-blue-900/95 top-0 left-0 fixed z-[999]" in:scale={{ duration: 50 }}>
    <div class="fixed z-[999] inset-0 top-20 mx-auto
        p-5 border w-[90%] h-2/3 rounded-xl bg-white flex
        flex-col space-y-2 justify-between"
        in:fly|global={{ y: 50, duration: 100 }} out:fly|global={{ y: -50, duration: 100 }}
    >
        <div class="flex flex-row justify-between items-center scrollbar-hide pt-2">
            <span class="text-xl font-bold">Detail Transaction #{data.id}</span>
            <button class="text-md text-blue-900 font-bold" onclick={() => data.showDetailTransaction = false}>Close</button>
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
                        Other
                    </span>
                </div>
            </div>
            <div class="flex flex-col p-2">
                <span class="text-lg font-bold">Description</span>
                <div class="min-h-20">
                    {data.description ? data.description : '-'}
                </div>
            </div>
        </div>
    </div>  
</div>
