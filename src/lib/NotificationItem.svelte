<script lang="ts">
    import { formatDistanceToNow } from 'date-fns';
    import Fa from 'svelte-fa';
    import { faCircleExclamation, faArrowUp, faArrowDown } from '@fortawesome/free-solid-svg-icons';
    import Database from "@tauri-apps/plugin-sql";
    import DetailTransaction from './DetailTransaction.svelte';
    import Currency from './Currency.svelte';
    import { getDate, getMonth, getYear, getHours, getMinutes } from 'date-fns';

    const months = [
        'January',
        'February',
        'March',
        'April',
        'May',
        'June',
        'July',
        'August',
        'September',
        'October',
        'November',
        'December'
    ];

    let {
        edit = false,
        id = 69,
        amount = 69,
        type = 'in',
        method = 'bank',
        date = 1733602382348,
        description = 'Description goes here',
        categoryId = 0
    } = $props();

    let dateObj = $state(new Date(date));
    let category = $state();

    let data = $state({
        showDetailTransaction: false,
        id,
        amount,
        type,
        method,
        date,
        description,
        categoryId
    });

    $effect(() => {
        Database.load("sqlite:database.db").then((db: Database) => {
            db.select(
                "SELECT name FROM categories WHERE id=?",
                [categoryId]
            ).then((categories: any) => {
                category = categories.length !== 0
                    ? categories[0].name
                    : 'Other';
            });
        });

        data = {
            showDetailTransaction: false,
            id,
            amount,
            type,
            method,
            date,
            description,
            categoryId
        };
    });
</script>

{#if data.showDetailTransaction}
    <DetailTransaction
        { data }
    />

{/if}

<div class="flex flex-col justify-center items-center space-y-4">
    <span class="pt-2 font-semibold text-md text-gray-600">
        {getDate(data.date)} {months[getMonth(data.date)]} {getYear(data.date)} at {getHours(data.date)}:{getMinutes(data.date)}
    </span>
    <button
        onclick={() => {
            if (!edit) {data.showDetailTransaction = true}}
        }
        class="flex flex-row justify-between items-center bg-white shadow-xl shadow-blue-100
            px-4 py-4 rounded-xl w-full text-left"
    >
        <div class="flex flex-row">
            <div class="flex flex-row items-center 
                p-0 rounded-full text-red-500
            ">
                <Fa icon={faCircleExclamation} size="2.0x"/>
            </div>
            <div class="flex flex-col pl-2 justify-center items-center">
                <span class="font-bold text-lg capitalize">Large transaction!</span>
            </div>
        </div>
        <span class="text-lg">
            {#if type === 'out'}
                -
            {:else}
                +
            {/if}
            <Currency amount={amount} bold={true} subUnit={false} />
        </span>
    </button>
</div>

<style>
    :global(html) {
        background-color: rgb(239 246 255);
    }
</style>
